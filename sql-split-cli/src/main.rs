#[macro_use] 
extern crate clap;

mod tokenizer;
mod parser;
mod splitter;
mod cli;

use std::str;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use splitter::SplitterSettings;
use splitter::SplitterState;
use splitter::Splitter;

fn log_error(err: &str) -> ! {
    eprintln!("{}", err);
    process::exit(0)
}

fn create_file(dir_name: &String, name: usize) -> File {
    let file_name = format!("./{:?}/{:?}.sql", dir_name, name);
    File::create(file_name).unwrap()
}


fn main() {
    let args = cli::args();

    // Create dir
    let _ = std::fs::create_dir(format!("./{:?}", args.name));

    let mut splitter = Splitter::new(SplitterSettings {
        write: args.output_size,
        file: args.file,
    });

    let mut file_count = 1;
    let mut buffer = create_file(&args.name, file_count);
    let mut first_file = true;

    loop {
        match splitter.process() {
            SplitterState::Chunk(create_new_file, tokens) => {
                if create_new_file  {
                    if first_file == true {
                        first_file = false;
                        continue;
                    }
                    file_count += 1;
                    buffer = create_file(&args.name, file_count);
                }

                buffer.write_all(&tokens).unwrap();
            },
            SplitterState::SyntaxErr(e) => log_error(e.text),
            SplitterState::Done => break,
        }
    }
}
