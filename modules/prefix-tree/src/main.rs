const ALPHABET_SIZE: u16 = 26;

fn index_from_char(character: char) -> u16 {
    character as u16 - 'a' as u16
}

#[derive(Debug)]
struct TrieNode{
    children: Box<[Option<TrieNode>; ALPHABET_SIZE as usize]>,
    is_last_character: bool,
}

impl TrieNode {
    fn new() -> TrieNode{
        TrieNode { 
            children: Default::default(), 
            is_last_character: false 
        }
    }

    fn insert(first_node: &mut TrieNode, word: &String) {
        let word_len = word.len();
        let mut current_node = first_node;

        for (i, c) in word.chars().enumerate() {
            let index: usize = index_from_char(c) as usize;

            current_node.children[index] = Some(
                TrieNode {
                    children: Default::default(),
                    is_last_character: if i == (word_len - 1) { true } else { false }
                }
            );

            current_node = current_node.children[index].as_mut().unwrap();
        }
    }

    fn insert_severals(first_node: &mut TrieNode, words: &Vec<String>) {
        words.iter().for_each(|word| {
            Self::insert(first_node, word);
        });
    }

    fn search(first_node: &TrieNode, word: &String) -> bool {
        let mut index: usize = 0;
        let mut current_node = first_node;

        for (i, c) in word.chars().enumerate() {
            index = index_from_char(c) as usize;
            let index_node = current_node.children[index].as_ref();

            match index_node {
                Some(node) => { 
                    if index_node.unwrap().is_last_character && i == word.len() - 1 {
                        return true
                    }

                    current_node = node ;
                },
                None => return false,
            }
        }
        
        index == word.len() - 1
    }
}

fn main() {
    let mut first_node: TrieNode = TrieNode::new();

    TrieNode::insert_severals(&mut first_node, &vec![String::from("yop"), String::from("burger")]);
    println!("{}", TrieNode::search(&first_node, &String::from("urger")));
}
