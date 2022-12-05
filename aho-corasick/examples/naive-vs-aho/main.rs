use std::fs;
use std::time::Instant;

use aho_corasick::{
    aho_corasick::{AhoCorasick, MatchResult},
};


struct StringSearch<'a> {
    patterns: Vec<&'a str>,
    result: Vec<MatchResult>
}

impl<'a> StringSearch<'a> {
    fn from_patterns(patterns: &Vec<&'a str>) -> Self {
        Self {
            patterns: patterns.clone(),
            result: Vec::new(),
        }
    }

    fn search_text(&mut self, text: &str) {
        for i in 0..text.len() {
            self.check_patterns(i, text);
        }
    }

    fn check_patterns(&mut self, i: usize, text: &str) {
        for (pattern_index, &pattern) in self.patterns.iter().enumerate() {
            let mut current_index = i;

            for c in pattern.chars() {
                if c != text.chars().nth(current_index).unwrap_or('\n') {
                    break;
                } else {
                    current_index += 1;
                }
            }

            if (current_index - i) == (pattern.len()) {
                self.result.push(MatchResult { pattern: pattern_index as u16, start: i as u16, end: (current_index - 1) as u16});
            }
        }
    }
}

fn main () {
    let contents = fs::read_to_string("./examples/naive-vs-aho/input.txt")
        .expect("Should have been able to read the file");
    
    let patterns: Vec<&str> = vec![
        "acc", "tis", "abc",
    ];

    /* Naive */
    let now = Instant::now();
    let mut str_search = StringSearch::from_patterns(&patterns);

    str_search.search_text(&contents);
    let elapsed = now.elapsed();
    println!("Naive duration: {:.2?}", elapsed);

    /* Aho Corasick */
    let now = Instant::now();
    let mut ac = AhoCorasick::new(&patterns);

    ac.search_text(&contents);
    let elapsed = now.elapsed();
    println!("Aho-Corasick duration: {:.2?}", elapsed);
}