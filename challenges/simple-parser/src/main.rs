use std::io;
use simple_parser::solve;
use rust_coding_challenges::utils::read_text_file_from_args;
fn main() -> io::Result<()> {
    let content = read_text_file_from_args()?;
    let word_map = solve(&content);

    for (word, lines) in word_map {
        println!("Word: '{}', Lines: {:?}", word, lines);
    }
    Ok(())
}