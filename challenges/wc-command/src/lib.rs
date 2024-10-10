use core::num;
use core::str;
use std::time::Instant;
use memmap2::MmapOptions;
use std::cmp::min;
use std::fs::File;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

pub fn read_file_as_string(file_path: &str) -> String {
    let file = File::open(file_path).expect("Path is not valid");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("Cannot map from file") };
    let text = unsafe { str::from_utf8_unchecked(&mmap).to_string() };
    text
}

pub fn count_words(text: &str) -> usize {
    count_in_paralell(text, count_words_in_chunk)
}

pub fn count_word_occurrences(text: &str, word: String) -> usize {
    count_in_paralell(text, move |chunk| {
        count_word_occurrences_in_chunk(chunk, &word)
    })
}

fn count_in_paralell<F>(text: &str, count_fn: F) -> usize
where
    F: Fn(&[&str]) -> usize + Send + Sync + 'static,
{
    let num_threads: usize = std::thread::available_parallelism().unwrap().get();
    let lines: Vec<&str> = text.lines().collect();
    let lines_per_thread = (lines.len() + num_threads - 1) / num_threads;
    let total_word_count = Arc::new(AtomicUsize::new(0));
    // let mut handles = Vec::new();
    let count_fn = Arc::new(count_fn);
    let start = Instant::now();
    // for i in 0..num_threads {
    //     let start = lines_per_thread * i;
    //     let end = min(start + lines_per_thread, lines.len());
    //     if is_valid_range(start, end, lines.len()) {
    //         let chunk = lines[start..end].to_vec();
    //         let count_fn_clone = Arc::clone(&count_fn);
    //         let total_word_count_clone = total_word_count.clone();

    //         let handle = thread::spawn(move || {
    //             let count = count_fn_clone(chunk);
    //             total_word_count_clone.fetch_add(count, Ordering::Relaxed);
    //         });
    //         handles.push(handle);
    //     }
    // }


    thread::scope(|s| {
        for i in 0..num_threads {
            let start = lines_per_thread * i;
            let end = min(start + lines_per_thread, lines.len());
            
            if is_valid_range(start, end, lines.len()) {
                let chunk = &lines[start..end]; // Borrow a slice of `&str`
                let count_fn_clone = Arc::clone(&count_fn);
                let total_word_count_clone = Arc::clone(&total_word_count);

                s.spawn(move || {
                    let count = count_fn_clone(chunk); // Use the borrowed slice here
                    total_word_count_clone.fetch_add(count, Ordering::Relaxed);
                });
            }
        }
    });

    println!("Finish loop in: {:?}", start.elapsed());

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    total_word_count.load(Ordering::Relaxed)
}

fn is_valid_range(start: usize, end: usize, len: usize) -> bool {
    start < end && end <= len 
}

fn chunk_text(text: &str, num_chunks: usize) -> Vec<&str> {
    let chunk_size = text.len() / num_chunks;
    let mut chunks = Vec::new();
    let mut start = 0;

    while start < text.len() {
        let end = min(text.len(), start + chunk_size);
        let chunk = &text[start..end];
        chunks.push(chunk);
        start = end;
    }
    chunks
}


fn collect_lines_parallel(text: &str, num_threads: usize) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut handles = Vec::new();
    
}

fn count_words_in_chunk(chunk: &[&str]) -> usize {
    chunk
        .iter()
        .flat_map(|line| line.split_whitespace())
        .count()
}

fn count_word_occurrences_in_chunk(chunk: &[&str], word: &str) -> usize {
    chunk
        .iter()
        .flat_map(|line| line.split_whitespace())
        .filter(|&w| w == word)
        .count()
}
