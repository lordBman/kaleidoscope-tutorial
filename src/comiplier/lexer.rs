use lalrpop_util::Lexer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'input>{
    Identifier(&'input str),
    Def,
    Extern,
    Number(f32)
}

pub fn construct_lexer(src: &str) -> Lexer<Token>{
    let mut lexer = Lexer::new(src).skipping(r"^\s+|#.*$");

    lexer.register_pattern(r"^def", |_| Token::Def);
    lexer.register_pattern(r"^extern", |_| Token::Extern);
    lexer.register_pattern(r"^\d+(\/d+)?", |s| {
        Token::Number(s.parse().expect("perser never fails"))
    });
    lexer.register_pattern(r"^[\w][\w\d_]*", |s| Token::Identifier(s));

    return lexer;
}