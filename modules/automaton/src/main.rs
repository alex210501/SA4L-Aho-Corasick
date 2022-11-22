use std::{
    cell::{RefCell}, 
    rc::Rc
};

const ALPHABET_SIZE: u16 = 26;

fn index_from_char(character: char) -> u16 {
    (character as u16) - ('a' as u16)
}

struct Node {
    key: char,
    parent: Option<Rc<Node>>,
    children: RefCell<[Option<Rc<Node>>; ALPHABET_SIZE as usize]>,
    failure_link: Option<Rc<Node>>,
    leaf: bool,
}

impl Node {
    fn new() -> Node {
        Self { 
            key: ' ',
            parent: None,
            children: RefCell::new(Default::default()),
            failure_link: None,
            leaf: false 
        }
    }

    fn set_child(&self, key: char, parent: Rc<Node>, leaf: bool) {
        let index = index_from_char(key) as usize;

        self.children.borrow_mut()[index] = Some(
            Rc::new(
                Self {
                    key,
                    parent: Some(parent),
                    children: RefCell::new(Default::default()),
                    failure_link: None,
                    leaf
                }
            )
        );
    }

    fn get_children(&self, key: char) -> Option<Rc<Node>> {
        let index = index_from_char(key) as usize;

        self.children.borrow()[index].clone()
    }
}

struct Automaton {
    root: Rc<Node>,
}

impl Automaton {
    fn new() -> Self {
        Self { root: Rc::new(Node::new()) }
    }

    fn add_pattern(&mut self, word: &String) {
        let word_len = word.len();
        let mut current_node = self.root.clone();

        for (i, c) in word.chars().enumerate() {
            current_node.set_child(c, current_node.clone(), if i == (word_len - 1) { true } else { false });

            current_node = match current_node.get_children(c) {
                Some(node) => node,
                None => panic!("Child does not exists !"),
            }
        }
    }
    
    fn add_several_patterns(&mut self, words: &Vec<String>) {
        words.iter().for_each(|word| self.add_pattern(word));
    }

    fn is_pattern_present(&self, word: &String) -> bool {
        let word_len = word.len();
        let mut current_node = self.root.clone();

        for (i, c) in word.chars().enumerate() {
            match current_node.get_children(c) {
                Some(child) => {
                    if child.leaf && (i == word_len - 1) {
                        return true;
                    }

                    current_node = child.clone();
                }
                None => return false
            }
        }

        false
    }
}

fn main() {
    let mut automaton: Automaton = Automaton::new();
    let pattern: String = String::from("pat");

    automaton.add_several_patterns(&vec![String::from("yop"), String::from("pattern")]);

    println!("The pattern {} is present: {}", &pattern, automaton.is_pattern_present(&pattern));
}
