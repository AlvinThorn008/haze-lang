const fs = require("fs");

const variants = ["Expr", "TypeExpr", "IfAlt", "Stmt", "TopLevelDecl"]; // Enums
const tokVariants = ["Expr", "TypeExpr"]; // Enums with at least one token node variant
const tokens = ["Ident", "Str", "Bool", "Int", "EmptyStmt"]; // Token nodes
const tagSubs = {
    "Str": "String",
    "Int": "Number",
    "EmptyStmt": "Semicolon"
};

const data = fs.readFileSync("language_nodes.txt").toString();


function generateToken(name) {
    const tagName = tagSubs[name] ?? name;

    let structAndImpl = 
`struct ${name}<'s, 'b> { token: &'b Token<'s> }

impl<'s, 'b> AstToken<'s, 'b> for ${name}<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
        where
            Self: Sized {
        debug_assert_eq!(token.tag, Tag::${tagName});

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> { self.token }
}`;

    return structAndImpl;
}

/**
 * 
 * @param {string} type 
 */
function propInfo(type) {
    return { 
        list: type.includes('['),
        enum: variants.includes(type),
        enumWithTok: tokVariants.includes(type),
        optional: type.includes('?'),
        tokenNode: tokens.includes(type)
    }
}

function generateEnum(name, fields) {

    // Make enum definition
    let union = `enum ${name}<'s, 'b> {`;
    let nodeFields = fields.filter(x => !tokens.includes(x[1] ?? x[0]));
    let tokenFields = fields.filter(x => tokens.includes(x[1] ?? x[0]));
    for (const [variantName, variantType] of fields) {
        union += `\n\t${variantName}(${variantType ?? variantName}<'s, 'b>),`;
    }
    union += '\n}\n';

    if (nodeFields.length !== 0) {
        union += 
`impl<'s, 'b> AstNode<'s, 'b> for ${name}<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
    where
        Self: Sized {
        
        match node.kind {
            ${nodeFields.map(([variantName, variantType]) => {
                let type = variantType ?? variantName;

                return `NodeKind::${type} => ${name}::${variantName}(<${type} as AstNode>::cast(node))`
            }).join(',\n\t\t\t')},
            _ => unreachable!()
        }
    }

    fn node(&self) -> &'b Node<'s, 'b> { 
        match self {
            ${nodeFields.map(([variantName, variantType]) => {
                return `${name}::${variantName}(inner) => inner.node()`
            }).join(',\n\t\t\t')},
            _ => unreachable!()
        }
    }
}
`;
    }
    if (tokenFields.length !== 0) {
        union += 
`impl<'s, 'b> AstToken<'s, 'b> for ${name}<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
    where
        Self: Sized {
        
        match token.tag {
            ${tokenFields.map(([variantName, variantType]) => {
                let type = variantType ?? variantName;
                const tagName = tagSubs[type] ?? type;

                return `Tag::${tagName} => ${name}::${variantName}(<${type} as AstToken>::cast(token))`
            }).join(',\n\t\t\t')},
            _ => unreachable!()
        }
    }

    fn token(&self) -> &'b Token<'s> { 
        match self {
            ${tokenFields.map(([variantName, variantType]) => {
                return `${name}::${variantName}(inner) => inner.token()`
            }).join(',\n\t\t\t')},
            _ => unreachable!()
        }
    }
}`;
    }

    return union;
}

/**
 * 
 * @param {[string, string]} param0
 * @param {number} index 
 */
function generateField([name, type], index) {
    const info = propInfo(type);

    // isNode is true for enums, isTokenNode is true for enumswithToken
    // (isToken, ...false)
    // (isTokenNode, ...false)
    // (isNode, ...false)
    // (isTokenNode, isNode, ...false)
    const newInfo = {
        isToken: type === "Token", // modifiers on Token are not supported currently ([], ?)
        isTokenNode: info.tokenNode || info.enumWithTok,
        isNode: !(type == "Token" || info.tokenNode) || info.enum,
        isVariant: info.enum,
        innerType: type.replace(/[\[\]\?]/, "")
    };

    return `   pub fn ${name}(&self) -> ${info.list
        ? `impl Iterator<Item = ${innerType}<'s, 'b>>` 
        : type === "Token" ? "&'b Token<'s>" : `${innerType}<'s, 'b>` } {
            ${info.list ? listWrap(matchStmt("x")) : 
            `let elem = &self.node.children.0[${number}];
            ${matchStmt("elem")}`}
        }`
}

const matchStmt = (element, { isNode, isToken, isTokenNode, isVariant, innerType }) => {
    return `match ${element} { 
    ${isNode ? `NodeChild::Node(node) => <${innerType} as AstNode>::cast(node),` : ""}
    ${isTokenNode ? `NodeChild::Token(token) => <${innerType} as AstToken>::cast(token),` : ""}
    ${isToken ? `NodeChild::Token(token) => token,` : ""}
    ${isVariant ? "" : `_ => unreachable!()` }
}`
}

const listWrap = inner => `let list = self.node.children.0.as_ref();
list.iter().map(|x| {
    ${inner}
})`

/**
 * 
 * @param {string} data 
 * @returns {string[]}
 */
function lines(data) {
    return data.split('\n').filter(line => line.length > 0);
}

/**
 * @param {string} line
 * @returns { { declName: string, props: [string, string][], type: "enum" | "struct" | "token_node" } }
 */
function decl(line) {
    const declDelim = line.search(/(\{|\>)/);
    const declName = line.slice(0, declDelim).trimEnd();

    const propsString = line.slice(declDelim + 1, -1);
    const props = propsString
        .replace(" ", "")
        .split(",")
        .map(([part1, part2]) => [part1, part2 ?? part1]); // for unnamed variants

    return { 
        declName, 
        props, 
        type: line[declDelim] === '<' 
            ? "enum" 
            : line[declDelim] === '{'
            ? "struct"
            : "token_node"
    };
}

const decls = lines(data).map(decl);





  