use std::vec;

use aho_corasick::aho_corasick::AhoCorasick;

fn main() {
    let patterns: Vec<String> = vec![
        String::from("a"), 
        String::from("ag"), 
        String::from("c"), 
        String::from("caa"),
        String::from("gag"),
        String::from("gc"),
        String::from("gca")
    ];
    let mut ac = AhoCorasick::new(&patterns);

    ac.automaton.print_nodes_dfs();
    ac.search_text(&String::from("gcaa"));

    println!("Hello, world!");
}
