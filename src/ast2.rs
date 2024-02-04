use crate::token::{Tag, Token};
use bumpalo::Bump;
use serde::Serialize;
use serde::Serializer;
use std::convert::From;
use std::fmt::Debug;
use std::fmt::Display;
use std::iter::empty;
use std::iter::Map;

use crate::bumping::*;

pub fn tag_is_binop(tag: Tag) -> bool {
    match tag {
        Tag::Plus
        | Tag::Minus
        | Tag::Slash
        | Tag::Asterisk
        //| Tag::Dot
        | Tag::BangEqual
        | Tag::Equal
        | Tag::EqualEqual
        | Tag::Greater
        | Tag::GreaterEqual
        | Tag::Less
        | Tag::LessEqual
        //| Tag::DotDot
         => true,

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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum NodeType {
    // Should never exceed 127 variants as NodeType <-> NodeKind
    // uses the MSB as a null bit
    VarDecl,
    FnDef,
    BlockStmt,
    ExprStmt,
    EmptyStmt,

    Infix,
    Prefix,
    Group,
    BlockExpr,
    IfExpr,
    WhileExpr,
    ReturnExpr,
    ContinueExpr,
    AssignExpr,
    CallExpr,
    ArrayExpr,
    BreakExpr,
    StmtList,
    TopDeclList,
    Module,
    ImportDecl,
    EnumDecl,
    VariantList,
    Variant,
    StructDecl,
    FieldList,
    Field,
    TypeAlias,
    ConstDecl,
    TupleExpr,
    TupleType,
    ArrayType,
    GroupType,
    FnType,
    FnTypeParamList,
    ParamList,
    Param,
    ArgList,
    FieldAccessExpr,
    IndexExpr,
    MethodCall,
    StructExpr,
    FieldInitList,
    FieldInit,

    // Generic nodetype for any one of the above types.
    // Typically used as the default node type before a concrete one is assigned.
    Any,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum NodeAttr {
    None,
    Invalid,
    // Optional,
}

// impl NodeKind {
//     pub const fn to_type(self) -> NodeType {
//         NodeType(self as u8)
//     }

//     /// Mark a nodekind as nullable
//     pub const fn null(self) -> NodeType {
//         NodeType((self as u8) | 128) // set last bit
//     }
// }

// #[derive(PartialEq, Eq, Debug, Clone, Copy)]
// struct NodeType(u8);

// impl NodeType {
//     pub fn is_null(self) -> bool {
//         (self.0 & 128) != 0
//     }

//     pub const fn kind(self) -> NodeKind {
//         // SAFETY!: NodeType can only be constructed via `NodeKind::null`
//         // which only modifies the MSB.
//         // NodeKind should never exceed 127
//         unsafe { core::mem::transmute(self.0 & 127) }
//     }
// }

// #[derive(PartialEq, Eq, Debug, Clone, Copy)]
// #[repr(u8)]
// enum NodeClass {
//     IfAlt,
//     Expr,
//     TypeExpr,
//     TopLevelDecl,
//     Stmt,
// }

// impl NodeClass {
//     pub const fn to_data(self) -> NodeClassData {
//         NodeClassData(self as u8)
//     }

//     pub const fn from_id(self, id: u8) -> NodeClassData {
//         debug_assert!(id <= 16);
//         NodeClassData((self as u8) | id << 4)
//     }
// }

// /// NodeClass and instance ids
// #[derive(PartialEq, Eq, Debug, Clone, Copy)]
// struct NodeClassData(u8);

// impl NodeClassData {
//     pub const fn class(self) -> NodeClass {
//         // "SAFETY!":
//         // 1. It is assumed that NodeClass has no more than 16 variants.
//         // 2. NodeClassData can only be constructed through NodeClass, which never
//         // modifies the last 4 bits
//         // 3. ANDing with 15 (0b0000_1111) clears the first 4 bits (instance id)
//         unsafe { core::mem::transmute(self.0 & 15) }
//     }

//     pub const fn instance(self) -> u8 {
//         self.0 >> 4
//     }

//     pub const fn is(self, class: NodeClass, instance: u8) -> bool {
//         self == class.from_id(instance)
//     }
// }

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct NodeKind(pub(crate) NodeType, pub(crate) NodeAttr);

impl NodeKind {
    pub fn from_type(node_type: NodeType) -> Self {
        Self(node_type, NodeAttr::None)
    }
}

#[derive(Debug)]
pub struct Node<'s, 'b> {
    pub(crate) kind: NodeKind,
    pub(crate) children: Option<Box<'b, [NodeChild<'s, 'b>]>>,
}

struct PureString(String);
impl Debug for PureString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'s, 'b> Display for Node<'s, 'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ", self.kind.0);
        let alternate = f.alternate();
        f.debug_set()
            .entries(
                self.children
                    .as_ref()
                    .map(|x| x.0.as_ref())
                    .unwrap_or(&[])
                    .iter()
                    .map(|x| match x {
                        NodeChild::Node(node) => PureString(if alternate {
                            format!("{:#}", node)
                        } else {
                            format!("{}", node)
                        }),
                        NodeChild::Token(token) => PureString(format!(
                            "({:?} {:?}) @ {}:{}",
                            token.tag, token.value, token.line, token.pos
                        )),
                    }),
            )
            .finish()
    }
}

impl<'s, 'b> Node<'s, 'b> {
    pub fn null(node_type: NodeType) -> Self {
        Self {
            kind: NodeKind(node_type, NodeAttr::None),
            children: None,
        }
    }
    pub fn invalid(node_type: NodeType) -> Self {
        Self {
            kind: NodeKind(node_type, NodeAttr::Invalid),
            children: None,
        }
    }
    pub fn is_null(&self) -> bool {
        self.kind.1 != NodeAttr::Invalid && self.children.is_none()
    }
    pub fn is_invalid(&self) -> bool {
        self.kind.1 == NodeAttr::Invalid
    }
    pub fn children<'a>(&'a self) -> &'a [NodeChild<'s, 'b>] {
        let c = self.children.as_ref().unwrap();
        &c.0[..]
    }
}

