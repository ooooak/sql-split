use clap::App;
use std::fs::File;
use std::str;
use std::path::Path;
// use std::ffi::OsStr;

pub struct CliArgs {
    pub name: String,
    pub output_size: usize,
    pub file: File
}

fn log_error(err: &str) -> ! {
    eprintln!("{}", err);
    std::process::exit(0)
}

fn get_output_size(input: Option<&str>) -> usize {
    if input.is_none() {
        log_error("output-size is required")
    }

    let val = input.unwrap();
    if val.len() < 3 {
        log_error("output-size has invalid length.")
    }

    let (number, format) = val.split_at(val.len() - 2);
    let offset  = match format {
        "kb" => 1024,
        "mb" => 1024 * 1024,
        "gb" => 1024 * 1024 * 1024,
        _ => 0,
    };

    if offset == 0 {
        log_error("output-size has invalid format. choose from kb, mb or gb.")
    }
    match number.parse::<usize>() {
        Ok(number) => offset * number,
        Err(_) => {
            log_error("unable to parse number output-size number")
        }
    }
}

fn get_dir_name(path: &Path) -> String {
    if let Some(val) = path.file_name()  {
        if let Some(val) = val.to_str() {
            let name = val.split(".").next();
            if let Some(name) = name {
                return name.replace(" ", "_")
            }
        }
    }
    log_error("invalid dir name");
}

fn get_fd(file: Option<&str>) -> (String, File) {
    if file.is_none() {
        log_error("File name is missing")
    }

    let path = Path::new(file.unwrap());
    if !path.exists() {
        log_error("File path is invalid")
    }

    let file = File::open(path);
    if file.is_err() {
        log_error("Unable to open file")
    }

    // its all fun and games until you try to get the file name
    let name = get_dir_name(path);
    (name, file.unwrap())
}


pub fn args() -> CliArgs {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let output_size = get_output_size(matches.value_of("OUTPUT_SIZE"));
    let (name, file) = get_fd(matches.value_of("INPUT"));


    CliArgs {
        name,
        file, 
        output_size,
    }
}