use std::collections::HashMap;

use crate::{
    parser::{expressions::Expression, types::Type},
    type_checker::{statements::is_assignable, types::check_type},
};

use super::{statements::check_block, Scope, TypeError};

pub fn check_expr(expr: &Expression, scope: &Scope) -> Result<Type, TypeError> {
    match expr {
        Expression::Null => Ok(Type::Ptr(Box::new(Type::Void))),

        Expression::Bool(_) => Ok(Type::Bool),
        Expression::Int(_) => Ok(Type::Int),
        Expression::Float(_) => Ok(Type::Float),
        Expression::String(_) => Ok(Type::String),
        Expression::Char(_) => Ok(Type::Char),

        Expression::Add(lhs, rhs) => check_binop_expr(lhs, rhs, scope),
        Expression::Sub(lhs, rhs) => check_binop_expr(lhs, rhs, scope),
        Expression::Mul(lhs, rhs) => check_binop_expr(lhs, rhs, scope),
        Expression::Div(lhs, rhs) => check_binop_expr(lhs, rhs, scope),
        Expression::Mod(lhs, rhs) => match (check_expr(lhs, scope)?, check_expr(rhs, scope)?) {
            (Type::Int, Type::Int) => Ok(Type::Int),
            (lhs, _) => Err(TypeError::Unexpected {
                got: lhs,
                expected: Type::Int,
            }),
        },

        Expression::Neg(expr) => match check_expr(expr, scope)? {
            Type::Int => Ok(Type::Int),
            Type::Float => Ok(Type::Float),
            ty => Err(TypeError::Invalid(ty)),
        },

        Expression::Equal(lhs, rhs) => check_same_type(lhs, rhs, scope),

        Expression::GreaterEqual(lhs, rhs) => check_binop_cmp_expr(lhs, rhs, scope),
        Expression::GreaterThan(lhs, rhs) => check_binop_cmp_expr(lhs, rhs, scope),
        Expression::LessEqual(lhs, rhs) => check_binop_cmp_expr(lhs, rhs, scope),
        Expression::LessThan(lhs, rhs) => check_binop_cmp_expr(lhs, rhs, scope),

        Expression::And(lhs, rhs) => check_bool_op(lhs, rhs, scope),
        Expression::Or(lhs, rhs) => check_bool_op(lhs, rhs, scope),
        Expression::Not(expr) => match check_expr(expr, scope)? {
            Type::Bool => Ok(Type::Bool),
            ty => Err(TypeError::Invalid(ty)),
        },

        Expression::StructLiteral(fields) => {
            let mut struct_fields = HashMap::new();
            for (name, expr) in fields {
                struct_fields.insert(name.clone(), check_expr(expr, scope)?);
            }
            Ok(Type::Struct(struct_fields))
        }
        Expression::ArrayLiteral(exprs) => {
            let mut array_type = None;
            for expr in exprs {
                let ty = check_expr(expr, scope)?;
                if let Some(array_type) = &array_type {
                    if &ty != array_type {
                        return Err(TypeError::Unexpected {
                            got: ty,
                            expected: array_type.clone(),
                        });
                    }
                } else {
                    array_type = Some(ty);
                }
            }
            Ok(Type::SizedArray {
                element: Box::new(array_type.unwrap()),
                len: exprs.len() as i64,
            })
        }
        Expression::TupleLiteral(exprs) => {
            let mut tuple_type = Vec::new();
            for expr in exprs {
                tuple_type.push(Box::new(check_expr(expr, scope)?));
            }
            Ok(Type::Tuple(tuple_type))
        }
        Expression::FunctionLiteral { args, ret, body } => {
            let mut arg_types = Vec::new();
            for (_, ty) in args {
                let ty = check_type(ty, scope)?;
                arg_types.push(Box::new(ty));
            }
            let ret = Box::new(check_type(ret, scope)?);

            let mut scope = scope.create_child();

            // Insert the arguments into the scope
            for (name, ty) in args {
                scope.set_var(name, ty.clone());
            }

            let ret_type = check_block(body, &mut scope)?;

            if ret_type != *ret {
                return Err(TypeError::Unexpected {
                    got: ret_type,
                    expected: *ret,
                });
            }

            dbg!("FunctionScope:", scope);

            Ok(Type::Function {
                args: arg_types,
                ret,
            })
        }

        Expression::Index { expr, index } => {
            let index = check_expr(index, scope)?;
            if index != Type::Int {
                return Err(TypeError::Unexpected {
                    got: index,
                    expected: Type::Int,
                });
            }

            match check_expr(expr, scope)? {
                Type::Array(element) => Ok(*element),
                Type::SizedArray { element, .. } => Ok(*element),
                ty => Err(TypeError::Invalid(ty)),
            }
        }

        Expression::Ref(expr) => Ok(Type::Ptr(Box::new(check_expr(expr, scope)?))),

        Expression::Deref(expr) => match check_expr(expr, scope)? {
            Type::Ptr(ty) => Ok(check_type(&ty, scope)?),
            ty => Err(TypeError::Invalid(ty)),
        },

        Expression::Identifier(name) => match scope.get_var(name) {
            Some(ty) => Ok(check_type(&ty, scope)?),
            None => Err(TypeError::InvalidIdentifier(name.clone())),
        },

        Expression::Dot { expr, field } => {
            let typ = check_expr(expr, scope)?;
            match typ {
                Type::Struct(fields) => match fields.get(field) {
                    Some(ty) => Ok(check_type(ty, scope)?),
                    None => Err(TypeError::InvalidIdentifier(field.clone())),
                },
                ty => Err(TypeError::Invalid(ty)),
            }
        }

        Expression::Call { expr, args } => {
            let typ = check_expr(expr, scope)?;
            let typ_for_errors = typ.clone();
            println!("Call: {:?}", typ);
            match typ {
                Type::Function {
                    args: arg_types,
                    ret,
                } => {
                    if args.len() != arg_types.len() {
                        return Err(TypeError::Invalid(typ_for_errors));
                    }

                    for (arg, ty) in args.iter().zip(arg_types.iter()) {
                        let arg_type = check_expr(arg, scope)?;
                        if !is_assignable(&arg_type, &ty, scope) {
                            return Err(TypeError::Unexpected {
                                got: arg_type,
                                expected: *ty.clone(),
                            });
                        }
                    }

                    Ok(*ret)
                }
                ty => Err(TypeError::Invalid(ty)),
            }
        }
    }
}

