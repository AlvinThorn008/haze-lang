const fs = require("fs");

const data = fs.readFileSync("language_nodes copy.txt").toString();

const fields = /(?:,([\[\]\w]+):([\w\[\]]+))/g;
const variants = ["Expr", "TypeExpr", "IfAlt", "Stmt", "TopLevelDecl"]; // Enums
const tokVariants = ["Expr", "TypeExpr"]; // Enums with at least one token variant
const tokens = ["Ident", "Str", "Bool", "Int", "EmptyStmt"]; // Token nodes
const tagSubs = {
    "Str": "String",
    "Int": "Number",
    "EmptyStmt": "Semicolon"
};

function optionWrap(expr, check, wrap) { return wrap ? `(${check}).then_some(${expr})` : expr }

/***
 * HASTLY HACKED TOGETHER
 */

const decls = lines(data);
console.log(decls);
const generated = decls.map(x => decl(x)).join("\n");
fs.writeFileSync("ast2.rs", generated);

/**
 * 
 * @param {string} data 
 * @returns {string}
 */
function lines(data) {
    return data.split('\n').filter(line => line.length > 0);
}

/**
 * 
 * @param {string} line 
 */
function decl(line) {
    const brace = line.indexOf('{');
    const angle = line.indexOf('<');
    if (brace + angle == -2) {
        console.log(line);
        return generateToken(line.trim());
    } else {
        if (brace != -1) {
            const name = line.slice(0, brace).trim();
            let s = line.slice(brace + 1).replaceAll(" ", "").slice(0, -1);
            const fieldPairs = s.split(",").map(x => x.split(":"));
            console.log(fieldPairs);
            return generateNode(name, fieldPairs);
            // const fieldPairs = Array.from(
            //     s.matchAll(fields), 
            //     match => [match[1], match[2]]);    
        }

        else {
            const name = line.slice(0, angle).trim();
            let s = line.slice(angle + 1).replaceAll(" ", "").slice(0, -1);
            const fieldPairs = s.split(",").map(x => x.split(":"));
            console.log(fieldPairs);
            return generateEnum(name, fieldPairs);
        }
    }
}

