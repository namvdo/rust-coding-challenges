use core::str;
use memmap2::MmapOptions;
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
    F: Fn(&str) -> usize + Send + Sync + 'static,
{
    let num_threads: usize = thread::available_parallelism().unwrap().get();
    let total_word_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let count_fn = Arc::new(count_fn);
    let chunks = get_chunks(text, num_threads);
    thread::scope(|scope| {
        for i in 0..chunks.len() {
            let total_word_count_clone = Arc::clone(&total_word_count); 
            let count_fn_clone = Arc::new(&count_fn); 
            let chunk = chunks[i];
            scope.spawn(move || {
                let count = count_fn_clone(chunk);
                total_word_count_clone.fetch_add(count, Ordering::Relaxed);
            });
        } 
    });
    total_word_count.load(Ordering::Relaxed)
}



fn get_chunks(text: &str, partitions: usize) -> Vec<&str> {
    let mut end_index = 0;
    let mut chunks = Vec::with_capacity(partitions);  

    for i in 0..partitions {
        let start_index = if i == 0 { 0 } else { end_index + 1 }; 
        end_index = get_end_index(text, i, start_index, partitions);
        if start_index < text.len() {
            let chunk = &text[start_index..end_index];
            chunks.push(chunk);
        }
    }
    chunks
}

fn get_end_index(text: &str, i: usize, start_index: usize, partitions: usize) -> usize {
    let chunk_size = text.len() / partitions;
    let bytes = text.as_bytes();
    let mut end_index = start_index + chunk_size;
    if end_index >= text.len() || i == partitions - 1 {
        return text.len();  
    }
    while end_index < text.len() && bytes[end_index] != b' ' {
        end_index += 1;
    }
    end_index
}


// fn count_words_in_chunk(chunk: &str) -> usize {
//     chunk
//     .lines()
//         .flat_map(|line| line.split_whitespace())
//         .count()
// }

fn count_words_in_chunk(chunk: &str) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for c in chunk.chars() {
        if c.is_whitespace() {
            in_word = false;
        } else {
            if !in_word {
                count += 1;
                in_word = true;
            }
        }
    }
    count
}



// fn count_word_occurrences_in_chunk(chunk: &str, word: &str) -> usize {
//     chunk
//         .lines()
//         .flat_map(|line| line.split_whitespace())
//         .filter(|&w| w == word)
//         .count()
// }

fn count_word_occurrences_in_chunk(chunk: &str, word: &str) -> usize {
    let mut count = 0;
    let word_len = word.len();
    let mut start = 0;
    let chunk_bytes = chunk.as_bytes();
    while let Some(pos) = chunk[start..].find(word) {
        let end = start + pos + word_len;
        if start + pos == 0 
            || chunk_bytes[start + pos - 1].is_ascii_whitespace() 
            && (end == chunk.len() || chunk_bytes[end].is_ascii_whitespace()) 
        {
            count += 1;
        }
        start += pos + word_len;
    }
    count
}
