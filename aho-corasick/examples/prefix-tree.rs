use aho_corasick::prefix_tree::TrieNode;

fn main() {
    let mut first_node: TrieNode = TrieNode::new();

    TrieNode::insert_severals(&mut first_node, &vec![String::from("yop"), String::from("burger")]);
    println!("{}", TrieNode::search(&first_node, &String::from("burger")));
}