
use wc_command::{count_word_occurrences, count_words, read_file_as_string};
use std::{env, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] != "rccwc" {
        eprintln!("Usage: rccwc -w <file_path> or rccwc -wo <word> <file_path>");
        return;
    }

    match args[2].as_str() {
        "-w" => {
            if args.len() != 4 {
                eprintln!("Usage: rccwc -w <file>");
                return;
            }
            let start = Instant::now();
            let file_path = &args[3];
            let text = read_file_as_string(&file_path);
            let total_words = count_words(&text);
            println!("Total words: {}, elapsed time: {:?}", total_words, start.elapsed());
        } 
        "-wo" => {
            if args.len() != 5 {
                eprintln!("Usage: rccwc -wc <word> <file_path>");
                return;
            }
            let start = Instant::now();
            let word = &args[3];
            let file_path = &args[4];
            let text = read_file_as_string(&file_path);
            let total_occurrences = count_word_occurrences(&text, word.to_string());
            println!("Total occurrences of '{}': {}, elapsed time: {:?}", word, total_occurrences, start.elapsed());
        }
        _ => {
            eprintln!("Invalid option. Usage: rccwc -w <file_path> or -wo <word> <file_path>")
        }
    }
    
}

