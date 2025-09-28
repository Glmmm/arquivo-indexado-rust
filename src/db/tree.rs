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
        let new_node = Box::new(Node::new(key, offset));
        self.root = self.insert_recursive(self.root.take(), new_node);
    }

    fn insert_recursive(&self, node: Option<Box<Node>>, new_node: Box<Node>) -> Option<Box<Node>> {
        match node {
            None => Some(new_node),
            Some(mut current_node) => {
                if new_node.key < current_node.key {
                    current_node.left = self.insert_recursive(current_node.left.take(), new_node);
                } else if new_node.key > current_node.key {
                    current_node.right = self.insert_recursive(current_node.right.take(), new_node);
                }
                Some(current_node)
            }
        }
    }

    pub fn search(&self, key: u32) -> Option<u64> {
        self.search_recursive(&self.root, key)
    }

    fn search_recursive(&self, node: &Option<Box<Node>>, key: u32) -> Option<u64> {
        match node {
            None => None,
            Some(current_node) => {
                if key == current_node.key {
                    Some(current_node.offset)
                } else if key < current_node.key {
                    self.search_recursive(&current_node.left, key)
                } else {
                    self.search_recursive(&current_node.right, key)
                }
            }
        }
    }

    pub fn delete(&mut self, key: u32) -> bool {
        let was_deleted = self.delete_recursive(&mut self.root, key);
        was_deleted.is_some()
    }

    fn delete_recursive(&mut self, node: &mut Option<Box<Node>>, key: u32) -> Option<Box<Node>> {
        if let Some(mut current_node) = node.take() {
            if key < current_node.key {
                current_node.left = self.delete_recursive(&mut current_node.left, key);
            } else if key > current_node.key {
                current_node.right = self.delete_recursive(&mut current_node.right, key);
            } else {
                if current_node.left.is_none() {
                    return current_node.right.take();
                } else if current_node.right.is_none() {
                    return current_node.left.take();
                }

                let successor = self.find_min_recursive(current_node.right.as_mut().unwrap());
                current_node.key = successor.key;
                current_node.offset = successor.offset;
                current_node.right = self.delete_recursive(&mut current_node.right, successor.key);
            }
            return Some(current_node);
        }
        None
    }

    fn find_min_recursive<'a>(&'a self, node: &'a mut Node) -> &'a mut Node {
        if node.left.is_none() {
            node
        } else {
            self.find_min_recursive(node.left.as_mut().unwrap())
        }
    }
}