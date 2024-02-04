struct Ident<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Ident<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(token.tag, Tag::Ident);

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
struct Str<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Str<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(token.tag, Tag::String);

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
struct Bool<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Bool<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(token.tag, Tag::Bool);

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
struct Int<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for Int<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(token.tag, Tag::Number);

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
struct Group<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Group<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Group);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Group<'s, 'b> {
    pub fn expr(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct Infix<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Infix<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Infix);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Infix<'s, 'b> {
    pub fn left(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn op(&self) -> &'b Token<'s> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Token(token) => token,
            _ => unreachable!(),
        }
    }
    pub fn right(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[2];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct Prefix<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Prefix<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Prefix);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Prefix<'s, 'b> {
    pub fn op(&self) -> &'b Token<'s> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => token,
            _ => unreachable!(),
        }
    }
    pub fn right(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct BlockExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for BlockExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::BlockExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> BlockExpr<'s, 'b> {
    pub fn body(&self) -> StmtList<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <StmtList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct IfExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for IfExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::IfExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> IfExpr<'s, 'b> {
    pub fn condition(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn consequence(&self) -> BlockExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <BlockExpr as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
    pub fn alternate(&self) -> IfAlt<'s, 'b> {
        let elem = &self.node.children.0[2];
        match elem {
            NodeChild::Node(node) => <IfAlt as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
enum IfAlt<'s, 'b> {
    ElseIf(IfExpr<'s, 'b>),
    Else(BlockExpr<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for IfAlt<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind {
            NodeKind::IfExpr => IfAlt::ElseIf(<IfExpr as AstNode>::cast(node)),
            NodeKind::BlockExpr => IfAlt::Else(<BlockExpr as AstNode>::cast(node)),
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

struct WhileExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for WhileExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::WhileExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> WhileExpr<'s, 'b> {
    pub fn condition(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn consequence(&self) -> BlockExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <BlockExpr as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct ReturnExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ReturnExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ReturnExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ReturnExpr<'s, 'b> {
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct AssignExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for AssignExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::AssignExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> AssignExpr<'s, 'b> {
    pub fn ident(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct CallExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for CallExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::CallExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> CallExpr<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn args(&self) -> ArgList<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <ArgList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct ArrayExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ArrayExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ArrayExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ArrayExpr<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = Expr<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        })
    }
}
struct BreakExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for BreakExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::BreakExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> BreakExpr<'s, 'b> {
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct ContinueExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ContinueExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ContinueExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ContinueExpr<'s, 'b> {
    pub fn label(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
struct FnDef<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FnDef<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::FnDef);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FnDef<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn params(&self) -> ParamList<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <ParamList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
    pub fn body(&self) -> BlockExpr<'s, 'b> {
        let elem = &self.node.children.0[2];
        match elem {
            NodeChild::Node(node) => <BlockExpr as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
    pub fn return_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[3];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
struct VarDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for VarDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::VarDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> VarDecl<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn var_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[2];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct ExprStmt<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ExprStmt<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ExprStmt);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ExprStmt<'s, 'b> {
    pub fn expr(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct EmptyStmt<'s, 'b> {
    token: &'b Token<'s>,
}

impl<'s, 'b> AstToken<'s, 'b> for EmptyStmt<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(token.tag, Tag::Semicolon);

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> {
        self.token
    }
}
struct StmtList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for StmtList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::StmtList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> StmtList<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = Stmt<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Stmt as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
enum TopLevelDecl<'s, 'b> {
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
        match node.kind {
            NodeKind::Module => TopLevelDecl::Mod(<Module as AstNode>::cast(node)),
            NodeKind::ImportDecl => TopLevelDecl::Import(<ImportDecl as AstNode>::cast(node)),
            NodeKind::EnumDecl => TopLevelDecl::Enum(<EnumDecl as AstNode>::cast(node)),
            NodeKind::FnDef => TopLevelDecl::Fn(<FnDef as AstNode>::cast(node)),
            NodeKind::StructDecl => TopLevelDecl::Struct(<StructDecl as AstNode>::cast(node)),
            NodeKind::TypeAlias => TopLevelDecl::Type(<TypeAlias as AstNode>::cast(node)),
            NodeKind::ConstDecl => TopLevelDecl::Const(<ConstDecl as AstNode>::cast(node)),
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

struct Module<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Module<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Module);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Module<'s, 'b> {
    pub fn decls(&self) -> impl Iterator<Item = TopLevelDecl<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <TopLevelDecl as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
struct ImportDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ImportDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ImportDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ImportDecl<'s, 'b> {
    pub fn path(&self) -> impl Iterator<Item = Ident<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        })
    }
}
struct EnumDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for EnumDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::EnumDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> EnumDecl<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn variants(&self) -> VariantList<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <VariantList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct VariantList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for VariantList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::VariantList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> VariantList<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = Variant<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Variant as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
struct Variant<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Variant<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Variant);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Variant<'s, 'b> {
    pub fn tag(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn variant_type(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
struct StructDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for StructDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::StructDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> StructDecl<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn fields(&self) -> FieldList<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <FieldList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct FieldList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FieldList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::FieldList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FieldList<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = Field<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Field as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
struct Field<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Field<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Field);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Field<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn field_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
struct TypeAlias<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TypeAlias<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::TypeAlias);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TypeAlias<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn type_expr(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
struct ConstDecl<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ConstDecl<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ConstDecl);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ConstDecl<'s, 'b> {
    pub fn name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn const_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
    pub fn value(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[2];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
struct MethodCall<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for MethodCall<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::MethodCall);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> MethodCall<'s, 'b> {
    pub fn receiver(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn method_name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn args(&self) -> ArgList<'s, 'b> {
        let elem = &self.node.children.0[2];
        match elem {
            NodeChild::Node(node) => <ArgList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct FieldAccessExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FieldAccessExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::FieldAccessExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FieldAccessExpr<'s, 'b> {
    pub fn parent(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn field_name(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
struct IndexExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for IndexExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::IndexExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> IndexExpr<'s, 'b> {
    pub fn container(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
    pub fn index(&self) -> Expr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        }
    }
}
enum Expr<'s, 'b> {
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
}
impl<'s, 'b> AstNode<'s, 'b> for Expr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind {
            NodeKind::Infix => Expr::Infix(<Infix as AstNode>::cast(node)),
            NodeKind::Prefix => Expr::Prefix(<Prefix as AstNode>::cast(node)),
            NodeKind::ContinueExpr => Expr::ContinueExpr(<ContinueExpr as AstNode>::cast(node)),
            NodeKind::BreakExpr => Expr::BreakExpr(<BreakExpr as AstNode>::cast(node)),
            NodeKind::ReturnExpr => Expr::ReturnExpr(<ReturnExpr as AstNode>::cast(node)),
            NodeKind::Group => Expr::Group(<Group as AstNode>::cast(node)),
            NodeKind::ArrayExpr => Expr::ArrayExpr(<ArrayExpr as AstNode>::cast(node)),
            NodeKind::CallExpr => Expr::CallExpr(<CallExpr as AstNode>::cast(node)),
            NodeKind::IndexExpr => Expr::IndexExpr(<IndexExpr as AstNode>::cast(node)),
            NodeKind::FieldAccessExpr => {
                Expr::FieldAccessExpr(<FieldAccessExpr as AstNode>::cast(node))
            }
            NodeKind::MethodCall => Expr::MethodCall(<MethodCall as AstNode>::cast(node)),
            NodeKind::TupleExpr => Expr::TupleExpr(<TupleExpr as AstNode>::cast(node)),
            NodeKind::IfExpr => Expr::IfExpr(<IfExpr as AstNode>::cast(node)),
            NodeKind::WhileExpr => Expr::WhileExpr(<WhileExpr as AstNode>::cast(node)),
            NodeKind::BlockExpr => Expr::BlockExpr(<BlockExpr as AstNode>::cast(node)),
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
struct TupleExpr<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TupleExpr<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::TupleExpr);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TupleExpr<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = Expr<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        })
    }
}
struct ArgList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ArgList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ArgList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ArgList<'s, 'b> {
    pub fn args(&self) -> impl Iterator<Item = Expr<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Expr as AstNode>::cast(node),
            NodeChild::Token(token) => <Expr as AstToken>::cast(token),
        })
    }
}
enum TypeExpr<'s, 'b> {
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
        match node.kind {
            NodeKind::TupleType => TypeExpr::TupleType(<TupleType as AstNode>::cast(node)),
            NodeKind::ArrayType => TypeExpr::ArrayType(<ArrayType as AstNode>::cast(node)),
            NodeKind::GroupType => TypeExpr::GroupType(<GroupType as AstNode>::cast(node)),
            NodeKind::FnType => TypeExpr::FnType(<FnType as AstNode>::cast(node)),
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
struct TupleType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for TupleType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::TupleType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> TupleType<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = TypeExpr<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        })
    }
}
struct ArrayType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ArrayType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ArrayType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ArrayType<'s, 'b> {
    pub fn element_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
    pub fn len(&self) -> Int<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Token(token) => <Int as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
}
struct GroupType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for GroupType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::GroupType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> GroupType<'s, 'b> {
    pub fn inner_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
}
struct FnType<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FnType<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::FnType);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FnType<'s, 'b> {
    pub fn params(&self) -> FnTypeParamList<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Node(node) => <FnTypeParamList as AstNode>::cast(node),
            _ => unreachable!(),
        }
    }
}
struct FnTypeParamList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for FnTypeParamList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::FnTypeParamList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> FnTypeParamList<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = TypeExpr<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        })
    }
}
enum Stmt<'s, 'b> {
    VarDecl(VarDecl<'s, 'b>),
    ExprStmt(ExprStmt<'s, 'b>),
}
impl<'s, 'b> AstNode<'s, 'b> for Stmt<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        match node.kind {
            NodeKind::VarDecl => Stmt::VarDecl(<VarDecl as AstNode>::cast(node)),
            NodeKind::ExprStmt => Stmt::ExprStmt(<ExprStmt as AstNode>::cast(node)),
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

struct ParamList<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for ParamList<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::ParamList);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> ParamList<'s, 'b> {
    pub fn items(&self) -> impl Iterator<Item = Param<'s, 'b>> {
        let list = self.node.children.0.as_ref();
        list.iter().map(|x| match x {
            NodeChild::Node(node) => <Param as AstNode>::cast(node),
            _ => unreachable!(),
        })
    }
}
struct Param<'s, 'b> {
    node: &'b Node<'s, 'b>,
}

impl<'s, 'b> AstNode<'s, 'b> for Param<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized,
    {
        debug_assert_eq!(node.kind, NodeKind::Param);

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> {
        self.node
    }
}

impl<'s, 'b> Param<'s, 'b> {
    pub fn ident(&self) -> Ident<'s, 'b> {
        let elem = &self.node.children.0[0];
        match elem {
            NodeChild::Token(token) => <Ident as AstToken>::cast(token),
            _ => unreachable!(),
        }
    }
    pub fn param_type(&self) -> TypeExpr<'s, 'b> {
        let elem = &self.node.children.0[1];
        match elem {
            NodeChild::Node(node) => <TypeExpr as AstNode>::cast(node),
            NodeChild::Token(token) => <TypeExpr as AstToken>::cast(token),
        }
    }
    // pub fn some_opt(&self) -> Option<SomeOpt<'s, 'b>> {
    //     let elem = &self.node.children.0[2..];
    //     elem. 
    // }
}
