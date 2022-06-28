struct Parser<'a> {
    tokens: Lexer<'a>
}

impl<'a> Parser<'a> {
    fn parse(&mut self, source: &'a str) -> Program {
        let next = self.tokens.next().unwrap();

        match next.tag {
            Tag::Ident => {
                if self.tokens.next()
            }
        }
    }

}