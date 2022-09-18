enum ParseErrorKind {
    
}

struct Loc {
    start: u32,
    len: u32,
    line: u32
}

pub struct ParseError {
    kind: ParseErrorKind,
    location: Loc
}