use std::vec;

use aho_corasick::aho_corasick::AhoCorasick;

fn main() {
    let patterns: Vec<&str> = vec![
        "a", 
        "ag",
        "c",
        "caa",
        "gag",
        "gc",
        "gca"
    ];
    let mut ac = AhoCorasick::new(&patterns);

    ac.automaton.print_nodes_dfs();
    ac.search_text(&String::from("gcaa"));

    println!("Hello, world!");
}
