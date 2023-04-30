extern crate reader;
use reader::Reader;
use std::io::{
    Read,
};

pub struct Parser<T> {
    reader: Reader<T>,
    writer: Reader<T>,
}

impl<T> Parser<T> where T: Read {
    pub fn new(reader: Reader<T>) -> Self {
        Self { reader,  writer: reader }
    }
    
    pub fn read_while(&mut self, token: &Token) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            match self.tokenizer.token()? {
                Some(t) => {
                    if t == *token {
                        collection.extend(t.value());
                        break
                    }else{
                        collection.extend(t.value());
                    }
                },
                None => {
                    return Err(TokenErr{
                        text:"invalid end of file"
                    })
                }
            }
        }

        Ok(collection)
    }

    pub fn values(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            match self.tokenizer.token()? {
                Some(token @ Token::LP) => {
                    collection.extend(token.value());
                    match self.read_while(&Token::RP) {
                        Ok(val) => {
                            collection.extend(val);
                        },
                        Err(e) => return Err(e),
                    }                    
                },
                Some(token @ Token::Comma) => {
                    collection.extend(token.value());
                    break;
                },
                Some(token @ Token::SemiColon) => {
                    collection.extend(token.value());
                    break;
                },
                Some(token) => {
                    collection.extend(token.value());
                },
                None => {
                    return Err(TokenErr{
                        text: "Unable to parse values."
                    })
                }
            }
        }
        Ok(collection)
    }

    pub fn values_tuple(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        let value = self.read_while(&Token::RP)?; 
        collection.extend(value);
        loop {
            match self.tokenizer.token()? {
                Some(token @ Token::Comma) => {
                    collection.extend(token.value());
                    break;
                },
                Some(token @ Token::SemiColon) => {
                    collection.extend(token.value());
                    break;
                },
                Some(token) => collection.extend(token.value()),
                None => {
                    return Err(TokenErr{
                        text: "Unable to parse values."
                    })
                },
            }
        }

        Ok(collection)
    }

    fn insert(&mut self, token: Vec<u8>) ->  Result<(Vec<u8>, Vec<u8>), TokenErr> {
        let mut collection = token;
        let mut insert_stmt;

        loop {
            match self.tokenizer.token()? {
                Some(token) => {
                    if token.keyword("values") {
                        collection.extend(token.value());
                        insert_stmt = collection.clone();
                        insert_stmt.push(b' ');                        

                        collection.extend(self.values()?);                        
                        break;
                    }else{
                        collection.extend(token.value());
                    }
                },
                None => {
                    return Err(TokenErr{
                        text: "Incomplete Insert statement."
                    })
                },
            }
        }

        Ok((collection, insert_stmt))
    }

    pub fn token_stream(&mut self) {
        let byte = self.reader.get();

        match byte {
            b'a'..=b'Z' | 
            b'A'..=b'Z' => {
                insert

            }
        }

        // keyword is insert 
        match self.tokenizer.token()? {
            Some(token) => {
                match token {
                    Token::Keyword(_) => {
                        if token.keyword("insert") {
                            // parse insert statement
                            // should end with with , or ;
                            // example: "insert into xyz values (),"
                            // example: "insert into xyz values ();"

                            let (insert, insert_stmt) = self.insert(token.value())?;
                            Ok(Some(TokenStream::Insert(insert, insert_stmt)))
                        }else{
                            // we assume its a block handle blocks
                            // anything that ends with `;` and 
                            // start with create, drop or set etc etc
                            self.write_till(b";")
                        }
                    },
                    Token::LP => {
                        let mut output = token.value();
                        output.extend(self.values_tuple()?);
                        Ok(Some(TokenStream::ValuesTuple(output)))
                    }
                    Token::Comment(_) | 
                    Token::InlineComment(_) => {
                        Ok(Some(TokenStream::Comment(token.value())))
                    },
                    Token::RP |
                    Token::Dot |
                    Token::String(_) |
                    Token::Identifier(_) |
                    Token::Comma |
                    Token::Ignore(_) => {
                        Err(TokenErr{
                            text: "Invalid sql file."
                        })
                    },
                    Token::SemiColon |
                    Token::Space |
                    Token::LineFeed(_) => {
                        Ok(Some(TokenStream::SpaceOrLineFeed(token.value())))
                    }
                }
            },
            None => Ok(None),
        }
    }
}
