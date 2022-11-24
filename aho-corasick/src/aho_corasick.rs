use std::rc::Rc;

use crate::automaton::{
    Automaton, 
    Node,
};

pub struct AhoCorasick {
    pub automaton: Automaton,
    pub patterns: Vec<String>,
    node: Rc<Node>
}

impl AhoCorasick {
    pub fn new(patterns: &Vec<String>) -> Self {
        Self {
            automaton: Automaton::from_patterns(patterns),
            patterns: patterns.clone(),
            node: Rc::new(Node::new()),
        }
    }

    fn match_pattern(&mut self, index: usize, key: char) {
        // If the character is not found and you are at the root, return
        if !(self.node.contains_child(key)) && Rc::ptr_eq(&(self.automaton.root), &(self.node)) {
            return;
        }

        match self.node.get_child(key) {
            Some(child) => {
                if child.leaf {
                    println!("Pattern match at {} {}", (index + 1) as u16 - child.size as u16, index);
                }

                match child.get_dictionary_link() {
                    Some(dict_link) => {
                        if dict_link.leaf {
                            println!("Pattern match at {} {}", (index + 1) as u16 - dict_link.size as u16, index);
                        }
                    },
                    None => {}
                };

                self.node = child.clone();
            },
            None => {
                self.node = match self.node.get_failure_link() {
                    Some(failure_link) => {
                        failure_link.clone()
                    },
                    None => panic!("No failure link"),
                };

                self.match_pattern(index, key);
            },
        };
    }

    pub fn search_text(&mut self, text: &String) {
        self.node = self.automaton.root.clone();

        for (i, c) in text.chars().enumerate() {
            self.match_pattern(i, c);
        }
    }
}