#[derive(Debug)]
pub enum NodeChild<'s, 'b> {
    Node(Node<'s, 'b>),
    Token(Token<'s>),
}

pub struct NodeBuilder<'s, 'b> {
    kind: NodeKind,
    children: Vec<'b, NodeChild<'s, 'b>>,
}

impl<'s, 'b> NodeBuilder<'s, 'b> {
    pub fn from_type(kind: NodeType, bump: &'b Bump) -> Self {
        Self {
            kind: NodeKind(kind, NodeAttr::None),
            children: Vec::new_in(bump),
        }
    }

    pub fn add(&mut self, child: impl Into<NodeChild<'s, 'b>>) {
        self.children.0.push(child.into())
    }

    pub fn finish(self, invalid: bool) -> Node<'s, 'b> {
        let has_children = !self.children.0.is_empty();
        Node {
            kind: NodeKind(
                self.kind.0,
                if !has_children && invalid {
                    NodeAttr::Invalid
                } else {
                    NodeAttr::None
                },
            ),
            children: has_children.then_some(Box(self.children.0.into_boxed_slice())),
        }
    }
}

impl<'s, 'b> From<Node<'s, 'b>> for NodeChild<'s, 'b> {
    fn from(value: Node<'s, 'b>) -> Self {
        NodeChild::Node(value)
    }
}

impl<'s, 'b> From<Token<'s>> for NodeChild<'s, 'b> {
    fn from(value: Token<'s>) -> Self {
        NodeChild::Token(value)
    }
}

pub trait AstNode<'s, 'b> {
    /// Wraps a raw Node as a concrete AST type
    /// Null or invalid nodes should not be casted.
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized;

    fn node(&self) -> &'b Node<'s, 'b>;
}

pub trait AstToken<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized;

    fn token(&self) -> &'b Token<'s>;
}

// The remainder of this file was generated by a handy script

