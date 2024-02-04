// use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use hashbrown::HashMap;

use crate::{ast2::{self, AstToken, Ident, TypeExpr}, visitor::{Visitor, Walker}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Bool,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,

    String,
    Array(Box<Type>, u8),

    Unit,
    Never, 

    Const(Box<Type>),
    Fn(Box<[Type]>, Box<Type>),

    Struct(Box<[(Box<str>, Type)]>),
    TypeAlias(Box<Type>),

    Unresolved(Box<str>)
}

#[derive(Debug, Default)]
struct Env<'s> {
    global_scope: HashMap<&'s str, Type>,
    local_scopes: Vec<IndexMap<&'s str, Type>>,
    // unresolved: Vec<&>
}

impl<'s, 'b> Env<'s> {
    fn new() -> Self { 
        Self::default()
    }

    fn register(&mut self, key: &'b ast2::Ident<'s, 'b>, typ: Type) { self.global_scope.insert(key.token().value, typ); }

    fn register_function(&mut self, node: ast2::FnDef<'s, 'b>) {
        let param_types: Box<[Type]> = node.params()
            .items()
            .map(|param| (&param.param_type()).into())
            .collect();
        let mut return_type = Box::new(node.return_type().map(|ret_type| (&ret_type).into()).unwrap_or(Type::Unit));

        self.register(&node.name(), Type::Fn(param_types, return_type));
    }

    fn register_struct(&mut self, node: ast2::StructDecl<'s, 'b>) {
        let fields: Box<[(Box<str>, Type)]> = node.fields()
            .items()
            .map(|field| (field.name().token().value.into(), (&field.field_type()).into()))
            .collect();
        self.register(&node.name(), Type::Struct(fields));
    }

    fn register_const(&mut self, node: ast2::ConstDecl<'s, 'b>) {
        let typ = (&node.const_type()).into();
        self.register(&node.name(), Type::Const(Box::new(typ)));
    }

    fn register_type_alias(&mut self, node: ast2::TypeAlias<'s, 'b>) {
        let typ = (&node.type_expr()).into();
        self.register(&node.name(), Type::TypeAlias(Box::new(typ)));
    }

    fn resolve_globals(&mut self) {

    }
}



impl Type {
    // pub fn as_c_literal(&self) -> String {
    //     match self {
    //         Bool => "bool".into(),
    //         U32 => "uint32_t".into(),
    //         F32 => "float".into(),
    //         F64 => "double".into(),
    //         String => 
    //     }
    // }

    pub fn from_type_expr<'a, 's, 'b>(expr: &'a TypeExpr<'s, 'b>) -> Type {
        match expr {
            TypeExpr::Ident(ident) => {
                match ident.token().value {
                    "bool" => Type::Bool,
                    "u32" => Type::U32,
                    "u64" => Type::U64,
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    "string" => Type::String,
                    _ => Type::Unresolved(ident.token().value.into())
                }
            },
            TypeExpr::ArrayType(array_type) => {
                let elem = array_type.element_type();
                let elem_type = Type::from_type_expr(&elem);
                Type::Array(Box::new(elem_type), array_type.len().token().value.parse().unwrap())
            }
            TypeExpr::GroupType(group_type) => {
                Type::from_type_expr(&group_type.inner_type())
            }
            _ => todo!()
        }
    }
}

impl<'a, 's, 'b> From<&'a TypeExpr<'s, 'b>> for Type {
    fn from(value: &'a TypeExpr<'s, 'b>) -> Self {
        match value {
            TypeExpr::Ident(ident) => {
                match ident.token().value {
                    "bool" => Type::Bool,
                    "u32" => Type::U32,
                    "u64" => Type::U64,
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    "string" => Type::String,
                    _ => Type::Unresolved(ident.token().value.into())
                }
            },
            TypeExpr::ArrayType(array_type) => {
                let elem = array_type.element_type();
                let elem_type = (&elem).into();
                Type::Array(Box::new(elem_type), array_type.len().token().value.parse().unwrap())
            }
            TypeExpr::GroupType(group_type) => {
                (&group_type.inner_type()).into()
            }
            _ => todo!()
        }
    }
}

// pub struct Resolver<'a> {
//     global: HashMap<&'a str, Type>,
//     local_scopes: Vec<HashMap<&'a str, Type>>,
//     unresolved_types: HashSet<Box<str>>
// }

// use crate::ast2::*;

// impl<'a> Resolver<'a> {
//     fn mark(&mut self, typ: Type) -> Type {
//         match &typ {
//             Type::Unresolved(sym) => { self.unresolved_types.insert(sym.clone()); },
//             _ => {}
//         };
//         typ
//     } 

//     pub fn first_pass<'b>(&mut self, decls: crate::ast2::TopDeclList<'a, 'b>) {
//         for decl in decls.items() {
//             match decl {
//                 TopLevelDecl::Fn(fn_def) => {
//                     let symbol_name = fn_def.name().token().value;

//                     let params: Box<[Type]> = fn_def.params().items().map(|param| {
//                         let param_type = self.mark(Type::from_type_expr(&param.param_type()));
//                         param_type
//                     }).collect();

//                     let ret_type = self.mark(fn_def.return_type().map(|ret_type| Type::from_type_expr(&ret_type)).unwrap_or(Type::Unit));

//                     self.global.insert(symbol_name, Type::Fn(params, Box::new(ret_type)));
//                 }
//                 TopLevelDecl::Const(const_def) => {
//                     let symbol_name = const_def.name().token().value;

//                     let const_type = self.mark(Type::from_type_expr(&const_def.const_type()));

//                     self.global.insert(symbol_name, const_type);
//                 }
//                 TopLevelDecl::Struct(struct_def) => {
//                     let symbol_name = struct_def.name().token().value;

//                     let fields = struct_def.fields().items().map(|field| {
//                         let field_typ = Type::from_type_expr(&field.field_type());
//                         (field.name().token().value.to_string().into_boxed_str(), field_typ)
//                     });

//                     let mut field_type_list = Vec::new();

//                     for field in fields {
//                         field_type_list.push(field);
//                     }
//                     let field_type_list = self.mark(Type::Struct(field_type_list.into_boxed_slice()));
//                     self.global.insert(symbol_name, field_type_list);
//                 }
//                 _ => todo!()
//             }
//         }
//     }

//     pub fn second_pass<'b>(&mut self, decls: crate::ast2::TopDeclList<'a, 'b>) {
//         // for decl in decls.items() {
//         //     match decl {}
//         // }
//     }
// }

// impl<'s, 'b> Visitor<'s, 'b> for Resolver<'s> {
//     // fn visit_function_defintion(&mut self, node: crate::ast2::FnDef<'s, 'b>) {
//     //     let param_types = node.params().items().map(|param| {
//     //         param.param_type()
//     //     });
//     //     self.global.insert(node.name().token().value, Type::Fn());
//     // }
// }

