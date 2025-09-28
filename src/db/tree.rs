use std::option::Option;
use std::boxed::Box;

#[derive(Debug)]
pub struct Node {
    pub key: u32,
    pub offset: u64,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: u32, offset: u64) -> Self {
        Node {
            key,
            offset,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree {
    pub root: Option<Box<Node>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, key: u32, offset: u64) {
        Self::insert_recursive(&mut self.root, key, offset);
    }
    
    fn insert_recursive(node_opt: &mut Option<Box<Node>>, key: u32, offset: u64) {
        match node_opt {
            Some(node) => {
                if key < node.key {
                    Self::insert_recursive(&mut node.left, key, offset);
                } else if key > node.key {
                    Self::insert_recursive(&mut node.right, key, offset);
                }
            }
            None => {
                *node_opt = Some(Box::new(Node::new(key, offset)));
            }
        }
    }

    pub fn search(&self, key: u32) -> Option<u64> {
        Self::search_recursive(&self.root, key)
    }

    fn search_recursive(node_opt: &Option<Box<Node>>, key: u32) -> Option<u64> {
        match node_opt {
            Some(node) => {
                if key == node.key {
                    Some(node.offset)
                } else if key < node.key {
                    Self::search_recursive(&node.left, key)
                } else {
                    Self::search_recursive(&node.right, key)
                }
            }
            None => None,
        }
    }

    pub fn delete(&mut self, key: u32) -> bool {
        delete_recursive(&mut self.root, key).is_some()
    }
}

fn delete_recursive(node: &mut Option<Box<Node>>, key: u32) -> Option<Box<Node>> {
    if let Some(mut current_node) = node.take() {
        if key < current_node.key {
            current_node.left = delete_recursive(&mut current_node.left, key);
        } else if key > current_node.key {
            current_node.right = delete_recursive(&mut current_node.right, key);
        } else {
            if current_node.left.is_none() {
                return current_node.right.take();
            } else if current_node.right.is_none() {
                return current_node.left.take();
            }

            let successor = find_min_and_remove(&mut current_node.right);
            if let Some(s) = successor {
                current_node.key = s.key;
                current_node.offset = s.offset;
            }
        }
        return Some(current_node);
    }
    None
}

fn find_min_and_remove(node_opt: &mut Option<Box<Node>>) -> Option<Box<Node>> {
    let mut current = node_opt.as_mut().unwrap();
    
    if current.left.is_none() {
        return node_opt.take();
    }
    
    while current.left.as_ref().unwrap().left.is_some() {
        current = current.left.as_mut().unwrap();
    }
    
    let mut successor = current.left.take().unwrap();
    
    current.left = successor.right.take();
    
    Some(successor)
}