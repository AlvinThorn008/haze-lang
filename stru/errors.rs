use crate::token::{Tag, Token};

#[derive(Debug)]
pub enum ParseErrorKind {
    ExpectedSemi,
    ExpectedLBrace,
    ExpectedBlock,
    UnexpectedEOF,
    ExpectedRBrace,

    Expected(Tag),

    // Import related errors

    MalformedImportPath,
    MissingImportDelimeter,

    // Struct related errors

    MissingFieldDelimeter,

    // Expr related errors

    ExpectedExpr,
    ExpectedOperator,
    BlockExprDisallowed,

    ExpectedArrayDelimeter,
    
    ExpectedIfOrBlock,

    ExpectedExprOrSemi,

    ExpectedFunctionParameters,
    ParamIncomplete,
    ExpectedColon,
    ExpectedReturnType,

    ExpectedVarDeclSeparator,

    ExpectedType,
}

#[derive(Debug)]
pub struct Loc {
    pub start: u32,
    pub len: u32,
    pub line: u32
}

impl Loc {
    pub fn new(start: u32, len: u32, line: u32) -> Self {
        Self { start, len, line }
    }

    pub fn from_token<'s>(token: Token<'s>) -> Loc { Loc::new(token.pos, token.value.len() as u32, token.line) }
}

#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub location: Loc
}

#[derive(Copy, Clone, Debug)]
pub enum ParsingError {
    /// Terminate all parsing
    /// 
    /// TODO: Maybe EOF should really be Fatal
    Fatal,
    
    /// Generic error for "well-contained" syntax errors
    /// A syntax error is well-contained if the parsing routine
    /// reaches a checkpoint token(e.g. semicolons or EOF)
    /// 
    /// TODO: Let `Failed` be used to signal that synchronization is required 
    Failed,

    // Errors for dealing with lists of constructs e.g. Struct fields, function params
    /// One or more syntax errors occurred but parsing other constructs may continue
    CouldRecover,

    /// A more local `Fatal`. For whenever one construct in a list is invalid, the entire list is invalid.
    Terminal
    


}