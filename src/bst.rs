// Define a node in the BST
pub struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
}

// Define the BST itself
pub struct BST<K, V> {
    pub root: Option<Box<Node<K, V>>>,
}

impl<K: Ord + std::fmt::Debug, V: std::fmt::Debug> BST<K, V> {
    // Create a new, empty BST
    pub fn new() -> Self {
        BST { root: None }
    }

    // Insert a key-value pair into the BST
    pub fn insert(&mut self, key: K, value: V) {
        let new_node = Box::new(Node {
            key,
            value,
            left: None,
            right: None,
        });

        match self.root {
            Some(ref mut node) => Self::insert_node(node, new_node),
            None => self.root = Some(new_node),
        }
    }

    fn insert_node(current: &mut Box<Node<K, V>>, new_node: Box<Node<K, V>>) {
        if new_node.key < current.key {
            match current.left {
                Some(ref mut left_node) => Self::insert_node(left_node, new_node),
                None => current.left = Some(new_node),
            }
        } else if new_node.key > current.key {
            match current.right {
                Some(ref mut right_node) => Self::insert_node(right_node, new_node),
                None => current.right = Some(new_node),
            }
        }else{
            current.value=new_node.value
        }
    }

    // Get a value by key
    pub fn get(&self, key: &K) -> Option<&V> {
        Self::get_node(&self.root, key)
    }

    fn get_node<'a>(current: &'a Option<Box<Node<K, V>>>, key: &K) -> Option<&'a V> {
        match current {
            Some(node) => {
                if *key < node.key {
                    Self::get_node(&node.left, key)
                } else if *key > node.key {
                    Self::get_node(&node.right, key)
                } else {
                    Some(&node.value)
                }
            }
            None => None,
        }
    }

    // Update a value by key
    pub fn update(&mut self, key: K, value: V) {
        match self.root {
            Some(ref mut node) => Self::update_node(node, key, value),
            None => self.insert(key, value), // If the tree is empty, insert the new key-value pair
        }
    }

    fn update_node(current: &mut Box<Node<K, V>>, key: K, value: V) {
        if key < current.key {
            match current.left {
                Some(ref mut left_node) => Self::update_node(left_node, key, value),
                None => current.left = Some(Box::new(Node { key, value, left: None, right: None })),
            }
        } else if key > current.key {
            match current.right {
                Some(ref mut right_node) => Self::update_node(right_node, key, value),
                None => current.right = Some(Box::new(Node { key, value, left: None, right: None })),
            }
        } else {
            current.value = value;
        }
    }

    // Print all key-value pairs
    pub fn print_in_order(&self) {
        Self::print_node_in_order(&self.root);
    }

    fn print_node_in_order(node: &Option<Box<Node<K, V>>>) {
        if let Some(ref node) = node {
            Self::print_node_in_order(&node.left);
            println!("{:?}: {:?}", node.key, node.value);
            Self::print_node_in_order(&node.right);
        }
    }
}
