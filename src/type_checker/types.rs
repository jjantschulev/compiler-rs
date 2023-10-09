use std::collections::HashMap;

use crate::parser::types::Type;

use super::{Scope, TypeError};

pub fn check_type(ty: &Type, scope: &Scope) -> Result<Type, TypeError> {
    match ty {
        Type::Int => Ok(Type::Int),
        Type::Float => Ok(Type::Float),
        Type::Char => Ok(Type::Char),
        Type::Bool => Ok(Type::Bool),
        Type::String => Ok(Type::String),
        Type::Void => Ok(Type::Void),
        Type::Ptr(ty) => Ok(Type::Ptr(Box::new(check_type(ty, scope)?))),
        Type::SizedArray { element, len } => Ok(Type::SizedArray {
            element: Box::new(check_type(element, scope)?),
            len: *len,
        }),
        Type::Struct(fields) => {
            let mut ret = HashMap::new();
            for (name, ty) in fields {
                ret.insert(name.clone(), check_type(ty, scope)?);
            }
            Ok(Type::Struct(ret))
        }
        Type::Array(ty) => Ok(Type::Array(Box::new(check_type(ty, scope)?))),
        Type::Tuple(tys) => {
            let mut ret = Vec::new();
            for ty in tys {
                ret.push(Box::new(check_type(ty, scope)?));
            }
            Ok(Type::Tuple(ret))
        }
        Type::Function { args, ret } => {
            let mut checked_args = Vec::new();
            for ty in args {
                checked_args.push(Box::new(check_type(ty, scope)?));
            }
            Ok(Type::Function {
                args: checked_args,
                ret: Box::new(check_type(ret, scope)?),
            })
        }
        Type::Named(ident) => match scope.get_type(ident) {
            Some(ty) => Ok(ty.clone()),
            None => Err(TypeError::InvalidIdentifier(ident.clone())),
        },
    }
}
