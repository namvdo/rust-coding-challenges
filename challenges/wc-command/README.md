# wc-command
**Author:** [namvdo](https://github.com/namvdo)
## Overview

`wc-command` is a program that implements a simple command-line utility for counting words in a text file and the occurrences of a specific word. The program is designed to leverage parallel processing for improved performance, making it suitable for handling large text files efficiently.

## Features

- Count the total number of words in a text file.
- Count the occurrences of a specified word in a text file.

## Relevant Background Knowledge
To effectively tackle this problem, it is important to have a basic understanding of the following concepts:
1. **Rust Syntax**: Familiarity with Rust's syntax, including functions, control flow, and data structures.
2. **Ownership and Borrowing**: Understanding Rust’s ownership model is crucial for memory safety and efficient resource management.
3. **Parallel Processing**: Knowledge of threading in Rust to leverage concurrency for performance improvements.
4. **Memory Mapping**: Skills in using memory-mapped files for efficient file reading.
5. **Command-Line Arguments**: Understanding how to parse command-line arguments for user input.

## Main Functions

1. **`read_file_as_string(file_path: &str) -> String`**  
   Reads a file and returns its content as a `String`.

2. **`count_words(text: &str) -> usize`**  
   Counts the total number of words in the provided text.

3. **`count_word_occurrences(text: &str, word: String) -> usize`**  
   Counts the occurrences of a specific word in the provided text.

4. **`count_in_paralell<F>(text: &str, count_fn: F) -> usize`**  
   A generic function that processes text in parallel using the provided counting function.

## Usage Example
First, compile the program:
```bash
cd challenges/wc-command && cargo build
cargo run -- rccwc -w /path/to/the/input/file.txt
cargo run -- rccwc -wo rust /path/to/the/input/file.txt
```
Run test test:
```bash
cargo test
```

Blog post explaining the implementation: https://learntocodetogether.com/rcc-word-count-command/
