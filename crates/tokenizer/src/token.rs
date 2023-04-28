use std::str;

#[derive(Debug,PartialEq,Clone)]
pub enum Token{
    String(Vec<u8>),
    Keyword(Vec<u8>),
    Comment(Vec<u8>),
    InlineComment(Vec<u8>),
    Identifier(Vec<u8>),
    // could be /t or /n /r
    LineFeed(u8),
    Space,
    Comma,
    LP, 
    RP,
    SemiColon,
    Ignore(u8),
    Dot,
    EOF
}

impl Token {
    pub fn keyword(&self, string: &str) -> bool {
        match self {
            Token::Keyword(chunk) => {
                let value = str::from_utf8(&chunk).unwrap();
                value.to_lowercase() == string
            },
            _ => false,
        }
    }

    pub fn value(self) -> Vec<u8> {
        match self {
            Token::String(chunk) => chunk,
            Token::Keyword(chunk) => chunk,
            Token::Comment(chunk) => chunk,
            Token::InlineComment(chunk) => chunk,
            Token::Identifier(chunk) => chunk,
            Token::Ignore(byte) => vec![byte],
            Token::Comma => vec![b','],
            Token::LP => vec![b'('],
            Token::RP => vec![b')'],
            Token::SemiColon => vec![b';'],
            Token::Dot => vec![b'.'],
            Token::Space => vec![b' '],
            Token::LineFeed(byte) => vec![byte],
            Token::EOF => vec![],
        }        
    }
}