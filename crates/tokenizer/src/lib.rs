pub mod token;
pub mod token_err;

use reader::{Reader, is_eof};
use std::io;
use token::Token;
use token_err::TokenErr;

pub struct Tokenizer<T> {
    reader: Reader<T>,
}

impl<T> Tokenizer<T> where T: io::Read {
    pub fn new(reader: Reader<T>) -> Self {
        Self {reader}
    }

    fn read_till(&mut self, item: u8) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];

        loop {
            let byte = self.reader.get();
            if byte == 0 {
              return Err(TokenErr{
                text: "Unexpected end of the file."
              })
            }

            collection.push(byte);
            if byte == item {
                break;
            }
        }
        
        Ok(collection)
    }

    fn keyword(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            let byte = self.reader.peek();
              if byte == 0 {
                // EOF
                return Err(TokenErr{
                  text:"While parsing keyword."
              })
            }

            match byte {
              b'a'..=b'z' |
              b'A'..=b'Z' => {
                  self.reader.increment_cursor();
                  collection.push(byte);
              },
              _ => break,
          }
        }
        
        Ok(collection)
    }


    fn number(&mut self) -> Token {
        let mut collection = vec![];
        while let byte @ b'0'..=b'9' = self.reader.peek() {
            self.reader.increment_cursor();
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
            if is_eof(byte) {
              return Err(TokenErr{
                  text: "Unclosed string."
              })
            }
              collection.push(byte);
              if byte == closing && last_byte != b'\\' {
                  break;
              }
              last_byte = byte;

        }
        Ok(Token::String(collection))
    }

    fn singular(&mut self, token: Token) -> Result<Option<Token>, TokenErr> {
        self.reader.increment_cursor();
        Ok(Some(token))
    }
    
    pub fn token(&mut self) -> Result<Option<Token>, TokenErr> {
        match self.reader.peek() {
          0 => Ok(None),
          closing @ b'"' | closing @ b'\'' => {
                Ok(Some(self.read_string(closing)?))
            },
            byte @ b'/' => {
                if self.reader.peek_next() == b'*' {
                    self.comment()
                }else{
                    self.reader.increment_cursor();
                    Ok(Some(Token::Ignore(byte)))
                }
            },
            b'0'..=b'9' => Ok(Some(self.number())),
            byte @ b'-' => {
                if self.reader.peek_next() == b'-' {
                    Ok(Some(Token::InlineComment(self.read_till(b'\n')?)))
                }else{
                    self.reader.increment_cursor();
                    Ok(Some(Token::Ignore(byte)))
                }
            },
            b'a'..=b'z' | 
            b'A'..=b'Z' => {
                // let keyword = ;
                Ok(Some(Token::Keyword(self.keyword()?)))
            },
            byte @ b'`' => {
                self.reader.increment_cursor(); // skip `
                let mut identifier = vec![byte];
                identifier.extend(self.read_till(b'`')?);
                Ok(Some(Token::Identifier(identifier)))
            },
            b'.' => self.singular(Token::Dot),
            b'(' => self.singular(Token::LP),
            b')' => self.singular(Token::RP),
            b';' => self.singular(Token::SemiColon),
            b',' => self.singular(Token::Comma),
            b' ' => self.singular(Token::Space),
            byte @ b'\r' |  
            byte @ b'\t' | 
            byte @ b'\n' => self.singular(Token::LineFeed(byte)),
            byte => self.singular(Token::Ignore(byte)),
        }
    }

    fn comment(&mut self) -> Result<Option<Token>, TokenErr> {
        let mut collection = vec![];
        loop {
            let cr = self.reader.get();
            // eof
            if is_eof(cr) {
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
        Ok(Some(Token::Comment(collection)))
    }
}