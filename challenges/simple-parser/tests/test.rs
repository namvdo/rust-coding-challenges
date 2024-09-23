use simple_parser::solve;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn test_solve_small_input() {
    let input = "Apple banana cherry\n\
                 Banana dog cat\n\
                 Rust is a programming language\n\
                 Hello world from Rust\n\
                 The quick brown fox jumps\n\
                 The lazy dog is cute\n\
                 This is a test file\n\
                 For testing random purposes\n\
                 Banana and apple are fruits\n\
                 Goodbye and hello";
    let expected = vec![
        ("apple", vec![1, 9]),
        ("banana", vec![1, 2, 9]),
        ("cherry", vec![1]),
        ("dog", vec![2, 6]),
        ("cat", vec![2]),
        ("rust", vec![3, 4]),
    ];

    let result = solve(input);

    for (word, expected_lines) in expected {
        let set: BTreeSet<usize> = expected_lines.into_iter().collect();
        assert_eq!(
            result.get(word),
            Some(&set),
            "Word: {} not found or incorrect",
            word
        );
    }
}

#[test]
fn test_solve_large_input() {
    let input = r#"Apple banana cherry
            Banana dog cat
            Rust is a programming language
            Hello world from Rust
            The quick brown fox jumps
            The lazy dog is cute
            This is a test file
            For testing random purposes
            Banana and apple are fruits
            Goodbye and hello
            Rust is a fast language
            Functional programming is fun
            Systems programming is powerful
            Rust ownership model is unique
            Memory safety in Rust is guaranteed
            Performance and safety are priorities in Rust
            Code that doesn’t crash, Rust’s goal
            Concurrent programming is easy in Rust
            Systems language that guarantees thread safety
            Borrow checker ensures safety
            Type system in Rust prevents null pointer dereferences
            Rust has no garbage collector
            Compile-time checks make Rust safe
            Rust's lifetimes prevent memory leaks
            Memory allocation in Rust is deterministic
            Ownership and borrowing make Rust efficient
            Rust’s syntax is modern and concise
            Pattern matching is powerful in Rust
            Generics in Rust allow code reusability
            Rust’s traits enable polymorphism
            Crates and modules organize Rust programs
            Tooling in Rust is modern and fast
            Rust’s standard library is powerful
            Testing in Rust is straightforward
            Cargo makes Rust development easy
            Rust has a growing ecosystem
            Zero-cost abstractions in Rust
            Compile-time errors prevent runtime crashes
            Rust’s error handling is robust
            Rust supports WebAssembly
            Rust’s community is friendly and helpful
            Developers love Rust’s performance guarantees
            Rust is loved for its memory safety
            Rust makes system-level programming safer
            Embedded systems in Rust are efficient
            Rust is increasingly adopted in the industry
            Cloud computing and Rust are a great fit
            Async programming in Rust is becoming popular
        "#;

    let mut expected_output = BTreeMap::new();
    expected_output.insert("apple", BTreeSet::from([1_usize, 9_usize]));
    expected_output.insert("banana", BTreeSet::from([1_usize, 2_usize, 9_usize]));
    expected_output.insert("cherry", BTreeSet::from([1_usize]));
    expected_output.insert("dog", BTreeSet::from([2_usize, 6_usize]));
    expected_output.insert("cat", BTreeSet::from([2_usize]));
    expected_output.insert(
        "rust",
        BTreeSet::from([
            3, 4, 11, 14, 15, 16, 18, 21, 22, 23, 25, 26, 28, 29, 31, 32, 34, 35, 36, 37, 40, 43, 44, 45, 46, 47, 48
        ]),
    );
    expected_output.insert(
        "is",
        BTreeSet::from([
            3, 6, 7, 11, 12, 13, 14, 15, 18, 25, 27, 28, 32, 33, 34, 39, 41, 43, 46, 48
        ]),
    );
    expected_output.insert("a", BTreeSet::from([3, 7, 11, 36, 47]));
    expected_output.insert("programming", BTreeSet::from([3, 12, 13, 18, 44, 48]));
    let result = solve(&input);
    for (word, lines) in expected_output {
        match result.get(word) {
            Some(actual_lines) => {
                assert_eq!(
                    actual_lines, &lines,
                    "Word '{}' does not match expected lines. Expected: {:?}, Actual: {:?}",
                    word, lines, actual_lines
                )
            }
            None => panic!("Word '{}' was not found in the result map", word),
        }
    }
}
