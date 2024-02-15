use crate::tokenizer::{
    token_err::{TokenErr, terr},
    token::Token,
    tokenizer::Tokenizer
};

#[derive(Debug,PartialEq)]
pub enum TokenStream {
    Insert(Vec<u8>, Vec<u8>),
    ValuesTuple(Vec<u8>),
    Block(Vec<u8>),
    Comment(Vec<u8>),
    SpaceOrLineFeed(Vec<u8>),
}

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }
    
    pub fn read_while(&mut self, token: &Token) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            match self.tokenizer.token()? {
                t => {
                    if t == *token {
                        collection.extend(t.value());
                        break
                    }else{
                        collection.extend(t.value());
                    }
                },
                Token::EOF => {
                    return terr("invalid end of file")
                }
            }
        }

        Ok(collection)
    }

    pub fn values(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        loop {
            let token = self.tokenizer.token()?;
            match token {
                Token::LP => {
                    collection.extend(token.value());
                    match self.read_while(&Token::RP) {
                        Ok(val) => {
                            collection.extend(val);
                        },
                        Err(e) => return Err(e),
                    }                    
                },
                Token::Comma | Token::SemiColon => {
                    collection.extend(token.value());
                    break;
                },
                Token::EOF => return terr("Unable to parse values."),
                _ => collection.extend(token.value())
            }
        }
        Ok(collection)
    }

    pub fn values_tuple(&mut self) -> Result<Vec<u8>, TokenErr> {
        let mut collection = vec![];
        let value = self.read_while(&Token::RP)?; 
        collection.extend(value);
        loop {
            let token = self.tokenizer.token()?;
            match token {
                Token::EOF => return terr("Unable to parse values."),
                Token::Comma | Token::SemiColon => {
                    collection.extend(token.value());
                    break;
                },
                token => collection.extend(token.value()),
            }
        }
        Ok(collection)
    }

    fn insert(&mut self, token: Vec<u8>) ->  Result<(Vec<u8>, Vec<u8>), TokenErr> {
        let mut collection = token;
        let mut insert_stmt;

        loop {
            let token = self.tokenizer.token()?;
            if token == Token::EOF {
                return terr("Incomplete Insert statement.")
            }
            if token.keyword("values") {
                collection.extend(token.value());
                insert_stmt = collection.clone();
                insert_stmt.push(b' ');
                collection.extend(self.values()?);                        
                break;
            } 
            collection.extend(token.value());
        }

        Ok((collection, insert_stmt))
    }

    pub fn token_stream(&mut self) -> Result<Option<TokenStream>, TokenErr> {
        let token = self.tokenizer.token()?;
        match token {
            Token::EOF => Ok(None),
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
                    match self.read_while(&Token::SemiColon) {
                        Ok(val) => {
                            let mut output = token.value();
                            output.extend(val);
                            Ok(Some(TokenStream::Block(output)))
                        },
                        Err(e) => Err(e)  
                    }
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
    }
}





#[cfg(test)]
mod reader_test{
    use std::fs::File;
    use sql_split_reader::Reader;
    use crate::tokenizer::tokenizer::Tokenizer;
    use crate::tokenizer::token_err::TokenErr;

    use super::Parser;
    use super::TokenStream;

    type TS = Result<Option<TokenStream>, TokenErr>;    
    fn is_space(value: TS) -> bool {
        match value {
            Ok(Some(TokenStream::SpaceOrLineFeed(_))) => {
                true
            },
            _ => false,
        }
    }

    fn is_comment(value: TS) -> bool {
        match value {
            Ok(Some(TokenStream::Comment(_))) => {
                true
            },
            _ => false,
        }
    }

    fn valid_values_tuple(value: TS) -> (bool, &'static str) {
        match value {
            Ok(Some(TokenStream::ValuesTuple(tokens))) => {
                match tokens[tokens.len() - 1] {
                    b';' => (true, ""),
                    _ => (false, "Last token should be semicolon or comma"),
                }
            },
            _ => (false, "expected ValuesTuple"),
        }
    }

    fn valid_block(value: TS) -> (bool, &'static str) {
        match value {
            Ok(Some(TokenStream::Block(tokens))) => {
                match tokens[tokens.len() - 1] {
                    b';' => (true, ""),
                    _ => (false, "Last token should be semicolon"),
                }
            },
            _ => (false, "expected block"),
        }
    }

    fn valid_insert(value: TS) -> (bool, &'static str) {
        match value {
            Ok(Some(TokenStream::Insert(tokens, _))) => {
                match tokens[tokens.len() - 1] {
                    b';' => (true, ""),
                    b',' => (true, ""),
                    _ => (false, "Last token should be semicolon or Comma"),
                }
            },
            _ => (false, "expected insert statement"),
        }
    }
    
    #[test]
    fn tokenizer(){
        let file = File::open("../resources/test_db/small.sql").unwrap();
        let tokenizer = Tokenizer::new(Reader::new(file));
        let mut parser = Parser::new(tokenizer);
                
        // inline comment
        assert!(is_comment(parser.token_stream()), "Expecting a comment");
        assert!(is_comment(parser.token_stream()), "Expecting a comment");

        
        // comment ends with "\n"
        // so we only expect one new line 
        assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");

        // create table
        let (state, msg) = valid_block(parser.token_stream());
        assert!(state, "{}", msg);

        // white space or line feed
        assert!(is_space(parser.token_stream()), "white space");
        assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");

        // insert
        let (state, msg) = valid_insert(parser.token_stream());
        assert!(state, "{}", msg);

        assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");

        let (state, msg) = valid_insert(parser.token_stream());
        assert!(state, "{}", msg);

        // line feeds
        assert!(is_space(parser.token_stream()), "white space");
        assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");

        // set FOREIGN_KEY_CHECKS block
        let (state, msg) = valid_block(parser.token_stream());
        assert!(state, "{}", msg);
        
        // line feed
        assert!(is_space(parser.token_stream()), "white space");
        assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");

        // create table block
        let (state, msg) = valid_block(parser.token_stream());
        assert!(state, "{}", msg);

        // line feed
        assert!(is_space(parser.token_stream()), "white space");
        assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");
        // assert!(is_space(parser.token_stream()), "white space");

        // insert 
        let (state, msg) = valid_insert(parser.token_stream());
        assert!(state, "{}", msg);

        // value tuple
        let (state, msg) = valid_values_tuple(parser.token_stream());
        assert!(state, "{}", msg);
    }


}