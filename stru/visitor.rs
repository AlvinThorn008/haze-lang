use crate::ast;
use crate::ast::{
    self, tag_is_binop, AstNode, AstToken, Node, NodeBuilder, NodeChild, NodeKind, NodeType::*,
    TopLevelDecl,
};
use crate::errors::Loc;
use crate::errors::{
    ParseErrorKind::*,
    ParsingError::{self, *},
};
use crate::parser3::Parser;
use crate::token::{self, Tag};

pub trait Visitor<'s, 'b> {
    fn visit_program(&mut self, node: &'b Node<'s, 'b>) {
        self.visit_top_level_declarations(ast::TopDeclList::cast(node));
    }

    fn visit_top_level_declarations(&mut self, node: ast::TopDeclList<'s, 'b>) {
        for decl in node.items() {
            match decl {
                TopLevelDecl::Fn(node) => self.visit_function_defintion(node),
                TopLevelDecl::Const(node) => self.visit_const_declaration(node),
                TopLevelDecl::Mod(node) => self.visit_module(node),
                TopLevelDecl::Import(node) => self.visit_import_declaration(node),
                TopLevelDecl::Enum(node) => self.visit_enum_declaration(node),
                TopLevelDecl::Struct(node) => self.visit_struct_declaration(node),
                TopLevelDecl::Type(node) => self.visit_type_alias(node),
            }
        }
    }

    fn visit_module(&mut self, node: ast::Module<'s, 'b>);
    fn visit_import_declaration(&mut self, node: ast::ImportDecl<'s, 'b>);
    fn visit_enum_declaration(&mut self, node: ast::EnumDecl<'s, 'b>);
    fn visit_function_defintion(&mut self, node: ast::FnDef<'s, 'b>);
    fn visit_struct_declaration(&mut self, node: ast::StructDecl<'s, 'b>);
    fn visit_type_alias(&mut self, node: ast::TypeAlias<'s, 'b>);
    fn visit_const_declaration(&mut self, node: ast::ConstDecl<'s, 'b>);

    fn visit_statement(&mut self, node: ast::Stmt<'s, 'b>) {
        match node {
            ast::Stmt::ExprStmt(stmt) => self.visit_expression_statement(stmt),
            ast::Stmt::VarDecl(decl) => self.visit_variable_declaration(decl),
        }
    }

    fn visit_variable_declaration(&mut self, node: ast::VarDecl<'s, 'b>) {}
    fn visit_expression_statement(&mut self, node: ast::ExprStmt<'s, 'b>) {}

    fn visit_expression(&mut self, node: ast::Expr<'s, 'b>) {
        match node {
            ast::Expr::Infix(expr) => self.visit_infix_expression(expr),
            ast::Expr::Prefix(expr) => self.visit_prefix_expression(expr),
            ast::Expr::Group(expr) => self.visit_group_expression(expr),
            ast::Expr::CallExpr(expr) => self.visit_call_expression(expr),
            ast::Expr::Ident(expr) => self.visit_ident(expr),
            ast::Expr::Str(expr) => self.visit_string(expr),
            ast::Expr::Int(expr) => self.visit_number(expr),
            _ => todo!(),
        }
    }

    fn visit_ident(&mut self, node: ast::Ident<'s, 'b>) {}
    fn visit_string(&mut self, node: ast::Str<'s, 'b>) {}
    fn visit_number(&mut self, node: ast::Int<'s, 'b>) {}
    fn visit_bool(&mut self, node: ast::Bool<'s, 'b>) {}

    fn visit_infix_expression(&mut self, node: ast::Infix<'s, 'b>) {
        self.visit_expression(node.left());
        self.visit_expression(node.right());
    }

    fn visit_prefix_expression(&mut self, node: ast::Prefix<'s, 'b>) {

        self.visit_expression(node.right());
    }

    fn visit_group_expression(&mut self, node: ast::Group<'s, 'b>) {
        self.visit_expression(node.expr());
    }

    fn visit_call_expression(&mut self, node: ast::CallExpr<'s, 'b>);

