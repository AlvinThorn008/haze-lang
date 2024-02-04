use crate::ast;
use crate::ast2::{
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
        self.visit_top_level_declarations(ast2::TopDeclList::cast(node));
    }

    fn visit_top_level_declarations(&mut self, node: ast2::TopDeclList<'s, 'b>) {
        // for decl in node.items() {
        //     match decl {
        //         TopLevelDecl::Fn(node) => self.visit_function_defintion(node),
        //         TopLevelDecl::Const(node) => self.visit_const_declaration(node),
        //         TopLevelDecl::Mod(node) => self.visit_module(node),
        //         TopLevelDecl::Import(node) => self.visit_import_declaration(node),
        //         TopLevelDecl::Enum(node) => self.visit_enum_declaration(node),
        //         TopLevelDecl::Struct(node) => self.visit_struct_declaration(node),
        //         TopLevelDecl::Type(node) => self.visit_type_alias(node),
        //     }
        // }
    }

    fn visit_module(&mut self, node: ast2::Module<'s, 'b>) {}
    fn visit_import_declaration(&mut self, node: ast2::ImportDecl<'s, 'b>) {}
    fn visit_enum_declaration(&mut self, node: ast2::EnumDecl<'s, 'b>) {}
    fn visit_function_defintion(&mut self, node: ast2::FnDef<'s, 'b>) {}
    fn visit_struct_declaration(&mut self, node: ast2::StructDecl<'s, 'b>) {}
    fn visit_type_alias(&mut self, node: ast2::TypeAlias<'s, 'b>) {}
    fn visit_const_declaration(&mut self, node: ast2::ConstDecl<'s, 'b>) {}

    fn visit_statement(&mut self, node: ast2::Stmt<'s, 'b>) {
        // match node {
        //     ast2::Stmt::ExprStmt(stmt) => self.visit_expression_statement(stmt),
        //     ast2::Stmt::VarDecl(decl) => self.visit_variable_declaration(decl),
        // }
    }

    fn visit_variable_declaration(&mut self, node: ast2::VarDecl<'s, 'b>) {}
    fn visit_expression_statement(&mut self, node: ast2::ExprStmt<'s, 'b>) {}

    fn visit_expression(&mut self, node: ast2::Expr<'s, 'b>) {
        // match node {
        //     ast2::Expr::Infix(expr) => self.visit_infix_expression(expr),
        //     ast2::Expr::Prefix(expr) => self.visit_prefix_expression(expr),
        //     ast2::Expr::Group(expr) => self.visit_group_expression(expr),
        //     ast2::Expr::CallExpr(expr) => self.visit_call_expression(expr),
        //     ast2::Expr::Ident(expr) => self.visit_ident(expr),
        //     ast2::Expr::Str(expr) => self.visit_string(expr),
        //     ast2::Expr::Int(expr) => self.visit_number(expr),
        //     _ => todo!(),
        // }
    }

    fn visit_ident(&mut self, node: ast2::Ident<'s, 'b>) {}
    fn visit_string(&mut self, node: ast2::Str<'s, 'b>) {}
    fn visit_number(&mut self, node: ast2::Int<'s, 'b>) {}
    fn visit_bool(&mut self, node: ast2::Bool<'s, 'b>) {}

    fn visit_infix_expression(&mut self, node: ast2::Infix<'s, 'b>) {
        // self.visit_expression(node.left());
        // self.visit_expression(node.right());
    }

    fn visit_prefix_expression(&mut self, node: ast2::Prefix<'s, 'b>) {

        //self.visit_expression(node.right());
    }

    fn visit_group_expression(&mut self, node: ast2::Group<'s, 'b>) {
        // self.visit_expression(node.expr());
    }

    fn visit_call_expression(&mut self, node: ast2::CallExpr<'s, 'b>) {}

    fn visit_block_expression(&mut self, node: ast2::BlockExpr<'s, 'b>) {
        // for stmt in node.body().items() {
        //     self.visit_statement(stmt);
        // }
    }
    
    fn visit_if_expression(&mut self, node: ast2::IfExpr<'s, 'b>) {}
    fn visit_while_expression(&mut self, node: ast2::WhileExpr<'s, 'b>) {}
    fn visit_return_expression(&mut self, node: ast2::ReturnExpr<'s, 'b>) {}
    fn visit_continue_expression(&mut self, node: ast2::ContinueExpr<'s, 'b>) {}
    fn visit_assign_expression(&mut self, node: ast2::AssignExpr<'s, 'b>) {}
    fn visit_break_expression(&mut self, node: ast2::BreakExpr<'s, 'b>) {}
    
    fn visit_array_expression(&mut self, node: ast2::ArrayExpr<'s, 'b>) {}
    fn visit_index_expression(&mut self, node: ast2::IndexExpr<'s, 'b>) {}
    fn visit_field_access_expression(&mut self, node: ast2::FieldAccessExpr<'s, 'b>) {}

    fn visit_statement_list(&mut self, node: ast2::StmtList<'s, 'b>) {}
    
    fn visit_variant_list(&mut self, node: ast2::VariantList<'s, 'b>) {}
    fn visit_variant(&mut self, node: ast2::Variant<'s, 'b>) {}
    
    fn visit_field_list(&mut self, node: ast2::FieldList<'s, 'b>) {}
    fn visit_field(&mut self, node: ast2::Field<'s, 'b>) {}
    fn visit_tuple_expression(&mut self, node: ast2::TupleExpr<'s, 'b>) {}
    fn visit_array_type(&mut self, node: ast2::ArrayType<'s, 'b>) {}
    fn visit_tuple_type(&mut self, node: ast2::TupleType<'s, 'b>) {}
    fn visit_group_type(&mut self, node: ast2::GroupType<'s, 'b>) {}
    fn visit_fn_type(&mut self, node: ast2::FnType<'s, 'b>) {}
    fn visit_fn_type_param_list(&mut self, node: ast2::FnTypeParamList<'s, 'b>) {}
    fn visit_param_list(&mut self, node: ast2::ParamList<'s, 'b>) {}
    fn visit_param(&mut self, node: ast2::Param<'s, 'b>) {}
    fn visit_method_call(&mut self, node: ast2::MethodCall<'s, 'b>) {}
    fn visit_arg_list(&mut self, node: ast2::ArgList<'s, 'b>) {}
    
    fn visit_field_init(&mut self, node: ast2::FieldInit<'s, 'b>) {}
    fn visit_field_init_list(&mut self, node: ast2::FieldInitList<'s, 'b>) {}
    fn visit_struct_expression(&mut self, node: ast2::StructExpr<'s, 'b>) {}
}

