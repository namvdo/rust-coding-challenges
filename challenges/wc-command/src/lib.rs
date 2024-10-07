use core::num;
use std::sync::atomic::AtomicUsize;
use rand::Rng;
use rayon::{prelude::*, vec};
use rust_coding_challenges::utils::read_text_file_from_args;
use std::cmp::min;
use std::{fs::*, os};
use std::io::{BufRead, BufReader, Read};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;
use std::sync::atomic::Ordering;

pub fn solve(text: String) -> usize {
    let num_threads = std::thread::available_parallelism().unwrap().get();
    count_words_parallel(text, num_threads)
}

fn count_words_parallel(text: String, num_threads: usize) -> usize {
    let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();
    let lines_per_thread = (lines.len() + num_threads - 1) / num_threads;
    let total_word_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for i in 0..num_threads {
        let start = lines_per_thread * i;
        let end = min(start + lines_per_thread, lines.len());

        let total_word_count_clone = Arc::clone(&total_word_count);
        let chunk = lines[start..end].to_vec();

        let handle = thread::spawn(move || {
            let count = count_words_in_chunk(&chunk);
            total_word_count_clone.fetch_add(count, Ordering::Relaxed)
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    total_word_count.load(Ordering::Relaxed)
}

fn count_words_in_chunk(chunk: &[String]) -> usize {
    chunk
        .iter()
        .flat_map(|line| line.split_whitespace())
        .count()
}