    fn visit_block_expression(&mut self, node: ast::BlockExpr<'s, 'b>) {
        for stmt in node.body().items() {
            self.visit_statement(stmt);
        }
    }
    
    fn visit_if_expression(&mut self, node: ast::IfExpr<'s, 'b>) {}
    fn visit_while_expression(&mut self, node: ast::WhileExpr<'s, 'b>) {}
    fn visit_return_expression(&mut self, node: ast::ReturnExpr<'s, 'b>) {}
    fn visit_continue_expression(&mut self, node: ast::ContinueExpr<'s, 'b>) {}
    fn visit_assign_expression(&mut self, node: ast::AssignExpr<'s, 'b>) {}
    fn visit_break_expression(&mut self, node: ast::BreakExpr<'s, 'b>) {}
    
    fn visit_array_expression(&mut self, node: ast::ArrayExpr<'s, 'b>) {}
    fn visit_index_expression(&mut self, node: ast::IndexExpr<'s, 'b>) {}
    fn visit_field_access_expression(&mut self, node: ast::FieldAccessExpr<'s, 'b>) {}

    fn visit_statement_list(&mut self, node: ast::StmtList<'s, 'b>) {}
    
    fn visit_variant_list(&mut self, node: ast::VariantList<'s, 'b>) {}
    fn visit_variant(&mut self, node: ast::Variant<'s, 'b>) {}
    
    fn visit_field_list(&mut self, node: ast::FieldList<'s, 'b>) {}
    fn visit_field(&mut self, node: ast::Field<'s, 'b>) {}
    fn visit_tuple_expression(&mut self, node: ast::TupleExpr<'s, 'b>) {}
    fn visit_array_type(&mut self, node: ast::ArrayType<'s, 'b>) {}
    fn visit_tuple_type(&mut self, node: ast::TupleType<'s, 'b>) {}
    fn visit_group_type(&mut self, node: ast::GroupType<'s, 'b>) {}
    fn visit_fn_type(&mut self, node: ast::FnType<'s, 'b>) {}
    fn visit_fn_type_param_list(&mut self, node: ast::FnTypeParamList<'s, 'b>) {}
    fn visit_param_list(&mut self, node: ast::ParamList<'s, 'b>) {}
    fn visit_param(&mut self, node: ast::Param<'s, 'b>) {}
    fn visit_method_call(&mut self, node: ast::MethodCall<'s, 'b>) {}
    fn visit_arg_list(&mut self, node: ast::ArgList<'s, 'b>) {}
    
}

struct Walker<'s, 'b, T> {
    tree: &'b Node<'s, 'b>,
    visitor: T
}

