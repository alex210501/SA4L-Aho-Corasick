use std::{
    cell::{RefCell}, 
    rc::Rc,
    collections::LinkedList,
};
use queues::*;

const DEBUG_FAILURE_LINKS: bool = false;
const DEBUG_DICTIONARY_LINKS: bool = false;
const ALPHABET_SIZE: u16 = 26;

fn index_from_char(character: char) -> u16 {
    (character as u16) - ('a' as u16)
}

pub struct Node {
    pub key: char,
    pub parent: Option<Rc<Node>>,
    children: RefCell<[Option<Rc<Node>>; ALPHABET_SIZE as usize]>,
    failure_link: RefCell<Option<Rc<Node>>>,
    dictionary_link: RefCell<Option<Rc<Node>>>,
    pub leaf: bool,
    pub size: usize,
}

impl Node {
    pub fn new() -> Node {
        Self { 
            key: ' ',
            parent: None,
            children: RefCell::new(Default::default()),
            failure_link: RefCell::new(None),
            dictionary_link: RefCell::new(None),
            leaf: false,
            size: 0,
        }
    }

    fn add_child(&self, key: char, parent: Rc<Node>, leaf: bool, size: usize) {
        match self.get_child(key) {
            Some(_) => {},
            None => {
                let index = index_from_char(key) as usize;

                self.children.borrow_mut()[index] = Some(
                    Rc::new(
                        Self {
                            key,
                            parent: Some(parent),
                            children: RefCell::new(Default::default()),
                            failure_link: RefCell::new(None),
                            dictionary_link: RefCell::new(None),
                            leaf,
                            size
                        }
                    )
                );
            }
        }
    }

    pub fn get_child(&self, key: char) -> Option<Rc<Node>> {
        let index = index_from_char(key) as usize;

        self.children.borrow()[index].clone()
    }

    pub fn contains_child(&self, key: char) -> bool {
        match self.get_child(key) {
            Some(_) => true,
            None => false
        }
    }

    pub fn get_failure_link(&self) -> Option<Rc<Node>> {
        self.failure_link.borrow().as_ref().cloned()
    }

    pub fn get_dictionary_link(&self) -> Option<Rc<Node>> {
        self.dictionary_link.borrow().as_ref().cloned()
    }

    fn get_linked_children(&self) -> Vec<Rc<Node>> {
        self.children
            .borrow().iter()
            .filter_map(|child| {
                if child.is_some() {
                    Some(child.as_ref().unwrap())
                } else {
                    None
                }
            })
            .map(|rc| rc.clone())
            .collect::<Vec<Rc<Node>>>()
    }
}

pub struct Automaton {
    pub root: Rc<Node>,
}

impl Automaton {
    pub fn new() -> Self {
        Self { root: Rc::new(Node::new()) }
    }

    pub fn from_patterns(patterns: &Vec<&str>) -> Self{
        let mut automaton: Automaton = Automaton::new();

        automaton.add_several_patterns(patterns);
        automaton.construct_failure_links();
        automaton.construct_dictionary_links();

        automaton
    }

    pub fn add_pattern(&mut self, word: &str) {
        let word_len = word.len();
        let mut current_node = self.root.clone();

        for (i, c) in word.chars().enumerate() {
            current_node.add_child(c, 
                current_node.clone(), 
                if i == (word_len - 1) { true } else { false }, 
                i + 1);

            current_node = match current_node.get_child(c) {
                Some(node) => node,
                None => panic!("Child does not exists !"),
            }
        }
    }
    
    pub fn add_several_patterns(&mut self, words: &Vec<&str>) {
        words.iter().for_each(|word| self.add_pattern(word));
    }

