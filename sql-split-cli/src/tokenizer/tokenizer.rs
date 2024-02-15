use sql_split_reader::Reader;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_err::{TokenErr, terr};

pub struct Tokenizer {
    reader: Reader,
}

impl Tokenizer {
    pub fn new(reader: Reader) -> Self {
        Self {reader}
    }
    fn read_till(&mut self, item: u8) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            let b = self.reader.get();
            if b == 0 {
                return terr("Unexpected end of the file.");
            }
            collection.push(b);
            if b == item {
                break;
            }
        }
        
        Ok(collection)
    }

    fn keyword(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            let b = self.reader.peek();
            if b == 0 {
                return terr("While parsing keyword.")
            }
            match b {
                b'a'..=b'z' |
                b'A'..=b'Z' => {
                    self.reader.increment_index();
                    collection.push(b);
                },
                _ => break,
            }
        }
        
        Ok(collection)
    }


    fn number(&mut self) -> Token {
        let mut collection = vec![];
        while let byte @ b'0'..=b'9' = self.reader.peek() {
            self.reader.increment_index();
            collection.push(byte);
        }
        Token::String(collection)
    }

    fn read_string(&mut self, closing: u8) -> Result<Token, TokenErr> {
        let mut collection = vec![];
        let mut last_byte = self.reader.get();
        collection.push(last_byte);

        loop {
            let byte = self.reader.get();
            if byte == 0 {
                return terr("Unclosed string.")
            }
            collection.push(byte);
            if byte == closing && last_byte != b'\\' {
                break;
            }
            last_byte = byte;
        }
        Ok(Token::String(collection))
    }

    fn singular(&mut self, token: Token) -> Result<Token, TokenErr> {
        self.reader.increment_index();
        Ok(token)
    }
    
    pub fn token(&mut self) -> Result<Token, TokenErr> {
        let b = self.reader.peek();

        match b {
            b'"' | b'\'' =>  Ok(self.read_string(b)?),
            b'/' => {
                if self.reader.peek_next() == b'*' {
                    self.comment()
                }else{
                    self.reader.increment_index();
                    Ok(Token::Ignore(b))
                }
            },
            b'0'..=b'9' => Ok(self.number()),
            b'-' => {
                if self.reader.peek_next() == b'-' {
                    Ok(Token::InlineComment(self.read_till(b'\n')?))
                }else{
                    self.reader.increment_index();
                    Ok(Token::Ignore(b))
                }
            },
            b'a'..=b'z' | 
            b'A'..=b'Z' => {
                Ok(Token::Keyword(self.keyword()?))
            },
            b'`' => {
                self.reader.increment_index(); // skip `
                let mut identifier = vec![b];
                identifier.extend(self.read_till(b'`')?);
                Ok(Token::Identifier(identifier))
            },
            b'.'=> self.singular(Token::Dot),
            b'('=> self.singular(Token::LP),
            b')'=> self.singular(Token::RP),
            b';'=> self.singular(Token::SemiColon),
            b','=> self.singular(Token::Comma),
            b' '=> self.singular(Token::Space),
            b'\r' | b'\t' | b'\n' => self.singular(Token::LineFeed(b)),
            byte => self.singular(Token::Ignore(b)),
            0 => Ok(Token::EOF),
        }
    }

    fn comment(&mut self) -> Result<Token, TokenErr> {
        let mut collection = vec![];
        loop {
            let cr = self.reader.get();
            // eof
            if cr == 0 {
                return Err(TokenErr{
                    text: "Incomplete multi-line comment."
                });
            }
            
            collection.push(cr);
            if cr == b'*' && self.reader.peek() == b'/' {
                let get_peeked = self.reader.get();
                collection.push(get_peeked);
                break
            }
        }
        Ok(Token::Comment(collection))
    }
}