/*
    Example based on the the YouTube video:
    https://youtu.be/O7_w001f58c
*/

use aho_corasick::aho_corasick::AhoCorasick;

fn main() {
    let patterns: Vec<String> = vec![
        String::from("acc"),
        String::from("atc"),
        String::from("cat"),
        String::from("gcg"),
    ];
    let text = String::from("gcatcg");

    let mut ac = AhoCorasick::new(&patterns);
    ac.search_text(&text);

    println!("Search in \"{}\" the followings patterns {:?}", text, patterns);
    println!("Results: {:?}", ac.result.clone());
}