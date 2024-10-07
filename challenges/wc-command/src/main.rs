
use rust_coding_challenges::utils::read_text_file_from_args;
use wc_command::solve;
use std::{fs::File, io::{self, BufRead, BufReader, Read}, time::Instant};
fn main() -> std::io::Result<()> {
    // let start = Instant::now();
    let file_path = "/Users/namdo/Desktop/learntocodetogether/random_1m_lines.txt";
    let text = read_file_to_string(file_path)?;
    let start = Instant::now();
    let wc = solve(text);
    println!("total words: {:?}", wc);
    println!("total time: {:?}", start.elapsed());
    println!("OK: {}", 8503930 == wc);
    Ok(())
}


fn million_lines(n: usize, file_path: &str, new_file_path: &str) {

}

fn read_file_to_string(file_path: &str) -> io::Result<String>{
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut content = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        content.push_str(&buffer);
        buffer.clear();
    }
    Ok(content)
}
