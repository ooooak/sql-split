use std::io::{Read, Seek, SeekFrom};
use std::fs::File;

// const DEFAULT_BUF_SIZE: usize = 5;
const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub struct Reader {
    buffer: [u8; DEFAULT_BUF_SIZE],
    cursor: usize,
    file: File,
    bytes_read: usize,
}

impl Reader {
    pub fn new(file: File) -> Self {
        // reader
        let mut reader = Self {
            buffer: [0; DEFAULT_BUF_SIZE],
            file,
            cursor: 0,
            bytes_read: 0,
        };

        reader.fill_buf();
        reader
    }

    #[inline(always)]
    pub fn get(&mut self) -> Option<u8> {
        let byte = self.peek();
        self.cursor += 1;
        byte
    }

    pub fn peek_next(&mut self) -> Option<u8> {
        if self.next_in_current_buff() {
            self.peek_next_from_buff()
        } else {
            self.peek_next_from_file()
        }
    }

    #[inline(always)]
    pub fn increment_index(&mut self) {
        self.cursor += 1;
    }

    #[inline(always)]
    pub fn peek(&mut self) -> Option<u8> {
        if self.bytes_read == 0 {
            return None
        }

        if self.cursor < self.bytes_read {
            let byte = self.buffer.get(self.cursor).unwrap();
            return Some(*byte)
        }

        // out of index load next buffer
        self.fill_buf();

        self.peek()
    }

    fn next_in_current_buff(&self) -> bool {
        (self.cursor + 1) < self.bytes_read 
    }

    fn fill_buf(&mut self) {
        let size = self.file.read(&mut self.buffer).expect("unable to read buffer");
        self.bytes_read = size;
        self.cursor = 0;
    }

    fn peek_next_from_file(&mut self) -> Option<u8> {
        let mut tmp_buff = [0; 1];

        // move ahead
        let _ = self.file.seek(SeekFrom::Current(1)); 
        let read = self.file.read(&mut tmp_buff).expect("unable to read buff");
        if read == 0 {
            let _ = self.file.seek(SeekFrom::Current(-1)); 
            return None
        }

        // move back
        let _ = self.file.seek(SeekFrom::Current(-2)); 
        Some(tmp_buff[0])
    }

    fn peek_next_from_buff(&mut self) -> Option<u8> {
        self.cursor += 1;
        let item = self.peek();
        self.cursor -= 1;
        item
    }
}


#[cfg(test)]
mod reader_test{
    use std::fs::File;
    use super::Reader;

    #[test]
    fn empty_file(){
        let file = File::open("../resources/test_db/empty.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.get(), None);
    }

    #[test]
    fn get(){
        let file = File::open("../resources/test_db/content.sql").unwrap();
        let mut reader = Reader::new(file);
        
        assert_eq!(reader.get(), Some(b'1'));
        assert_eq!(reader.get(), Some(b'2'));
        assert_eq!(reader.get(), Some(b'3'));
        assert_eq!(reader.get(), Some(b'4'));
        println!("{:?}", char::from_u32(reader.peek_next().unwrap() as u32));

        assert_eq!(reader.get(), Some(b'5'));
        println!("{:?}", char::from_u32(reader.peek_next().unwrap() as u32));

        assert_eq!(reader.get(), Some(b'6'));
        println!("{:?}", char::from_u32(reader.peek_next().unwrap() as u32));

        assert_eq!(reader.get(), Some(b'7'));
        assert_eq!(reader.get(), Some(b'8'));
        assert_eq!(reader.get(), Some(b'9'));
        assert_eq!(reader.get(), Some(b'0'));
        assert_eq!(reader.get().is_none(), true);
        assert_eq!(reader.get().is_none(), true);
    }

    #[test]
    fn peek(){
        let file = File::open("../resources/test_db/content.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek(), Some(b'1'));
        let _skip_it = reader.get();

        assert_eq!(reader.peek(), Some(b'2'));
    }

    #[test]
    fn peek_next(){
        let file = File::open("../resources/test_db/content.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek_next(), Some(b'2'));
        assert_eq!(reader.get(), Some(b'1'));
        assert_eq!(reader.get(), Some(b'2'));
        assert_eq!(reader.peek(), Some(b'3'));
        assert_eq!(reader.peek_next(), Some(b'4'));
    }
}
