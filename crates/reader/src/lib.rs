use std::io::{
    BufReader,
    Read,
};

const DEFAULT_BUF_SIZE: usize = 1024;

pub struct Reader<T> {
    pub cursor: usize,
    buffer: [u8; DEFAULT_BUF_SIZE],
    bytes_read: usize,
    reader: BufReader<T>,
}

pub fn is_eof(b: u8) -> bool {
    return b == 0
}

impl<T> Reader<T> where T: Read {
    pub fn new(file: T) -> Self {
        // reader
        let mut reader = Self {
            buffer: [0; DEFAULT_BUF_SIZE],
            reader: BufReader::new(file),
            cursor: 0,
            bytes_read: 0,
        };

        reader.fill_buf();
        reader
    }

    fn fill_buf(&mut self)  {
        match self.reader.read(&mut self.buffer) {
            Ok(size) => {
                self.bytes_read = size;
                self.cursor = 0;
            },
            Err(e) => panic!("{:}", e),
        }
    }

    fn read_byte(&mut self) -> u8 {
        if self.bytes_read == 0 {
            // reached EOF
            return 0
        }
        
        // println!("cursor: {}, bytes_read: {}", self.cursor, self.bytes_read);
        if self.cursor < self.bytes_read {
            let byte = self.buffer.get(self.cursor).unwrap();
            return *byte;
        }

        // read next chunk of buffer
        self.fill_buf(); 
        self.read_byte()
    }

    pub fn get(&mut self) -> u8 {
        let byte = self.read_byte();
        self.cursor += 1;
        byte
    }

    pub fn peek(&mut self) -> u8  {
        self.read_byte()
    }

    pub fn peek_next(&mut self) -> u8  {
        self.cursor += 1;
        let item = self.read_byte();
        self.cursor -= 1;
        item
    }


    pub fn increment_cursor(&mut self) {
        self.cursor += 1;
    }
}


#[cfg(test)]
mod reader_test{
    use super::*;
    use std::fs::File;

    #[test]
    fn empty_file(){
        let file = File::open("../../resources/test_files/empty.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.get(), 0);
    }

    #[test]
    fn get(){
        let file = File::open("../../resources/test_files/content.sql").unwrap();
        let mut reader = Reader::new(file);

        assert_eq!(reader.get(), b'1');
        assert_eq!(reader.get(), b'2');
        assert_eq!(reader.get(), b'3');
        assert_eq!(reader.get(), b'4');
        assert_eq!(reader.get(), b'5');
        assert_eq!(reader.get(), b'6');
        assert_eq!(reader.get(), b'7');
        assert_eq!(reader.get(), b'8');
        assert_eq!(reader.get(), b'9');
        assert_eq!(reader.get(), b'0');
        assert_eq!(reader.get(), 0);
        assert_eq!(reader.get(), 0);
    }

    #[test]
    fn iter_all() {
        let file = File::open("../../resources/test_files/big.sql").unwrap();
        let mut reader = Reader::new(file);
        let mut len = 0; 
        loop {
            len += 1;
            if reader.get() == 0 {
                println!("{}", len);
                break;
            }
        }
        
    }

    #[test]
    fn peek(){
        let file = File::open("../../resources/test_files/content.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek(), b'1');
        let _skip_it = reader.get();

        assert_eq!(reader.peek(), b'2');
    }

    #[test]
    fn peek_next(){
        let file = File::open("../../resources/test_files/content.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek_next(), b'2');
        assert_eq!(reader.get(), b'1');
        assert_eq!(reader.get(), b'2');
        assert_eq!(reader.peek(), b'3');
        assert_eq!(reader.peek_next(), b'4');
    }


}