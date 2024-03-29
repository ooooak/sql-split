use sql_split_reader::Reader;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_err::TokenErr;

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
            let byte = self.reader.get();
            match byte {
                Some(value) => {
                    collection.push(value);
                    if value == item {
                        break;
                    }
                },
                None => {
                    return Err(TokenErr{
                        text: "Unexpected end of the file."
                    })
                }
            }
        }
        
        Ok(collection)
    }

    fn keyword(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            let byte = self.reader.peek();
            match byte {
                Some(item)  => {
                    match item {
                        b'a'..=b'z' |
                        b'A'..=b'Z' => {
                            self.reader.increment_index();
                            collection.push(item);
                        },
                        _ => break,
                    }
                },
                None => {
                    return Err(TokenErr{
                        text:"While parsing keyword."
                    })
                }
            }
        }
        
        Ok(collection)
    }


    fn number(&mut self) -> Token {
        let mut collection = vec![];
        while let Some(byte @ b'0'..=b'9') = self.reader.peek() {
            self.reader.increment_index();
            collection.push(byte);
        }
        Token::String(collection)
    }

    fn read_string(&mut self, closing: u8) -> Result<Token, TokenErr> {
        let mut collection = vec![];
        let mut last_byte = self.reader.get().unwrap();
        collection.push(last_byte);

        loop {
            let byte = self.reader.get();
            if let Some(item) = byte {
                collection.push(item);
                if item == closing && last_byte != b'\\' {
                    break;
                }
                last_byte = item;
            }else{
                return Err(TokenErr{
                    text: "Unclosed string."
                })
            }
        }
        Ok(Token::String(collection))
    }

    fn singular(&mut self, token: Token) -> Result<Option<Token>, TokenErr> {
        self.reader.increment_index();
        Ok(Some(token))
    }
    
    pub fn token(&mut self) -> Result<Option<Token>, TokenErr> {
        match self.reader.peek() {
            Some(closing @ b'"') |
            Some(closing @ b'\'') => {
                Ok(Some(self.read_string(closing)?))
            },
            Some(byte @ b'/') => {
                if self.reader.peek_next() == Some(b'*') {
                    self.comment()
                }else{
                    self.reader.increment_index();
                    Ok(Some(Token::Ignore(byte)))
                }
            },
            Some(b'0'..=b'9') => Ok(Some(self.number())),
            Some(byte @ b'-') => {
                if self.reader.peek_next() == Some(b'-') {
                    Ok(Some(Token::InlineComment(self.read_till(b'\n')?)))
                }else{
                    self.reader.increment_index();
                    Ok(Some(Token::Ignore(byte)))
                }
            },
            Some(b'a'..=b'z') | 
            Some(b'A'..=b'Z') => {
                // let keyword = ;
                Ok(Some(Token::Keyword(self.keyword()?)))
            },
            Some(byte @ b'`') => {
                self.reader.increment_index(); // skip `
                let mut identifier = vec![byte];
                identifier.extend(self.read_till(b'`')?);
                Ok(Some(Token::Identifier(identifier)))
            },
            Some(b'.') => self.singular(Token::Dot),
            Some(b'(') => self.singular(Token::LP),
            Some(b')') => self.singular(Token::RP),
            Some(b';') => self.singular(Token::SemiColon),
            Some(b',') => self.singular(Token::Comma),
            Some(b' ') => self.singular(Token::Space),
            Some(byte @ b'\r') |  
            Some(byte @ b'\t') | 
            Some(byte @ b'\n') => self.singular(Token::LineFeed(byte)),
            Some(byte) => self.singular(Token::Ignore(byte)),
            None => Ok(None),
        }
    }

    fn comment(&mut self) -> Result<Option<Token>, TokenErr> {
        let mut collection = vec![];
        loop {
            let cr = self.reader.get();
            // eof
            if cr.is_none() {
                return Err(TokenErr{
                    text: "Incomplete multi-line comment."
                });
            }
            
            collection.push(cr.unwrap());
            if cr == Some(b'*') && self.reader.peek() == Some(b'/') {
                let get_peeked = self.reader.get();
                collection.push(get_peeked.unwrap());
                break
            }
        }
        Ok(Some(Token::Comment(collection)))
    }
}