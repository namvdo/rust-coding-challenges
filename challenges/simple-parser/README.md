# Problem Name: Simple Parser

**Author:** [Rudi Cilibrasi](https://github.com/rudi-cilibrasi)

## Relevant Background Knowledge

To effectively tackle this problem, it is important to have a basic understanding of the following concepts:

1. **Rust Syntax**: Familiarity with Rust's syntax, including functions, control flow, and data structures.
2. **Ownership and Borrowing**: Understanding Rust’s ownership model is crucial for memory safety and efficient resource management.
3. **Data Structures**: Knowledge of `BTreeMap` and `BTreeSet` for organizing unique words and their occurrences.
4. **String Manipulation**: Skills in manipulating strings and working with slices.
5. **File I/O**: Understanding how to read from files and handle command-line arguments.

## Main Functions

1. **`read_file(file_path: &str) -> Result<String, std::io::Error>`**  
   Reads the contents of a specified file and returns it as a `String`.

2. **`get_lines_lower_case(text: &str) -> Vec<String>`**  
   Converts the input text to lowercase and splits it into lines, returning a vector of strings.

3. **`process_words(content: &str) -> BTreeMap<String, BTreeSet<usize>>`**  
   Processes the content of the text, identifying unique words and mapping them to the line numbers where they appear.

4. **`read_file_from_args() -> Result<String, std::io::Error>`**  
   Reads the file input from command-line arguments.

## Example Output

Given the input text:
```
Apple banana cherry
Banana dog cat
Rust is a programming language
Hello world from Rust
The quick brown fox jumps
The lazy dog is cute
This is a test file
For testing random purposes
Banana and apple are fruits
Goodbye and hello
```
The output will be:
```
Word: 'a', Lines: {3, 7}
Word: 'and', Lines: {9, 10}
Word: 'apple', Lines: {1, 9}
Word: 'are', Lines: {9}
Word: 'banana', Lines: {1, 2, 9}
Word: 'brown', Lines: {5}
Word: 'cat', Lines: {2}
Word: 'cherry', Lines: {1}
Word: 'cute', Lines: {6}
Word: 'dog', Lines: {2, 6}
Word: 'file', Lines: {7}
Word: 'for', Lines: {8}
Word: 'fox', Lines: {5}
Word: 'from', Lines: {4}
Word: 'fruits', Lines: {9}
Word: 'goodbye', Lines: {10}
Word: 'hello', Lines: {4, 10}
Word: 'is', Lines: {3, 6, 7}
Word: 'jumps', Lines: {5}
Word: 'language', Lines: {3}
Word: 'lazy', Lines: {6}
Word: 'programming', Lines: {3}
Word: 'purposes', Lines: {8}
Word: 'quick', Lines: {5}
Word: 'random', Lines: {8}
Word: 'rust', Lines: {3, 4}
Word: 'test', Lines: {7}
Word: 'testing', Lines: {8}
Word: 'the', Lines: {5, 6}
Word: 'this', Lines: {7}
Word: 'world', Lines: {4}
```

## Usage Example

To run the parser on an input file, use the following command in your terminal:

```bash
cargo run -- ./input/small.txt
