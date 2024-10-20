use crate::parser::{
    expressions::Expression,
    statements::{Block, ElseStatement, Statement},
    types::Type,
};

use super::{expect_type, expressions::check_expr, types::check_type, Scope, TypeError};

pub fn check_block(block: &Block, scope: &mut Scope) -> Result<Type, TypeError> {
    let mut ret_type = None;
    for statement in block {
        let stmt_ret = check_statement(statement, scope)?;
        if let Some(stmt_ret) = stmt_ret {
            if let Some(ret_type) = ret_type {
                if ret_type != stmt_ret {
                    return Err(TypeError::Unexpected {
                        got: stmt_ret,
                        expected: ret_type,
                    });
                }
            }
            ret_type = Some(stmt_ret);
        }
    }
    Ok(ret_type.unwrap_or(Type::Void))
}

pub fn check_statement(
    statement: &Statement,
    scope: &mut Scope,
) -> Result<Option<Type>, TypeError> {
    match statement {
        Statement::Import {
            path: _,
            imports: _,
        } => {
            todo!()
        }

        Statement::VarDef { name, typ, expr } => {
            let typ = typ.as_ref().map(|typ| check_type(typ, scope)).transpose()?;
            let expr_typ = check_expr(&expr, scope)?;
            if let Some(ref typ) = typ {
                if !is_assignable(&expr_typ, typ, scope) {
                    return Err(TypeError::Unexpected {
                        got: expr_typ,
                        expected: typ.clone(),
                    });
                }
            }
            scope.set_var(&name, typ.unwrap_or(expr_typ));
            Ok(None)
        }

        Statement::TypeDef { name, typ } => {
            scope.set_type(&name, typ.clone());
            let typ = check_type(&typ, scope)?;
            scope.set_type(&name, typ);

            Ok(None)
        }

        Statement::Expr(expr) => {
            check_expr(&expr, scope)?;
            Ok(None)
        }

        Statement::Continue => Ok(None),
        Statement::Break => Ok(None),
        Statement::Return(expr) => {
            if let Some(expr) = expr {
                let ret_type = check_expr(&expr, scope)?;
                return Ok(Some(ret_type));
            }
            Ok(None)
        }

        Statement::Assign { lhs, rhs } => {
            let lhs_typ = check_expr(&lhs, scope)?;
            let rhs_typ = check_expr(&rhs, scope)?;
            if !is_assignable(&rhs_typ, &lhs_typ, scope) {
                return Err(TypeError::Unexpected {
                    got: rhs_typ,
                    expected: lhs_typ,
                });
            }
            if !can_assign_to_expr(&lhs) {
                return Err(TypeError::Invalid(lhs_typ));
            }
            Ok(None)
        }

        Statement::If {
            body,
            cond,
            else_stmt,
        } => {
            let cond_typ = check_expr(&cond, scope)?;
            expect_type(cond_typ, Type::Bool)?;
            check_block(body, scope)?;

            match else_stmt {
                ElseStatement::Block(block) => {
                    check_block(block, scope)?;
                }
                ElseStatement::If(stmt) => {
                    check_statement(stmt, scope)?;
                }
                ElseStatement::None => {}
            }

            Ok(None)
        }

        Statement::Loop(body) => {
            check_block(body, scope)?;
            Ok(None)
        }

        Statement::While { cond, body } => {
            let cond_typ = check_expr(&cond, scope)?;
            expect_type(cond_typ, Type::Bool)?;
            check_block(body, scope)?;
            Ok(None)
        }
    }
}

fn can_assign_to_expr(expr: &Expression) -> bool {
    match expr {
        Expression::Identifier(_) => true,
        Expression::Deref(_) => true,
        Expression::Index { expr: _, index: _ } => true,
        Expression::Dot { expr: _, field: _ } => true,
        _ => false,
    }
}

fn is_assignable(src: &Type, dst: &Type, scope: &Scope) -> bool {
    if src == dst {
        return true;
    }

    match (src, dst) {
        (Type::Ptr(src), Type::Ptr(dst)) => {
            if is_assignable(src, dst, scope) {
                return true;
            }
            match (src.as_ref(), dst.as_ref()) {
                (Type::Void, _) => true,
                (_, Type::Void) => true,
                _ => false,
            }
        }
        // (Type::Int, Type::Char) => true,
        // (Type::Char, Type::Int) => true,
        (
            Type::SizedArray {
                element: src,
                len: _,
            },
            Type::Array(dst),
        ) => is_assignable(src, dst, scope),
        (
            Type::SizedArray {
                element: src,
                len: src_len,
            },
            Type::SizedArray {
                element: dst,
                len: dst_len,
            },
        ) => is_assignable(src, dst, scope) && src_len == dst_len,
        (Type::Array(src), Type::Array(dst)) => is_assignable(src, dst, scope),
        (Type::Struct(src), Type::Struct(dst)) => {
            for (dst_name, dst_ty) in dst {
                match src.get(dst_name) {
                    Some(src_ty) => {
                        if !is_assignable(src_ty, dst_ty, scope) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            true
        }
        (Type::Tuple(src), Type::Tuple(dst)) => {
            if src.len() != dst.len() {
                return false;
            }
            for (src, dst) in src.iter().zip(dst.iter()) {
                if !is_assignable(src, dst, scope) {
                    return false;
                }
            }
            true
        }
        (
            Type::Function {
                args: src_args,
                ret: src_ret,
            },
            Type::Function {
                args: dst_args,
                ret: dst_ret,
            },
        ) => {
            if src_args.len() != dst_args.len() {
                return false;
            }
            for (src, dst) in src_args.iter().zip(dst_args.iter()) {
                if !is_assignable(src, dst, scope) {
                    return false;
                }
            }
            is_assignable(src_ret, dst_ret, scope)
        }
        (Type::Named(src), dst) => match scope.get_type(src) {
            Some(src) => is_assignable(&src, dst, scope),
            None => false,
        },
        (src, Type::Named(dst)) => match scope.get_type(dst) {
            Some(dst) => is_assignable(src, &dst, scope),
            None => false,
        },
        _ => false,
    }
}
