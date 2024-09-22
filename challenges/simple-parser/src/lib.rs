use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::io;

pub fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn get_lines_lower_case(text: &str) -> Vec<String> {
    text.to_lowercase().lines().map(|line| line.to_string()).collect()
}


pub fn solve(content: &str) -> BTreeMap<String, BTreeSet<usize>> {
    let mut word_map: BTreeMap<String, BTreeSet<usize>> = BTreeMap::new();
    let lines = get_lines_lower_case(content);

    for(line_number, line) in lines.iter().enumerate() {
        let words: HashSet<&str> = line.split_whitespace().collect();
        for word in words {
            let word = word.to_string();
            word_map.entry(word).or_insert_with(BTreeSet::new).insert(line_number + 1);
        }
    }

    word_map
}