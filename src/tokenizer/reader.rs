use std::io::{BufReader, Seek, Read, SeekFrom};

const DEFAULT_BUF_SIZE: usize = 1024;

pub struct Reader<T>{
    buffer: [u8; DEFAULT_BUF_SIZE],
    index: usize,
    reader: BufReader<T>,
    bytes_read: usize,
}

impl<T> Reader<T> where T: Seek + Read {
    pub fn new(file: T) -> Self {
        // reader
        let mut reader = Self {
            buffer: [0; DEFAULT_BUF_SIZE],
            reader: BufReader::new(file),
            index: 0,
            bytes_read: 0,
        };

        reader.fill_buf();
        reader
    }

    pub fn get(&mut self) -> Option<u8> {
        let byte = self.read_byte();
        self.index += 1;
        byte
    }

    pub fn peek(&mut self) -> Option<u8> {
        self.read_byte()
    }

    pub fn peek_next(&mut self) -> Option<u8> {
        if (self.index+1) < self.bytes_read {
            self.index += 1;
            let item = self.read_byte();
            self.index -= 1;
            item
        } else {
            let b = self.reader.seek(SeekFrom::Current(1)).unwrap();
            let b = Some(b as u8);
            let _ = self.reader.seek(SeekFrom::Current(-1));
            return b

            // unimplemented!()
        }
    }

    pub fn increment_index(&mut self){
        self.index += 1;
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.bytes_read == 0 {
            return None
        }

        if self.index < self.bytes_read {
            let byte = self.buffer.get(self.index).unwrap();
            return Some(*byte)
        }

        // out of index load next buffer
        self.fill_buf();
        self.read_byte()
    }

    fn fill_buf(&mut self) {
        match self.reader.read(&mut self.buffer) {
            Ok(size) => {
                self.bytes_read = size;
                self.index = 0; // reset index
            },
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}


#[cfg(test)]
mod reader_test{
    use std::fs::File;
    use super::Reader;

    #[test]
    fn empty_file(){
        let file = File::open("./resources/test_db/empty.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.get(), None);
    }

    #[test]
    fn get(){
        let file = File::open("./resources/test_db/content.sql").unwrap();
        let mut reader = Reader::new(file);
        
        assert_eq!(reader.get(), Some(b'1'));
        assert_eq!(reader.get(), Some(b'2'));
        assert_eq!(reader.get(), Some(b'3'));
        assert_eq!(reader.get(), Some(b'4'));
        println!("{:?}", reader.peek_next());

        assert_eq!(reader.get(), Some(b'5'));
        println!("{:?}", reader.peek_next());

        assert_eq!(reader.get(), Some(b'6'));
        println!("{:?}", reader.peek_next());

        assert_eq!(reader.get(), Some(b'7'));
        assert_eq!(reader.get(), Some(b'8'));
        assert_eq!(reader.get(), Some(b'9'));
        assert_eq!(reader.get(), Some(b'0'));
        assert_eq!(reader.get().is_none(), true);
        assert_eq!(reader.get().is_none(), true);
    }

    #[test]
    fn peek(){
        let file = File::open("./resources/test_db/content.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek(), Some(b'1'));
        let _skip_it = reader.get();

        assert_eq!(reader.peek(), Some(b'2'));
    }

    #[test]
    fn peek_next(){
        let file = File::open("./resources/test_db/content.sql").unwrap();
        let mut reader = Reader::new(file);
        assert_eq!(reader.peek_next(), Some(b'2'));
        assert_eq!(reader.get(), Some(b'1'));
        assert_eq!(reader.get(), Some(b'2'));
        assert_eq!(reader.peek(), Some(b'3'));
        assert_eq!(reader.peek_next(), Some(b'4'));
    }
}