enum NodePoint<'s, 'b> { Node(&'b Node<'s, 'b>), Token(&'b token::Token<'s>) }

impl<'s, 'b, T: Visitor<'s, 'b>> Walker<'s, 'b, T> {
    pub fn walk(&mut self) {
        let mut stack = Vec::new();
        stack.push(NodePoint::Node(self.tree));

        while let Some(point) = stack.pop() {
            match point {
                NodePoint::Node(node) => {
                    match node.kind.0 {
                        TopDeclList => self.visitor.visit_top_level_declarations(ast::TopDeclList::cast(node)),
                        VarDecl => self.visitor.visit_variable_declaration(ast::VarDecl::cast(node)),
                        FnDef => self.visitor.visit_function_defintion(ast::FnDef::cast(node)),
                        BlockStmt => unreachable!(),
                        ExprStmt => self.visitor.visit_expression_statement(ast::ExprStmt::cast(node)),
                        EmptyStmt => unimplemented!(),
            
                        Infix => self.visitor.visit_infix_expression(ast::Infix::cast(node)),
                        Prefix => self.visitor.visit_prefix_expression(ast::Prefix::cast(node)),
                        Group => self.visitor.visit_group_expression(ast::Group::cast(node)),
                        BlockExpr => self.visitor.visit_block_expression(ast::BlockExpr::cast(node)),
                        IfExpr => self.visitor.visit_if_expression(ast::IfExpr::cast(node)),
                        WhileExpr => self.visitor.visit_while_expression(ast::WhileExpr::cast(node)),
                        ReturnExpr => self.visitor.visit_return_expression(ast::ReturnExpr::cast(node)),
                        ContinueExpr => self.visitor.visit_continue_expression(ast::ContinueExpr::cast(node)),
                        AssignExpr => self.visitor.visit_assign_expression(ast::AssignExpr::cast(node)),
                        CallExpr => self.visitor.visit_call_expression(ast::CallExpr::cast(node)),
                        ArrayExpr => self.visitor.visit_array_expression(ast::ArrayExpr::cast(node)),
                        BreakExpr => self.visitor.visit_break_expression(ast::BreakExpr::cast(node)),
                        StmtList => self.visitor.visit_statement_list(ast::StmtList::cast(node)),
                        TopDeclList => self.visitor.visit_top_level_declarations(ast::TopDeclList::cast(node)),
                        Module => self.visitor.visit_module(ast::Module::cast(node)),
                        ImportDecl => self.visitor.visit_import_declaration(ast::ImportDecl::cast(node)),
                        EnumDecl => self.visitor.visit_enum_declaration(ast::EnumDecl::cast(node)),
                        VariantList => self.visitor.visit_variant_list(ast::VariantList::cast(node)),
                        Variant => self.visitor.visit_variant(ast::Variant::cast(node)),
                        StructDecl => self.visitor.visit_struct_declaration(ast::StructDecl::cast(node)),
                        FieldList => self.visitor.visit_field_list(ast::FieldList::cast(node)),
                        Field => self.visitor.visit_field(ast::Field::cast(node)),
                        TypeAlias => self.visitor.visit_type_alias(ast::TypeAlias::cast(node)),
                        ConstDecl => self.visitor.visit_const_declaration(ast::ConstDecl::cast(node)),
                        TupleExpr => self.visitor.visit_tuple_expression(ast::TupleExpr::cast(node)),
                        TupleType => self.visitor.visit_tuple_type(ast::TupleType::cast(node)),
                        ArrayType => self.visitor.visit_array_type(ast::ArrayType::cast(node)),
                        GroupType => self.visitor.visit_group_type(ast::GroupType::cast(node)),
                        FnType => self.visitor.visit_fn_type(ast::FnType::cast(node)),
                        FnTypeParamList => self.visitor.visit_fn_type_param_list(ast::FnTypeParamList::cast(node)),
                        ParamList => self.visitor.visit_param_list(ast::ParamList::cast(node)),
                        Param => self.visitor.visit_param(ast::Param::cast(node)),
                        ArgList => self.visitor.visit_arg_list(ast::ArgList::cast(node)),
                        FieldAccessExpr => self.visitor.visit_field_access_expression(ast::FieldAccessExpr::cast(node)),
                        IndexExpr => self.visitor.visit_index_expression(ast::IndexExpr::cast(node)),
                        MethodCall => self.visitor.visit_method_call(ast::MethodCall::cast(node)),
                        Any => unreachable!("[DEV]: A valid syntax tree should not contain an Any node"),
                    }
                    // Valid syntax tree should not contain nodes without children
                    for child in node.children.as_ref().unwrap().0.iter().rev() {
                        let point = match child {
                            NodeChild::Node(node) => NodePoint::Node(node),
                            NodeChild::Token(token) => NodePoint::Token(token),
                        };
                        stack.push(point);
                    }
                }
                NodePoint::Token(token) => match token.tag {
                    Tag::Ident => self.visitor.visit_ident(ast::Ident::cast(token)),
                    Tag::Number => self.visitor.visit_number(ast::Int::cast(token)),
                    Tag::String => self.visitor.visit_string(ast::Str::cast(token)),
                    Tag::Bool => self.visitor.visit_bool(ast::Bool::cast(token)),
                    Tag::Invalid => unreachable!("[DEV]: Did I forget to check if any errors occurred after parsing?"),
                    _ => unreachable!("[DEV]: A valid syntax tree should not contain a token of this type"),
                }
            }
        }
    }
}