    pub fn is_pattern_present(&self, word: &String) -> bool {
        let word_len = word.len();
        let mut current_node = self.root.clone();

        for (i, c) in word.chars().enumerate() {
            match current_node.get_child(c) {
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

    fn calcul_suffix_link(&mut self, node: &mut Rc<Node>) {
        // If the node is the root, the failure link its itself
        if node.parent.is_none() && Rc::ptr_eq(&self.root, node) {
            node.failure_link.replace(Some(self.root.clone()));
            return;
        }

        // If the parent is root, define the failure link as root
        if Rc::ptr_eq(&node.parent.as_ref().unwrap().clone(), &self.root) {
            node.failure_link.replace(Some(self.root.clone()));
            return;
        }

        // Return if it is a direct child of the root, the failure link is the root
        if Rc::ptr_eq(&self.root, &(node.parent.as_ref().unwrap())) {
            return;
        }

        // Search failure link for each child
        let mut tmp_node = match node.parent.as_ref().unwrap().failure_link.borrow_mut().as_ref() {
            Some(link) => link.clone(),
            None => panic!("No failure link defined !")
        };

        while !Rc::ptr_eq(&self.root, &tmp_node) && !tmp_node.contains_child(node.key) {
            let failure_link = match tmp_node.failure_link.borrow_mut().as_ref() {
                Some(link) => link.clone(),
                None => panic!("No failure link defined !")
            };

            tmp_node = failure_link;
        }

        match tmp_node.get_child(node.key) {
            Some(c) => node.failure_link.replace(Some(c.clone())),
            None => node.failure_link.replace(Some(self.root.clone()))
        };
    }

    pub fn construct_failure_links(&mut self) {
        let mut current_node = self.root.clone();
        let mut queue: Queue<Rc<Node>> = Queue::new();

        // Add every root children in the queue
        current_node.get_linked_children().iter().for_each(|val| { let _ = queue.add(val.clone()); });

        // Loop on every node
        while queue.size() > 0 {
            current_node = queue.remove().unwrap();
            self.calcul_suffix_link(&mut current_node);
            

            if DEBUG_FAILURE_LINKS {
                let failure_link = current_node.failure_link.borrow().as_ref().unwrap().clone();
                println!("{} {} -> {} {}", current_node.key, current_node.size, failure_link.key, failure_link.size);
            }

            current_node.get_linked_children().iter().for_each(|val| { let _ = queue.add(val.clone()); });
        }
    }

    fn calcul_dictionary_links(&self, node: &mut Rc<Node>) {
        if node.dictionary_link.borrow().is_some() || 
            Rc::ptr_eq(&self.root, &node) || 
            Rc::ptr_eq(&self.root, &node.parent.as_ref().unwrap()){
            return;
        }

        // Check the failure link for the dictionary link
        match node.failure_link.borrow_mut().as_mut() {
            Some(link) => {
                if Rc::ptr_eq(&self.root, link) {
                    // node.dictionary_link.replace(None);
                } else if link.leaf {
                    node.dictionary_link.replace(Some(link.clone()));
                } else {
                    node.dictionary_link.replace(match link.dictionary_link.borrow().as_ref() {
                        Some(dict_link) => Some(dict_link.clone()),
                        None => {
                            self.calcul_dictionary_links(&mut link.clone());
                            link.dictionary_link.borrow().as_ref().cloned()
                        },
                    });
                }
            },
            None => panic!("No failure link defined for {} {}", node.key, node.size),
        }
    }

    pub fn construct_dictionary_links(&mut self) {
        let mut current_node = self.root.clone();
        let mut queue: Queue<Rc<Node>> = Queue::new();

        // Add every root children in the queue
        current_node.get_linked_children().iter().for_each(|val| { let _ = queue.add(val.clone()); });

        // Loop on every node
        while queue.size() > 0 {
            current_node = queue.remove().unwrap();
            self.calcul_dictionary_links(&mut current_node);

            if DEBUG_DICTIONARY_LINKS {
                match current_node.dictionary_link.borrow().as_ref() {
                    Some(link) => println!("{} {} -> {} {}", current_node.key, current_node.size, link.key, link.size),
                    None => {},
                };
            }

            current_node.get_linked_children().iter().for_each(|val| { let _ = queue.add(val.clone()); });
        }
    }

    pub fn print_nodes_dfs(&self) {
        let mut stack: LinkedList<Rc<Node>> = LinkedList::new();

        self.root.get_linked_children().iter().for_each(|val| { let _ = stack.push_back(val.clone()); });

        while !stack.is_empty() {
            let node = stack.pop_back().unwrap();

            println!("{} {}, (leaf: {})", node.key, node.size, node.leaf);
            node.get_linked_children().iter().for_each(|val| { let _ = stack.push_back(val.clone()); });
        }
    }
}
