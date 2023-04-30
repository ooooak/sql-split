use std::io::{
    BufReader,
    Read,
};

pub struct Writer {
    max_file_size: usize,
    bytes_written: usize,
    output_dir: String,
    cache: String,
}

pub fn create_file(path: Path) {
    match File::create(&path).unwrap {
        Err(why) => {
            panic!("couldn't create {}: {}", display, why)
        },
        Ok(file) => file,
    };
}

impl Writer {
    pub fn new(max_file_size: usize) {
        // Path::new("output.txt")

    
        let content = "This is some text we want to write to the file.";
    
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        };
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