pub struct Walker<'s, 'b, T> {
    tree: &'b Node<'s, 'b>,
    visitor: T
}

enum NodePoint<'s, 'b> { Node(&'b Node<'s, 'b>), Token(&'b token::Token<'s>) }

impl<'s, 'b, T: Visitor<'s, 'b>> Walker<'s, 'b, T> {
    pub fn new(tree: &'b Node<'s, 'b>, visitor: T) -> Self {
        Self {
            tree,
            visitor
        }
    }

    pub fn walk(&mut self) {
        let mut stack = Vec::new();
        stack.push(NodePoint::Node(self.tree));

        while let Some(point) = stack.pop() {
            match point {
                NodePoint::Node(node) => {
                    match node.kind.0 {
                        TopDeclList => self.visitor.visit_top_level_declarations(ast2::TopDeclList::cast(node)),
                        VarDecl => self.visitor.visit_variable_declaration(ast2::VarDecl::cast(node)),
                        FnDef => self.visitor.visit_function_defintion(ast2::FnDef::cast(node)),
                        BlockStmt => unreachable!(),
                        ExprStmt => self.visitor.visit_expression_statement(ast2::ExprStmt::cast(node)),
                        EmptyStmt => unimplemented!(),
            
                        Infix => self.visitor.visit_infix_expression(ast2::Infix::cast(node)),
                        Prefix => self.visitor.visit_prefix_expression(ast2::Prefix::cast(node)),
                        Group => self.visitor.visit_group_expression(ast2::Group::cast(node)),
                        BlockExpr => self.visitor.visit_block_expression(ast2::BlockExpr::cast(node)),
                        IfExpr => self.visitor.visit_if_expression(ast2::IfExpr::cast(node)),
                        WhileExpr => self.visitor.visit_while_expression(ast2::WhileExpr::cast(node)),
                        ReturnExpr => self.visitor.visit_return_expression(ast2::ReturnExpr::cast(node)),
                        ContinueExpr => self.visitor.visit_continue_expression(ast2::ContinueExpr::cast(node)),
                        AssignExpr => self.visitor.visit_assign_expression(ast2::AssignExpr::cast(node)),
                        CallExpr => self.visitor.visit_call_expression(ast2::CallExpr::cast(node)),
                        ArrayExpr => self.visitor.visit_array_expression(ast2::ArrayExpr::cast(node)),
                        BreakExpr => self.visitor.visit_break_expression(ast2::BreakExpr::cast(node)),
                        StmtList => self.visitor.visit_statement_list(ast2::StmtList::cast(node)),
                        TopDeclList => self.visitor.visit_top_level_declarations(ast2::TopDeclList::cast(node)),
                        Module => self.visitor.visit_module(ast2::Module::cast(node)),
                        ImportDecl => self.visitor.visit_import_declaration(ast2::ImportDecl::cast(node)),
                        EnumDecl => self.visitor.visit_enum_declaration(ast2::EnumDecl::cast(node)),
                        VariantList => self.visitor.visit_variant_list(ast2::VariantList::cast(node)),
                        Variant => self.visitor.visit_variant(ast2::Variant::cast(node)),
                        StructDecl => self.visitor.visit_struct_declaration(ast2::StructDecl::cast(node)),
                        FieldList => self.visitor.visit_field_list(ast2::FieldList::cast(node)),
                        Field => self.visitor.visit_field(ast2::Field::cast(node)),
                        TypeAlias => self.visitor.visit_type_alias(ast2::TypeAlias::cast(node)),
                        ConstDecl => self.visitor.visit_const_declaration(ast2::ConstDecl::cast(node)),
                        TupleExpr => self.visitor.visit_tuple_expression(ast2::TupleExpr::cast(node)),
                        TupleType => self.visitor.visit_tuple_type(ast2::TupleType::cast(node)),
                        ArrayType => self.visitor.visit_array_type(ast2::ArrayType::cast(node)),
                        GroupType => self.visitor.visit_group_type(ast2::GroupType::cast(node)),
                        FnType => self.visitor.visit_fn_type(ast2::FnType::cast(node)),
                        FnTypeParamList => self.visitor.visit_fn_type_param_list(ast2::FnTypeParamList::cast(node)),
                        ParamList => self.visitor.visit_param_list(ast2::ParamList::cast(node)),
                        Param => self.visitor.visit_param(ast2::Param::cast(node)),
                        ArgList => self.visitor.visit_arg_list(ast2::ArgList::cast(node)),
                        FieldAccessExpr => self.visitor.visit_field_access_expression(ast2::FieldAccessExpr::cast(node)),
                        IndexExpr => self.visitor.visit_index_expression(ast2::IndexExpr::cast(node)),
                        MethodCall => self.visitor.visit_method_call(ast2::MethodCall::cast(node)),
                        FieldInit => self.visitor.visit_field_init(ast2::FieldInit::cast(node)),
                        StructExpr => self.visitor.visit_struct_expression(ast2::StructExpr::cast(node)),
                        FieldInitList => self.visitor.visit_field_init_list(ast2::FieldInitList::cast(node)),
                        Any if node.is_null() => {},
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
                    Tag::Ident => self.visitor.visit_ident(ast2::Ident::cast(token)),
                    Tag::Number => self.visitor.visit_number(ast2::Int::cast(token)),
                    Tag::String => self.visitor.visit_string(ast2::Str::cast(token)),
                    Tag::Bool => self.visitor.visit_bool(ast2::Bool::cast(token)),
                    Tag::Invalid => unreachable!("[DEV]: Did I forget to check if any errors occurred after parsing?"),
                    _ => unreachable!("[DEV]: A valid syntax tree should not contain a token of this type"),
                }
            }
        }
    }
}
