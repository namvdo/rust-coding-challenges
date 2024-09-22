use std::env;
use std::fs;
use std::io;

pub fn read_text_file_from_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Usage: <file_path>"));
    }
    let file_path = &args[1];
    fs::read_to_string(file_path)
}