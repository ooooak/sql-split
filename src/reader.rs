use std::io::{BufReader};
use std::io::prelude::*;
use std::io;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub struct Reader<T> {
    buffer: [u8; DEFAULT_BUF_SIZE],
    pos: usize,
    reader: BufReader<T>,
    bytes_read: usize,
}

impl<T> Reader<T> where T: io::Read {
    pub fn new(file: T) -> Self {
        let mut reader = Self {
            buffer: [0; DEFAULT_BUF_SIZE],
            reader: BufReader::new(file),
            pos: 0,
            bytes_read: 0,
        };

        reader.fill_buffer();
        reader
    }

    fn fill_buffer(&mut self) {
        self.bytes_read = self.reader.read(&mut self.buffer).unwrap_or(0);
        self.pos = 0; // reset index
    }

    fn get_current_byte(&mut self) -> Option<u8> {
        if self.pos >= self.bytes_read {
            self.fill_buffer();
            if self.bytes_read == 0 {
                return None;
            }
        }
        Some(self.buffer[self.pos])
    }

    pub fn get(&mut self) -> Option<u8> {
        let byte = self.get_current_byte();
        self.pos += 1;
        byte
    }

    pub fn peek(&mut self) -> Option<u8> {
        self.get_current_byte()
    }

    pub fn peek_next(&mut self) -> Option<u8> {
        let byte = self.get_current_byte();
        byte
    }

    pub fn increment_index(&mut self){
        self.pos += 1;
    }
}


#[cfg(test)]
mod reader_test{
    use std::fs::File;
    use super::Reader;

    #[test]
    fn empty_file(){
        let file = File::open("./example-files/empty.txt").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.get(), None);
    }

    #[test]
    fn get(){
        let file = File::open("./example-files/content.txt").unwrap();
        let mut reader = Reader::new(file);
        
        assert_eq!(reader.get(), Some(b'1'));
        assert_eq!(reader.get(), Some(b'2'));
        assert_eq!(reader.get(), Some(b'3'));
        assert_eq!(reader.get(), Some(b'4'));
        assert_eq!(reader.get(), Some(b'5'));
        assert_eq!(reader.get(), Some(b'6'));
        assert_eq!(reader.get(), Some(b'7'));
        assert_eq!(reader.get(), Some(b'8'));
        assert_eq!(reader.get(), Some(b'9'));
        assert_eq!(reader.get(), Some(b'0'));
        assert_eq!(reader.get().is_none(), true);
        assert_eq!(reader.get().is_none(), true);
    }

    #[test]
    fn peek(){
        let file = File::open("./example-files/content.txt").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek(), Some(b'1'));
        let _skip_it = reader.get();

        assert_eq!(reader.peek(), Some(b'2'));
    }

    #[test]
    fn peek_next(){
        let file = File::open("./example-files/content.txt").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek_next(), Some(b'2'));
        assert_eq!(reader.get(), Some(b'1'));
        assert_eq!(reader.get(), Some(b'2'));
        assert_eq!(reader.peek(), Some(b'3'));
        assert_eq!(reader.peek_next(), Some(b'4'));
    }
}