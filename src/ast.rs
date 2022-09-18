use crate::token::{Tag, Token};
use std::convert::From;
use serde::Serialize;
use serde::Serializer;

use crate::bumping::*;

pub fn tag_is_binop(tag: Tag) -> bool {
    match tag {
        Tag::Plus
        | Tag::Minus
        | Tag::Slash
        | Tag::Asterisk
        | Tag::Dot
        | Tag::Bang
        | Tag::BangEqual
        | Tag::Equal
        | Tag::EqualEqual
        | Tag::Greater
        | Tag::GreaterEqual
        | Tag::Less
        | Tag::LessEqual => true,

        _ => false,
    }
}

pub fn tag_is_unaryop(tag: Tag) -> bool {
    match tag {
        Tag::Minus | Tag::Bang => true,
        _ => false,
    }
}

pub fn tag_is_literal(tag: Tag) -> bool {
    match tag {
        Tag::Ident | Tag::String | Tag::Number | Tag::Bool => true,
        _ => false,
    }
}

#[derive(Debug, Serialize)]
pub enum Node<'a, 'bump> {
    VarDecl(Box<'bump, VarDecl<'a, 'bump>>),
    FuncDecl(Box<'bump, FuncDecl<'a, 'bump>>),
    BlockStmt(BlockStmt<'a, 'bump>),
    #[serde(rename = "ExprStmt")]
    Expr(ExprStmt<'a, 'bump>),
    EmptyStmt(EmptyStmt<'a>),

    Id(Ident<'a>),
    Str(Str<'a>),
    Bool(Bool<'a>),
    Int(Int<'a>),
    Infix(Box<'bump, Infix<'a, 'bump>>),
    Prefix(Box<'bump, Prefix<'a, 'bump>>),
    Group(Box<'bump, Group<'a, 'bump>>),
    BlockExpr(Box<'bump, BlockExpr<'a, 'bump>>),
    If(Box<'bump, IfExpr<'a, 'bump>>),
    While(Box<'bump, WhileExpr<'a, 'bump>>),
    Return(Box<'bump, ReturnExpr<'a, 'bump>>),
    Assign(Box<'bump, AssignExpr<'a, 'bump>>),
    Call(Box<'bump, CallExpr<'a, 'bump>>),
    Array(Box<'bump, ArrayExpr<'a, 'bump>>),
    Break(Box<'bump, BreakExpr<'a, 'bump>>),
}

#[derive(Debug, Serialize)]
pub enum Item<'a, 'bump> {
    FuncDecl(Box<'bump, FuncDecl<'a, 'bump>>)
}

#[derive(Debug, Serialize)]
pub enum Stmt<'a, 'bump> {
    VarDecl(Box<'bump, VarDecl<'a, 'bump>>),
    FuncDecl(Box<'bump, FuncDecl<'a, 'bump>>),
    Block(BlockStmt<'a, 'bump>),
    #[serde(rename = "ExprStmt")]
    Expr(ExprStmt<'a, 'bump>),
    Empty(EmptyStmt<'a>)
}

#[derive(Debug)]
pub enum Expr<'a, 'bump> {
    Id(Ident<'a>),
    Str(Str<'a>),
    Bool(Bool<'a>),
    Int(Int<'a>),
    Infix(Box<'bump, Infix<'a, 'bump>>),
    Prefix(Box<'bump, Prefix<'a, 'bump>>),
    Group(Box<'bump, Group<'a, 'bump>>),
    Block(Box<'bump, BlockExpr<'a, 'bump>>),
    If(Box<'bump, IfExpr<'a, 'bump>>),
    While(Box<'bump, WhileExpr<'a, 'bump>>),
    Return(Box<'bump, ReturnExpr<'a, 'bump>>),
    Assign(Box<'bump, AssignExpr<'a, 'bump>>),
    Call(Box<'bump, CallExpr<'a, 'bump>>),
    Array(Box<'bump, ArrayExpr<'a, 'bump>>),
    Break(Box<'bump, BreakExpr<'a, 'bump>>)
}

impl Serialize for Expr<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        match self {
            Expr::Id(node) => node.serialize(serializer),
            Expr::Str(node) => node.serialize(serializer),
            Expr::Bool(node) => node.serialize(serializer),
            Expr::Int(node) => node.serialize(serializer),
            Expr::Infix(node) => serializer.serialize_newtype_variant("", 4, "Infix", node),
            Expr::Prefix(node) => serializer.serialize_newtype_variant("", 5, "Prefix", node),
            Expr::Group(node) => serializer.serialize_newtype_variant("", 6, "Group", node),
            Expr::Block(node) => serializer.serialize_newtype_variant("", 7, "Block", node),
            Expr::If(node) => serializer.serialize_newtype_variant("", 8, "If", node),
            Expr::While(node) => serializer.serialize_newtype_variant("", 9, "While", node),
            Expr::Return(node) => serializer.serialize_newtype_variant("", 10, "Return", node),
            Expr::Assign(node) => serializer.serialize_newtype_variant("", 10, "Assign", node),
            Expr::Call(node) => serializer.serialize_newtype_variant("", 10, "Call", node),
            Expr::Array(node) => serializer.serialize_newtype_variant("", 10, "Array", node),
            Expr::Break(node) => serializer.serialize_newtype_variant("", 10, "Break", node),
        }
    }
}

impl<'a, 'bump> From<Expr<'a, 'bump>> for Node<'a,'bump> {
    fn from(expr: Expr<'a, 'bump>) -> Self {
        use Expr::*;
        match expr {
            Id(inner) => Self::Id(inner),
            Str(inner) => Self::Str(inner),
            Bool(inner) => Self::Bool(inner),
            Int(inner) => Self::Int(inner),
            Infix(inner) => Self::Infix(inner),
            Prefix(inner) => Self::Prefix(inner),
            Group(inner) => Self::Group(inner),
            Block(inner) => Self::BlockExpr(inner),
            If(inner) => Self::If(inner),
            While(inner) => Self::While(inner),
            Return(inner) => Self::Return(inner),
            Assign(inner) => Self::Assign(inner),
            Call(inner) => Self::Call(inner),
            Array(inner) => Self::Array(inner),
            Break(inner) => Self::Break(inner)
        }
    }
}

impl<'a, 'bump> From<Stmt<'a, 'bump>> for Node<'a, 'bump> {
    fn from(stmt: Stmt<'a, 'bump>) -> Self {
        use Stmt::*;
        match stmt {
            VarDecl(inner) => Self::VarDecl(inner),
            FuncDecl(inner) => Self::FuncDecl(inner),
            Block(inner) => Self::BlockStmt(inner),
            Expr(inner) => Self::Expr(inner),
            Empty(inner) => Self::EmptyStmt(inner)
        }
    }
}

impl<'a, 'bump> From<Item<'a, 'bump>> for Node<'a, 'bump> {
    fn from(item: Item<'a, 'bump>) -> Self {
        use Item::*;
        match item {
            FuncDecl(inner) => Self::FuncDecl(inner)
        }
    }
}

impl<'a, 'bump> From<Item<'a, 'bump>> for Stmt<'a, 'bump> {
    fn from(item: Item<'a, 'bump>) -> Self {
        use Item::*;
        match item {
            FuncDecl(inner) => Self::FuncDecl(inner)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Ident<'a>(pub Token<'a>);
#[derive(Debug, Serialize)]

pub struct Str<'a>(pub Token<'a>);
#[derive(Debug, Serialize)]
pub struct Bool<'a>(pub Token<'a>);
#[derive(Debug, Serialize)]
pub struct Int<'a>(pub Token<'a>);
#[derive(Debug, Serialize)]
pub struct Group<'a, 'bump>(pub Expr<'a, 'bump>);
#[derive(Debug, Serialize)]
pub struct Infix<'a, 'bump> {
    pub left: Expr<'a, 'bump>,
    pub op: Token<'a>,
    pub right: Expr<'a, 'bump>,
}
#[derive(Debug, Serialize)]
pub struct Prefix<'a, 'bump> {
    pub op: Token<'a>,
    pub right: Expr<'a, 'bump>,
}
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct BlockExpr<'a, 'bump> {
    pub body: Vec<'bump, Stmt<'a, 'bump>>,
}
#[derive(Debug, Serialize)]
pub struct IfExpr<'a, 'bump> {
    pub condition: Expr<'a, 'bump>,
    pub consequence: BlockExpr<'a, 'bump>,
    pub alternate: Option<Box<'bump, IfAlt<'a, 'bump>>>,
}

#[derive(Debug, Serialize)]
pub enum IfAlt<'a, 'bump> {
    ElseIf(IfExpr<'a, 'bump>),
    Else(BlockExpr<'a, 'bump>),
}
#[derive(Debug, Serialize)]
pub struct WhileExpr<'a, 'bump> {
    pub condition: Expr<'a, 'bump>,
    pub consequence: BlockExpr<'a, 'bump>,
}
#[derive(Debug, Serialize)]
pub struct ReturnExpr<'a, 'bump> {
    pub value: Option<Expr<'a, 'bump>>,
}
#[derive(Debug, Serialize)] 
pub struct AssignExpr<'a, 'bump> {
    pub ident: Ident<'a>,
    pub value: Expr<'a, 'bump>
}
#[derive(Debug, Serialize)] 
pub struct CallExpr<'a, 'bump> {
    pub name: Ident<'a>,
    pub args: Vec<'bump, Expr<'a, 'bump>>,
}
#[derive(Debug, Serialize)]
pub struct ArrayExpr<'a, 'bump> {
    pub items: Vec<'bump, Expr<'a, 'bump>>
}
#[derive(Debug, Serialize)]
pub struct BreakExpr<'a, 'bump> {
    pub value: Option<Expr<'a, 'bump>>,
}
#[derive(Debug, Serialize)]
pub struct FuncDecl<'a, 'bump> {
    pub name: Ident<'a>,
    pub params: Vec<'bump, Ident<'a>>,
    pub body: BlockStmt<'a, 'bump>,
}
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct BlockStmt<'a, 'bump> {
    pub body: Vec<'bump, Stmt<'a, 'bump>>,
}
#[derive(Debug, Serialize)]
pub struct VarDecl<'a, 'bump> {
    pub name: Ident<'a>,
    pub value: Option<Expr<'a, 'bump>>,
}
#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct ExprStmt<'a, 'bump> {
    pub expr: Expr<'a, 'bump>,
}
#[derive(Debug, Serialize)]
pub struct EmptyStmt<'a>(pub Token<'a>);
