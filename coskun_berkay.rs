// Homework 3 - Berkay Coskun
use std::collections::HashMap;

// 1. Count words (uppercase and lowercase count as the same word)
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

// 2. Find how many times a word appears
fn count_of(counts: &HashMap<String, usize>, word: &str) -> usize {
    counts.get(&word.to_lowercase()).copied().unwrap_or(0)
}

// 3. Count all words
fn total_words(counts: &HashMap<String, usize>) -> usize {
    counts.values().sum()
}

// 4. Sort by count, then alphabetically if the counts are equal
fn sorted_words(counts: &HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut words: Vec<(String, usize)> = counts
        .iter()
        .map(|(word, count)| (word.clone(), *count))
        .collect();

    words.sort_by(|a, b| {
        if a.1 == b.1 {
            a.0.cmp(&b.0)
        } else {
            b.1.cmp(&a.1)
        }
    });

    words
}

fn main() {
    let text = "the cat The dog THE end";
    let counts = count_words(text);

    println!("Text: {}", text);
    println!("Count of the: {}", count_of(&counts, "the"));
    println!("Count of horse: {}", count_of(&counts, "horse"));
    println!("Total words: {}", total_words(&counts));
    println!("Sorted words: {:?}", sorted_words(&counts));

    // Check the examples from the homework
    assert_eq!(count_of(&counts, "the"), 3);
    assert_eq!(count_of(&counts, "horse"), 0);
    assert_eq!(total_words(&counts), 6);
    assert_eq!(sorted_words(&counts), vec![
        ("the".to_string(), 3),
        ("cat".to_string(), 1),
        ("dog".to_string(), 1),
        ("end".to_string(), 1),
    ]);

    let second = count_words("Rust is fun and Rust is fast");
    assert_eq!(total_words(&second), 7);
    assert_eq!(sorted_words(&second), vec![
        ("is".to_string(), 2),
        ("rust".to_string(), 2),
        ("and".to_string(), 1),
        ("fast".to_string(), 1),
        ("fun".to_string(), 1),
    ]);

    let empty = count_words("");
    assert!(empty.is_empty());
    assert_eq!(total_words(&empty), 0);
    assert!(sorted_words(&empty).is_empty());
}
