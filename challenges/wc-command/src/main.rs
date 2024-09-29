
use rust_coding_challenges::utils::read_text_file_from_args;
use wc_command::solve;
use std::{fs::File, time::Instant};
fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let file = File::open("/Users/namdo/Desktop/learntocodetogether/rust-coding-challenges/challenges/wc-command/tests/inputs/random_1m_lines.txt").expect("OK");
    let lines = solve(file);
    println!("total words: {:?}", lines);
    println!("total time: {:?}", start.elapsed());
    // Print results or perform other actions as needed
    Ok(())
}
