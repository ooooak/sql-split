use sql_split_reader::Reader;

use crate::parser::parser::TokenStream;
use crate::parser::parser::Parser;
use crate::tokenizer::{
    tokenizer::Tokenizer, 
    token_err::TokenErr, 
};

pub struct SplitterSettings{
    pub write: usize,
    pub file: std::fs::File,
}

pub struct Splitter {
    parser: Parser,
    total_bytes: usize,
    max_write_size:usize,
    last_insert: Vec<u8>,
}

pub enum SplitterState{
    SyntaxErr(TokenErr),
    // Reached output limit. send the chunk
    Chunk(bool, Vec<u8>),
    // reached the EOF.
    Done,
}

impl Splitter {
    pub fn new(settings: SplitterSettings) -> Self {
        let tokenizer = Tokenizer::new(Reader::new(settings.file));
        Self {
            parser: Parser::new(tokenizer),
            total_bytes: 0,
            last_insert: Vec::new(),
            max_write_size: settings.write,
        }
    }

    fn send(&mut self, tokens: Vec<u8>, starting_total: usize) -> SplitterState {
        self.total_bytes += tokens.len();
        if self.reached_limit(self.total_bytes) {
            self.total_bytes = 0;
        }

        let new_file_starts = starting_total == 0;
        SplitterState::Chunk(new_file_starts, tokens)
    }

    fn reached_limit(&self, total: usize) -> bool{
        total >= self.max_write_size
    }

    fn copy_last_insert(&self, chunk: &mut Vec<u8>) {
        chunk.extend(&self.last_insert)
    }

    fn close_values_tuple(&self, chunk: &mut Vec<u8>){
        let len = chunk.len() - 1;
        chunk[len] = b';';
    }
    
    pub fn process(&mut self) -> SplitterState {
        let starting_total = self.total_bytes;
        let token = self.parser.token_stream();
        if token.is_err() {
            return SplitterState::SyntaxErr(token.unwrap_err())
        }
        match token.unwrap() {
            TokenStream::Insert(insert_with_values, insert_stmt) => {
                self.last_insert = insert_stmt;                        
                self.send(insert_with_values, starting_total)
            },
            TokenStream::ValuesTuple(tokens) => {
                let mut ret = vec![];
                if starting_total == 0 {
                    // starting with fresh collection
                    // push last insert statement
                    self.copy_last_insert(&mut ret);
                }

                ret.extend(tokens);
                if self.reached_limit(starting_total + ret.len()) {
                    // maxed out in value tuple close statement
                    self.close_values_tuple(&mut ret)
                }
                
                self.send(ret, starting_total)
            },
            TokenStream::Block(tokens) => self.send(tokens, starting_total),
            TokenStream::Comment(tokens) |
            TokenStream::SpaceOrLineFeed(tokens) => self.send(tokens, starting_total),
            TokenStream::End => SplitterState::Done,
        }
    }
}