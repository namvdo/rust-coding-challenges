use std::time::Instant;
use std::io::{BufRead, BufReader};
use std::fs::*;
use rand::Rng;
use rayon::prelude::*;

pub fn solve(file: File) -> usize {
    let reader = BufReader::new(file);
    let str = reader
        .lines()
        .filter_map(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");
    count_words(&str)
}

fn count_words(text: &str) -> usize {
    text.lines()
    .flat_map(|f| f.split_whitespace())
    .count()
}
