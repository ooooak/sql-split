use std::io::{BufReader, Read, Seek, SeekFrom};

pub struct Reader<T> {
    reader: BufReader<T>,
}

pub fn is_eof(b: u8) -> bool {
    b == 0
}

impl<T> Reader<T>
where
    T: Read + Seek,
{
    pub fn new(file: T) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    fn read_byte(&mut self, buff_size: usize) -> &[u8] {
        let mut byte = [0; buff_size];
        match self.reader.read(&mut byte) {
            Ok(0) => &[0], // reached EOF
            Ok(_) => byte,
            Err(e) => {
                panic!("{:}", e)
            },
        }
    }

    pub fn get(&mut self) -> u8 {
        let byte = self.read_byte();
        self.reader.seek(SeekFrom::Current(0)).unwrap();
        byte
    }

    pub fn peek(&mut self) -> u8 {
        let byte = self.read_byte();
        self.reader.seek(SeekFrom::Current(-1)).unwrap();
        byte
    }

    pub fn peek_next(&mut self) -> u8 {
        self.reader.seek(SeekFrom::Current(1)).unwrap();
        let byte = self.read_byte();
        self.reader.seek(SeekFrom::Current(-2)).unwrap();
        byte
    }

    pub fn peek_n(&mut self) -> u8 {
        self.reader.seek(SeekFrom::Current(1)).unwrap();
        let byte = self.read_byte();
        self.reader.seek(SeekFrom::Current(-2)).unwrap();
        byte
    }

    pub fn increment_cursor(&mut self) {
        self.reader.seek(SeekFrom::Current(1)).unwrap();
    }
}


#[cfg(test)]
mod reader_test{
    use super::*;
    use std::fs::File;

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