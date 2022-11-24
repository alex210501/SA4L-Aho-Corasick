/*
    Example with dictionary links based on the YouTube video:
    https://youtu.be/OFKxWFew_L0
 */

 use aho_corasick::aho_corasick::AhoCorasick;

fn main() {
    let patterns: Vec<String> = vec![
        String::from("a"),
        String::from("ag"),
        String::from("c"),
        String::from("caa"),
        String::from("gag"),
        String::from("gc"),
        String::from("gca"),
    ];
    let text = String::from("gcaa");

    let mut ac = AhoCorasick::new(&patterns);
    ac.search_text(&text);

    println!("Search in \"{}\" the followings patterns {:?}", text, patterns);
    println!("Results: {:?}", ac.result.clone());
}