function generateEnum(name, fields) {

    // Make enum definition
    let union = `#[derive(Debug, Clone)]\npub enum ${name}<'s, 'b> {`;
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
        
        match node.kind.0 {
            ${nodeFields.map(([variantName, variantType]) => {
                let type = variantType ?? variantName;

                return `NodeType::${type} => ${name}::${variantName}(<${type} as AstNode>::cast(node))`
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

function generateToken(name) {
    const tagName = tagSubs[name] ?? name;

    let structAndImpl = `#[derive(Debug, Clone)]\npub struct ${name}<'s, 'b> { token: &'b Token<'s> }

impl<'s, 'b> AstToken<'s, 'b> for ${name}<'s, 'b> {
    fn cast(token: &'b Token<'s>) -> Self
        where
            Self: Sized {
        debug_assert!(token.tag == Tag::${tagName} || token.is_empty());

        Self { token }
    }

    fn token(&self) -> &'b Token<'s> { self.token }
}`;

    return structAndImpl;
}

function generateNode(name, fields) {
    let structAndImpl = `#[derive(Debug, Clone)]\npub struct ${name}<'s, 'b> { node: &'b Node<'s, 'b> }

impl<'s, 'b> AstNode<'s, 'b> for ${name}<'s, 'b> {
    fn cast(node: &'b Node<'s, 'b>) -> Self
        where
            Self: Sized {
        debug_assert_eq!(node.kind.0, NodeType::${name});

        Self { node }
    }

    fn node(&self) -> &'b Node<'s, 'b> { self.node }
}

impl<'s, 'b> ${name}<'s, 'b> {
`;

    for (let i = 0; i < fields.length; i++) { 
        const [fieldName, fieldType] = fields[i];
        structAndImpl += `const ${fieldName.toUpperCase()}: usize = ${i};\n`
    }

    for (let i = 0; i < fields.length; i++) {
        const [fieldName, fieldType] = fields[i];
        structAndImpl += generateField(fieldName, fieldType, i);
        structAndImpl += "\n";
    }

    // structAndImpl += `pub fn new_node() -> Node<'s, 'b> { Node { kind: NodeKind(NodeType::${name}, ) } }`;
    structAndImpl += "}";

    return structAndImpl;
}

/**
 * match meta(type) {
 *  Node => 
 *  List => 
 * }
 */

/**
 * 
 * @param {string} name 
 * @param {string} type 
 * @param {number} number 
 * @returns 
 */
function generateField(name, type, number,) {
    let isList = false, isVariant = false, isOptional = false, innerType = type;
    
    // Just to not confuse myself,
    // optional arrays (e.g. []?) are not supported in this generator
    if (type.at(-1) === '?') { isOptional = true; innerType = type.slice(0, -1) };
    if (type[0] === '[') {
        isList = true;
        innerType = innerType.slice(1, -1);
    }
    isVariant = tokVariants.includes(innerType);
    const isToken = innerType === "Token";
    const isTokenNode = tokens.includes(innerType);

    const returnType = isOptional ? `Option<${innerType}<'s, 'b>>` :
    `${isList
        ? `impl Iterator<Item = ${innerType}<'s, 'b>>` 
        : isToken ? "&'b Token<'s>" : `${innerType}<'s, 'b>` }`;

    return `   pub fn ${name}(&self) -> ${returnType} {
        ${isList ? 
        `let list = self.node.children();
        list.iter().map(|x| {
            match x {
                ${isVariant ? 
                `NodeChild::Node(node) => <${innerType} as AstNode>::cast(node),
                NodeChild::Token(token) => <${innerType} as AstToken>::cast(token)`
                : isTokenNode ?
                `NodeChild::Token(token) => <${innerType} as AstToken>::cast(token),
                _ => unreachable!()`
                :
                `NodeChild::Node(node) => <${innerType} as AstNode>::cast(node),
                _ => unreachable!()` 
                } 
            }
        })` : 

        `let elem = &self.node.children()[${number}];

        match elem {
            ${isVariant ? 
            `NodeChild::Node(node) => ${optionWrap(`<${innerType} as AstNode>::cast(node)`, "!node.is_null()", isOptional)},
            NodeChild::Token(token) => ${optionWrap(`<${innerType} as AstToken>::cast(token)`, "!token.is_empty()", isOptional)},`
            : isTokenNode ?
            `NodeChild::Token(token) => ${optionWrap(`<${innerType} as AstToken>::cast(token)`, "!token.is_empty()", isOptional)},
            _ => unreachable!()`
            : isToken ?
            `NodeChild::Token(token) => token,
            _ => unreachable!()`
            :
            `NodeChild::Node(node) => ${optionWrap(`<${innerType} as AstNode>::cast(node)`, "!node.is_null()", isOptional)},
            _ => unreachable!()` 
            } 
        }`
        }
    }`;
}

function generateStruct(name, fields) {

    const [
        structType, StructType, 
        innerType, kindExpr, tagParentName
    ] = fields.length === 0 ?
            ["token", "Token", "Token<'s>", "token.tag", "Tag"] :
            ["node", "Node", "Node<'s, 'b>", "node.kind", "NodeKind"];

    /* const structType = fields.length === 0 ? "token" : "node";
    const StructType = structType.toUpperCase();

    const innerType = fields.length === 0 ? "Token<'s>" : "Node<'s, 'b>";

    const kindExpr = fields.length === 0 ? "token.tag" : "node.kind";
    const tagParentName = fields.length === 0 ? "Tag" : */

    let struct = 
`struct ${name}<'s, 'b> { pub(crate) ${structType}: &'b ${innerType} }

impl<'s, 'b> Ast${StructType}<'s, 'b> for ${name}<'s, 'b> {
    fn cast(${structType}: &'b ${StructType}<'s, 'b>) -> Self
    where
        Self: Sized {
        debug_assert_eq!(${kindExpr}, ${tagParentName}::${name});

        ${name} { ${structType} }
    }
}`;

/**
 * fn ${structType}(&self) -> &'b ${StructType}<'s, 'b> {
        self.${structType}
    }
 */
    let fieldNo = 0;
    struct += `impl<'s, 'b> ${name}<'s, 'b> {\n`;
    for (const [name, type] of fields) {
        const [parsedType, accessor] = type[0] === '['
            ? [type.slice(1, -1), ".as_ref()"]
            : [type, `[${fieldNo}]`]

        const [StructT, structT, cast] = parsedType === "Token" 
        ? ["Token", "token", "token"] 
        : ["Node", "node", `<${type} as AstNode>::cast(node)`];


        struct += type[0] === '['
    ?
`   pub fn ${name}(&self) -> impl Iterator<Item = ${type}<'s, 'b>> {
        let field = &self.node.children.as_ref();

            match x {
                NodeChild::${StructT}(${structT}) => ${cast},
                _ => unreachable!()
            } 
        })
    }
`
    :
`   pub fn ${name}(&self) -> ${type}<'s, 'b> {
        let field = &self.node.children.0[${fieldNo}];

        match field {
            NodeChild::${StructT}(${structT}) => ${cast},
            _ => unreachable!()
        }
    }
`;
        fieldNo += 1;
    }

    struct += '}'
    return struct;
}