#[derive(Debug, Clone)]
pub struct Ident<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Ident<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(token.tag == Tag::Ident || token.is_empty());

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
#[derive(Debug, Clone)]
pub struct Str<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Str<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(token.tag == Tag::String || token.is_empty());

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
#[derive(Debug, Clone)]
pub struct Bool<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Bool<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(token.tag == Tag::Bool || token.is_empty());

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
#[derive(Debug, Clone)]
pub struct Int<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Int<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(token.tag == Tag::Number || token.is_empty());

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
#[derive(Debug, Clone)]
pub struct Group<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Group<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Group);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Group<'s, 'b> {
    const EXPR: usize = 0;
    pub fn expr(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Infix<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Infix<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Infix);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Infix<'s, 'b> {
    const LEFT: usize = 0;
    const OP: usize = 1;
    const RIGHT: usize = 2;
    pub fn left(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn op(&self) -> &'b Token<'s> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Token(token) => token,
            _ => unreachable!(),
        }
    }
    pub fn right(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[2];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Prefix<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Prefix<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Prefix);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Prefix<'s, 'b> {
    const OP: usize = 0;
    const RIGHT: usize = 1;
    pub fn op(&self) -> &'b Token<'s> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => token,
            _ => unreachable!(),
        }
    }
    pub fn right(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BlockExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for BlockExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::BlockExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> BlockExpr<'s, 'b> {
    const BODY: usize = 0;
    pub fn body(&self) -> StmtList<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <StmtList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IfExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for IfExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::IfExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> IfExpr<'s, 'b> {
    const CONDITION: usize = 0;
    const CONSEQUENCE: usize = 1;
    const ALTERNATE: usize = 2;
    pub fn condition(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn consequence(&self) -> BlockExpr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <BlockExpr as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
    pub fn alternate(&self) -> Option<IfAlt<'s, 'b>> {
        let elem = &self.node.children()[2];

        match elem {
            NodeChild::Node(node) => (!node.is_null()).then_some(<IfAlt as AstNode>::cast(node)),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfAlt<'s, 'b> {
    ElseIf(IfExpr<'s, 'b>),
    Else(BlockExpr<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for IfAlt<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind.0 {
            NodeType::IfExpr => IfAlt::ElseIf(<IfExpr as AstNode>::cast(node)),
            NodeType::BlockExpr => IfAlt::Else(<BlockExpr as AstNode>::cast(node)),
            _ => unreachable!(),
        }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        match self {
            IfAlt::ElseIf(inner) => inner.node(),
            IfAlt::Else(inner) => inner.node(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for WhileExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::WhileExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> WhileExpr<'s, 'b> {
    const CONDITION: usize = 0;
    const CONSEQUENCE: usize = 1;
    pub fn condition(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn consequence(&self) -> BlockExpr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <BlockExpr as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ReturnExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ReturnExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ReturnExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ReturnExpr<'s, 'b> {
    const VALUE: usize = 0;
    pub fn value(&self) -> Option<Expr<'s, 'b>> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => (!node.is_null()).then_some(<Expr as AstNode>::cast(node)),
            NodeChild::Token(token) => {
                (!token.is_empty()).then_some(<Expr as AstToken>::cast(token))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct AssignExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for AssignExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::AssignExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> AssignExpr<'s, 'b> {
    const IDENT: usize = 0;
    const VALUE: usize = 1;
    pub fn ident(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CallExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for CallExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::CallExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> CallExpr<'s, 'b> {
    const NAME: usize = 0;
    const ARGS: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn args(&self) -> ArgList<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <ArgList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ArrayExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ArrayExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ArrayExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ArrayExpr<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = Expr<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        })
    }
}
#[derive(Debug, Clone)]
pub struct BreakExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for BreakExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::BreakExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> BreakExpr<'s, 'b> {
    const VALUE: usize = 0;
    pub fn value(&self) -> Option<Expr<'s, 'b>> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => (!node.is_null()).then_some(<Expr as AstNode>::cast(node)),
            NodeChild::Token(token) => {
                (!token.is_empty()).then_some(<Expr as AstToken>::cast(token))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct ContinueExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ContinueExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ContinueExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ContinueExpr<'s, 'b> {
    const LABEL: usize = 0;
    pub fn label(&self) -> Option<Ident<'s, 'b>> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => {
                (!token.is_empty()).then_some(<Ident as AstToken>::cast(token))
            }
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StructExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for StructExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::StructExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> StructExpr<'s, 'b> {
    const NAME: usize = 0;
    const FIELDS: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn fields(&self) -> FieldInitList<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <FieldInitList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FieldInitList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FieldInitList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FieldInitList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FieldInitList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = FieldInit<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <FieldInit as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub struct FieldInit<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FieldInit<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FieldInit);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FieldInit<'s, 'b> {
    const NAME: usize = 0;
    const VALUE: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FnDef<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FnDef<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FnDef);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FnDef<'s, 'b> {
    const NAME: usize = 0;
    const PARAMS: usize = 1;
    const BODY: usize = 2;
    const RETURN_TYPE: usize = 3;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn params(&self) -> ParamList<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <ParamList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
    pub fn body(&self) -> BlockExpr<'s, 'b> {
        let elem = &self.node.children()[2];

        match elem {
            NodeChild::Node(node) => <BlockExpr as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
    pub fn return_type(&self) -> Option<TypeExpr<'s, 'b>> {
        let elem = &self.node.children()[3];

        match elem {
            NodeChild::Node(node) => (!node.is_null()).then_some(<TypeExpr as AstNode>::cast(node)),
            NodeChild::Token(token) => {
                (!token.is_empty()).then_some(<TypeExpr as AstToken>::cast(token))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct VarDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for VarDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::VarDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> VarDecl<'s, 'b> {
    const NAME: usize = 0;
    const VAR_TYPE: usize = 1;
    const VALUE: usize = 2;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn var_type(&self) -> Option<TypeExpr<'s, 'b>> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => (!node.is_null()).then_some(<TypeExpr as AstNode>::cast(node)),
            NodeChild::Token(token) => {
                (!token.is_empty()).then_some(<TypeExpr as AstToken>::cast(token))
            }
        }
    }
    pub fn value(&self) -> Option<Expr<'s, 'b>> {
        let elem = &self.node.children()[2];

        match elem {
            NodeChild::Node(node) => (!node.is_null()).then_some(<Expr as AstNode>::cast(node)),
            NodeChild::Token(token) => {
                (!token.is_empty()).then_some(<Expr as AstToken>::cast(token))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExprStmt<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ExprStmt<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ExprStmt);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ExprStmt<'s, 'b> {
    const EXPR: usize = 0;
    pub fn expr(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct EmptyStmt<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for EmptyStmt<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(token.tag == Tag::Semicolon || token.is_empty());

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
#[derive(Debug, Clone)]
pub struct StmtList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for StmtList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::StmtList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> StmtList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = Stmt<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Stmt as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub enum TopLevelDecl<'s, 'b> {
    Mod(Module<'s, 'b>),
    Import(ImportDecl<'s, 'b>),
    Enum(EnumDecl<'s, 'b>),
    Fn(FnDef<'s, 'b>),
    Struct(StructDecl<'s, 'b>),
    Type(TypeAlias<'s, 'b>),
    Const(ConstDecl<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for TopLevelDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind.0 {
            NodeType::Module => TopLevelDecl::Mod(<Module as AstNode>::cast(node)),
            NodeType::ImportDecl => TopLevelDecl::Import(<ImportDecl as AstNode>::cast(node)),
            NodeType::EnumDecl => TopLevelDecl::Enum(<EnumDecl as AstNode>::cast(node)),
            NodeType::FnDef => TopLevelDecl::Fn(<FnDef as AstNode>::cast(node)),
            NodeType::StructDecl => TopLevelDecl::Struct(<StructDecl as AstNode>::cast(node)),
            NodeType::TypeAlias => TopLevelDecl::Type(<TypeAlias as AstNode>::cast(node)),
            NodeType::ConstDecl => TopLevelDecl::Const(<ConstDecl as AstNode>::cast(node)),
            _ => unreachable!(),
        }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        match self {
            TopLevelDecl::Mod(inner) => inner.node(),
            TopLevelDecl::Import(inner) => inner.node(),
            TopLevelDecl::Enum(inner) => inner.node(),
            TopLevelDecl::Fn(inner) => inner.node(),
            TopLevelDecl::Struct(inner) => inner.node(),
            TopLevelDecl::Type(inner) => inner.node(),
            TopLevelDecl::Const(inner) => inner.node(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Module<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Module<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Module);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Module<'s, 'b> {
    const NAME: usize = 0;
    const DECLS: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn decls(&self) -> TopDeclList<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <TopDeclList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TopDeclList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TopDeclList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::TopDeclList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TopDeclList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = TopLevelDecl<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <TopLevelDecl as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub struct ImportDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ImportDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ImportDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ImportDecl<'s, 'b> {
    const PATH: usize = 0;
    pub fn path(&self) -> impl Iterator<Item = Ident<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub struct EnumDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for EnumDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::EnumDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> EnumDecl<'s, 'b> {
    const NAME: usize = 0;
    const VARIANTS: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn variants(&self) -> VariantList<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <VariantList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct VariantList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for VariantList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::VariantList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> VariantList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = Variant<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Variant as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub struct Variant<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Variant<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Variant);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Variant<'s, 'b> {
    const TAG: usize = 0;
    const VARIANT_TYPE: usize = 1;
    pub fn tag(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn variant_type(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct StructDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for StructDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::StructDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> StructDecl<'s, 'b> {
    const NAME: usize = 0;
    const FIELDS: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn fields(&self) -> FieldList<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <FieldList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FieldList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FieldList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FieldList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FieldList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = Field<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Field as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub struct Field<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Field<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Field);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Field<'s, 'b> {
    const NAME: usize = 0;
    const FIELD_TYPE: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn field_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TypeAlias<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TypeAlias<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::TypeAlias);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TypeAlias<'s, 'b> {
    const NAME: usize = 0;
    const TYPE_EXPR: usize = 1;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn type_expr(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ConstDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ConstDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ConstDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ConstDecl<'s, 'b> {
    const NAME: usize = 0;
    const CONST_TYPE: usize = 1;
    const VALUE: usize = 2;
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn const_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[2];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct MethodCall<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for MethodCall<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::MethodCall);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> MethodCall<'s, 'b> {
    const RECEIVER: usize = 0;
    const METHOD_NAME: usize = 1;
    const ARGS: usize = 2;
    pub fn receiver(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn method_name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn args(&self) -> ArgList<'s, 'b> {
        let elem = &self.node.children()[2];

        match elem {
            NodeChild::Node(node) => <ArgList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FieldAccessExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FieldAccessExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FieldAccessExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FieldAccessExpr<'s, 'b> {
    const PARENT: usize = 0;
    const FIELD_NAME: usize = 1;
    pub fn parent(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn field_name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct IndexExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for IndexExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::IndexExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> IndexExpr<'s, 'b> {
    const CONTAINER: usize = 0;
    const INDEX: usize = 1;
    pub fn container(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn index(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expr<'s, 'b> {
    Ident(Ident<'s, 'b>),
    Str(Str<'s, 'b>),
    Int(Int<'s, 'b>),
    Bool(Bool<'s, 'b>),
    Infix(Infix<'s, 'b>),
    Prefix(Prefix<'s, 'b>),
    ContinueExpr(ContinueExpr<'s, 'b>),
    BreakExpr(BreakExpr<'s, 'b>),
    ReturnExpr(ReturnExpr<'s, 'b>),
    Group(Group<'s, 'b>),
    ArrayExpr(ArrayExpr<'s, 'b>),
    CallExpr(CallExpr<'s, 'b>),
    IndexExpr(IndexExpr<'s, 'b>),
    FieldAccessExpr(FieldAccessExpr<'s, 'b>),
    MethodCall(MethodCall<'s, 'b>),
    TupleExpr(TupleExpr<'s, 'b>),
    IfExpr(IfExpr<'s, 'b>),
    WhileExpr(WhileExpr<'s, 'b>),
    BlockExpr(BlockExpr<'s, 'b>),
    StructExpr(StructExpr<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for Expr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind.0 {
            NodeType::Infix => Expr::Infix(<Infix as AstNode>::cast(node)),
            NodeType::Prefix => Expr::Prefix(<Prefix as AstNode>::cast(node)),
            NodeType::ContinueExpr => Expr::ContinueExpr(<ContinueExpr as AstNode>::cast(node)),
            NodeType::BreakExpr => Expr::BreakExpr(<BreakExpr as AstNode>::cast(node)),
            NodeType::ReturnExpr => Expr::ReturnExpr(<ReturnExpr as AstNode>::cast(node)),
            NodeType::Group => Expr::Group(<Group as AstNode>::cast(node)),
            NodeType::ArrayExpr => Expr::ArrayExpr(<ArrayExpr as AstNode>::cast(node)),
            NodeType::CallExpr => Expr::CallExpr(<CallExpr as AstNode>::cast(node)),
            NodeType::IndexExpr => Expr::IndexExpr(<IndexExpr as AstNode>::cast(node)),
            NodeType::FieldAccessExpr => {
                Expr::FieldAccessExpr(<FieldAccessExpr as AstNode>::cast(node))
            }
            NodeType::MethodCall => Expr::MethodCall(<MethodCall as AstNode>::cast(node)),
            NodeType::TupleExpr => Expr::TupleExpr(<TupleExpr as AstNode>::cast(node)),
            NodeType::IfExpr => Expr::IfExpr(<IfExpr as AstNode>::cast(node)),
            NodeType::WhileExpr => Expr::WhileExpr(<WhileExpr as AstNode>::cast(node)),
            NodeType::BlockExpr => Expr::BlockExpr(<BlockExpr as AstNode>::cast(node)),
            NodeType::StructExpr => Expr::StructExpr(<StructExpr as AstNode>::cast(node)),
            _ => unreachable!(),
        }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        match self {
            Expr::Infix(inner) => inner.node(),
            Expr::Prefix(inner) => inner.node(),
            Expr::ContinueExpr(inner) => inner.node(),
            Expr::BreakExpr(inner) => inner.node(),
            Expr::ReturnExpr(inner) => inner.node(),
            Expr::Group(inner) => inner.node(),
            Expr::ArrayExpr(inner) => inner.node(),
            Expr::CallExpr(inner) => inner.node(),
            Expr::IndexExpr(inner) => inner.node(),
            Expr::FieldAccessExpr(inner) => inner.node(),
            Expr::MethodCall(inner) => inner.node(),
            Expr::TupleExpr(inner) => inner.node(),
            Expr::IfExpr(inner) => inner.node(),
            Expr::WhileExpr(inner) => inner.node(),
            Expr::BlockExpr(inner) => inner.node(),
            Expr::StructExpr(inner) => inner.node(),
            _ => unreachable!(),
        }
    }
}
impl<'s, 'b> AstToken<'s, 'b> for Expr<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        match token.tag {
            Tag::Ident => Expr::Ident(<Ident as AstToken>::cast(token)),
            Tag::String => Expr::Str(<Str as AstToken>::cast(token)),
            Tag::Number => Expr::Int(<Int as AstToken>::cast(token)),
            Tag::Bool => Expr::Bool(<Bool as AstToken>::cast(token)),
            _ => unreachable!(),
        }
    }

    fn token(&self) -> &'b Token<'s> {
        match self {
            Expr::Ident(inner) => inner.token(),
            Expr::Str(inner) => inner.token(),
            Expr::Int(inner) => inner.token(),
            Expr::Bool(inner) => inner.token(),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TupleExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TupleExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::TupleExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TupleExpr<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = Expr<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        })
    }
}
#[derive(Debug, Clone)]
pub struct ArgList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ArgList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ArgList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ArgList<'s, 'b> {
    const ARGS: usize = 0;
    pub fn args(&self) -> impl Iterator<Item = Expr<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        })
    }
}
#[derive(Debug, Clone)]
pub enum TypeExpr<'s, 'b> {
    TupleType(TupleType<'s, 'b>),
    ArrayType(ArrayType<'s, 'b>),
    GroupType(GroupType<'s, 'b>),
    Ident(Ident<'s, 'b>),
    FnType(FnType<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for TypeExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind.0 {
            NodeType::TupleType => TypeExpr::TupleType(<TupleType as AstNode>::cast(node)),
            NodeType::ArrayType => TypeExpr::ArrayType(<ArrayType as AstNode>::cast(node)),
            NodeType::GroupType => TypeExpr::GroupType(<GroupType as AstNode>::cast(node)),
            NodeType::FnType => TypeExpr::FnType(<FnType as AstNode>::cast(node)),
            _ => unreachable!(),
        }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        match self {
            TypeExpr::TupleType(inner) => inner.node(),
            TypeExpr::ArrayType(inner) => inner.node(),
            TypeExpr::GroupType(inner) => inner.node(),
            TypeExpr::FnType(inner) => inner.node(),
            _ => unreachable!(),
        }
    }
}
impl<'s, 'b> AstToken<'s, 'b> for TypeExpr<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        match token.tag {
            Tag::Ident => TypeExpr::Ident(<Ident as AstToken>::cast(token)),
            _ => unreachable!(),
        }
    }

    fn token(&self) -> &'b Token<'s> {
        match self {
            TypeExpr::Ident(inner) => inner.token(),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TupleType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TupleType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::TupleType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TupleType<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = TypeExpr<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        })
    }
}
#[derive(Debug, Clone)]
pub struct ArrayType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ArrayType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ArrayType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ArrayType<'s, 'b> {
    const ELEMENT_TYPE: usize = 0;
    const LEN: usize = 1;
    pub fn element_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
    pub fn len(&self) -> Int<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Token(token) => <Int as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct GroupType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for GroupType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::GroupType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> GroupType<'s, 'b> {
    const INNER_TYPE: usize = 0;
    pub fn inner_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FnType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FnType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FnType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FnType<'s, 'b> {
    const PARAMS: usize = 0;
    pub fn params(&self) -> FnTypeParamList<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Node(node) => <FnTypeParamList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FnTypeParamList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FnTypeParamList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::FnTypeParamList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FnTypeParamList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = TypeExpr<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        })
    }
}
#[derive(Debug, Clone)]
pub enum Stmt<'s, 'b> {
    VarDecl(VarDecl<'s, 'b>),
    ExprStmt(ExprStmt<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for Stmt<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind.0 {
            NodeType::VarDecl => Stmt::VarDecl(<VarDecl as AstNode>::cast(node)),
            NodeType::ExprStmt => Stmt::ExprStmt(<ExprStmt as AstNode>::cast(node)),
            _ => unreachable!(),
        }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        match self {
            Stmt::VarDecl(inner) => inner.node(),
            Stmt::ExprStmt(inner) => inner.node(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParamList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ParamList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::ParamList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ParamList<'s, 'b> {
    const ITEMS: usize = 0;
    pub fn items(&self) -> impl Iterator<Item = Param<'s, 'b>> {
        let list = self.node.children();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Param as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
#[derive(Debug, Clone)]
pub struct Param<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Param<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind.0, NodeType::Param);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Param<'s, 'b> {
    const IDENT: usize = 0;
    const PARAM_TYPE: usize = 1;
    pub fn ident(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children()[0];

        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn param_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children()[1];

        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
