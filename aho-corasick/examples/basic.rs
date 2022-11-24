/*
    Example based on the the YouTube video:
    https://youtu.be/O7_w001f58c
*/

use aho_corasick::aho_corasick::AhoCorasick;

fn main() {
    let patterns: Vec<&str> = vec![
        "acc", "atc", "cat", "gcg",
    ];
    let text = "gcatcg";

    let mut ac = AhoCorasick::new(&patterns);
    ac.search_text(&text);

    println!("Search in \"{}\" the followings patterns {:?}", text, patterns);
    println!("Results: {:?}", ac.result.clone());
}