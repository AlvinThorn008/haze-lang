// use crate::ast2::{Expr, FnDef, Node, NodeBuilder, NodeKind, NodeType::*, Stmt, TopDeclList, VarDecl};
// use crate::errors::Loc;
// use crate::errors::{ParsingError::{*, self}, ParseErrorKind::*};
// use crate::token::Tag;
// pub use crate::parser3::Parser;
// use crate::bumping::Box;
// use crate::typecheck::Type;
// use core::fmt::Write;

// use crate::{ast2::AstToken, visitor::{Visitor, Walker}};

// struct Generator<'s, 'b> {
//     ast: TopDeclList<'s, 'b>,
//     output: String
// }

// impl<'s, 'b> Generator<'s, 'b> {
//     pub fn generate(&mut self) {
//         for decl in self.ast.items() {
//             match decl {
//                 crate::ast2::TopLevelDecl::Mod(_) => unimplemented!(),
//                 crate::ast2::TopLevelDecl::Import(_) => unimplemented!(),
//                 crate::ast2::TopLevelDecl::Enum(_) => unimplemented!(),
//                 crate::ast2::TopLevelDecl::Fn(fn_def) => self.generate_func(fn_def),
//                 crate::ast2::TopLevelDecl::Struct(_) => todo!(),
//                 crate::ast2::TopLevelDecl::Type(_) => todo!(),
//                 crate::ast2::TopLevelDecl::Const(_) => todo!()
//             }
//         }
//     }

//     pub fn generate_func(&mut self, fn_def: FnDef<'s, 'b>) {
//         let return_type = fn_def.return_type().map(|ret| ret.token().value).unwrap_or("void");

//         let name = fn_def.name().token().value;

//         write!(self.output, "{} {}(", return_type, name);
//         fn_def.params().items().map(|param| {
//             let param_type = Type::from_type_expr(&param.param_type());
//             // write!(self.output, "{} {}", param.param_type().)
//         });
//         for stmt in fn_def.body().body().items() {
            
//         }
//     }

//     fn generate_statement(&mut self, stmt: Stmt<'s, 'b>) {
//         match stmt {
//             Stmt::VarDecl(node) => self.generate_var_decl(node),
//             Stmt::ExprStmt(_) => todo!(),
//         }
//     }

//     fn generate_var_decl(&mut self, var: VarDecl<'s, 'b>) {
//         write!(self.output, "{} {} = ", 
//             var.var_type().unwrap().token().value,
//             var.name().token().value
//         )?;
//         self.generate_expr(var.value().unwrap());
//         self.output.push(';');
//     }

//     fn generate_expr(&mut self, expr: Expr<'s, 'b>) {

//     }
// }