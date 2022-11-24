use std::rc::Rc;

use crate::automaton::{
    Automaton, 
    Node,
};

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub pattern: u16,
    pub start: u16,
    pub end: u16,
}

pub struct AhoCorasick<'a> {
    pub automaton: Automaton,
    pub patterns: Vec<&'a str>,
    pub result: Vec<MatchResult>,
    node: Rc<Node>
}

impl<'a> AhoCorasick<'a> {
    pub fn new(patterns: &Vec<&'a str>) -> Self {
        Self {
            automaton: Automaton::from_patterns(patterns),
            patterns: patterns.clone(),
            result: Vec::new(),
            node: Rc::new(Node::new()),
        }
    }

    fn add_result_from_last_pattern(&mut self, last_node: Rc<Node>, end_index: u16) {
        let mut pattern: String = String::new();
        let mut current_node = last_node.clone();

        loop {
            if Rc::ptr_eq(&current_node, &self.automaton.root) {
                break;
            }

            pattern.push(current_node.key);
            current_node = match current_node.parent.clone() {
                Some(parent) => parent.clone(),
                None => panic!("No parent defined for a non root node"),
            }
        }

        // Reverse the string because we look the tree in reverse order
        pattern = pattern.chars().rev().collect();
        match self.patterns.iter().position(|string| *string == pattern) {
            Some(pattern_number) => {
                self.result.push(MatchResult {
                    pattern: pattern_number as u16,
                    start: end_index + 1 - (last_node.size as u16),
                    end: end_index,
                });
            },
            None => panic!("No pattern {} found !", pattern),
        }

        println!("Pattern matched: {}", pattern);
    }

    fn match_pattern(&mut self, index: usize, key: char) {
        // If the character is not found and you are at the root, return
        if !(self.node.contains_child(key)) && Rc::ptr_eq(&(self.automaton.root), &(self.node)) {
            return;
        }

        match self.node.get_child(key) {
            Some(child) => {
                if child.leaf {
                    self.add_result_from_last_pattern(child.clone(), index as u16);
                }

                match child.get_dictionary_link() {
                    Some(dict_link) => {
                        if dict_link.leaf {
                            self.add_result_from_last_pattern(dict_link.clone(), index as u16);
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

    pub fn search_text(&mut self, text: &str) -> Vec<MatchResult> {
        self.node = self.automaton.root.clone();

        for (i, c) in text.chars().enumerate() {
            self.match_pattern(i, c);
        }

        self.result.clone()
    }
}