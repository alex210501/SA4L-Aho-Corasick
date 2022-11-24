/*
    Example with dictionary links based on the YouTube video:
    https://youtu.be/OFKxWFew_L0
 */

 use aho_corasick::aho_corasick::AhoCorasick;

fn main() {
    let patterns: Vec<&str> = vec![
        "a", "ag", "c", "caa", "gag", "gc", "gca"
    ];
    let text = "gcaa";

    let mut ac = AhoCorasick::new(&patterns);
    ac.search_text(&text);

    println!("Search in \"{}\" the followings patterns {:?}", text, patterns);
    println!("Results: {:?}", ac.result.clone());
}