pub fn infer_function_type_signature(expr: &Expression, scope: &Scope) -> Result<Type, TypeError> {
    match expr {
        Expression::FunctionLiteral { args, ret, body: _ } => {
            let mut arg_types = Vec::new();
            for (_, ty) in args {
                let ty = check_type(ty, scope)?;
                arg_types.push(Box::new(ty));
            }
            let ret = Box::new(check_type(ret, scope)?);

            Ok(Type::Function {
                args: arg_types,
                ret,
            })
        }
        _ => Err(TypeError::Unexpected {
            got: Type::Void,
            expected: Type::Function {
                args: vec![],
                ret: Box::new(Type::Void),
            },
        }),
    }
}

pub fn check_binop_expr(
    lhs: &Expression,
    rhs: &Expression,
    scope: &Scope,
) -> Result<Type, TypeError> {
    let lhs = check_expr(lhs, scope)?;
    let rhs = check_expr(rhs, scope)?;

    match (lhs, rhs) {
        (Type::Int, Type::Int) => Ok(Type::Int),
        (Type::Float, Type::Float) => Ok(Type::Float),
        (Type::Char, Type::Char) => Ok(Type::Char),
        (lhs, rhs) if lhs == rhs => Err(TypeError::Invalid(lhs)),
        (lhs, rhs) => Err(TypeError::Unexpected {
            got: lhs,
            expected: rhs,
        }),
    }
}

pub fn check_binop_cmp_expr(
    lhs: &Expression,
    rhs: &Expression,
    scope: &Scope,
) -> Result<Type, TypeError> {
    check_binop_expr(lhs, rhs, scope)?;
    Ok(Type::Bool)
}

fn check_bool_op(lhs: &Expression, rhs: &Expression, scope: &Scope) -> Result<Type, TypeError> {
    let lhs = check_expr(lhs, scope)?;
    let rhs = check_expr(rhs, scope)?;

    match (lhs, rhs) {
        (Type::Bool, Type::Bool) => Ok(Type::Bool),
        (lhs, rhs) => Err(TypeError::Unexpected {
            got: lhs,
            expected: rhs,
        }),
    }
}

fn check_same_type(lhs: &Expression, rhs: &Expression, scope: &Scope) -> Result<Type, TypeError> {
    let lhs = check_expr(lhs, scope)?;
    let rhs = check_expr(rhs, scope)?;
    if lhs == rhs {
        Ok(Type::Bool)
    } else {
        Err(TypeError::Unexpected {
            got: lhs,
            expected: rhs,
        })
    }
}
