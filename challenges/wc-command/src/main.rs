
use memmap2::MmapOptions;
use wc_command::solve;
use std::{fs::File, io::{self, BufRead, BufReader, Read, Write}, str, time::Instant};

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    // let file_path = "/Users/namdo/Desktop/learntocodetogether/random_1m_lines.txt";
    let file_path = "/Users/namdo/Desktop/learntocodetogether/random_10m_lines.txt";
    let text = read_file_to_string(file_path)?;
    let wc = solve(&text);
    println!("total words: {:?}", wc);
    println!("total time: {:?}", start.elapsed());
    println!("OK: {}", 8503930 == wc);

    let time = Instant::now();
    let file = File::open(file_path)?;

    let mmap = unsafe { MmapOptions::new().map(&file)? };
    let text = unsafe { str::from_utf8_unchecked(&mmap) };
    let str_time = Instant::now();
    println!("finish converting to string: {:?}", str_time.elapsed());
    solve(text);
    println!("Elapsed time for mmap: {:?}", time.elapsed());
    Ok(())
}


fn million_lines(n: usize, file_path: &str, new_file_path: &str) -> io::Result<()> {
    let mut src = File::open(file_path)?;

    let mut content = String::new();
    src.read_to_string(&mut content)?;

    let mut des = File::create(new_file_path)?;

    for i in 0..n {
        des.write_all(content.as_bytes())?;
    }

    Ok(())
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
