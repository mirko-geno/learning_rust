use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;


#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>, // Weak can receive None type
    pub children: Option<RefCell<HashMap<String, Rc<Node>>>>
}

impl Node {
    pub fn add_parent(&mut self, parent: RefCell<Weak<Node>>) {
        self.parent = parent;
    }

    pub fn add_child(&mut self, name: String, child:Rc<Node>) {
        match self.children.as_mut() {
            Some(child_map) => {
                child_map.borrow_mut().insert(name, child);
            },
            None => {self.children = Some(RefCell::new(HashMap::from([(name, child)])));}
        }
    }

    pub fn add_children(&mut self, children: Vec<(String, Rc<Node>)>) {
        match self.children.as_mut() {
            Some(child_map) => {
                children.into_iter().for_each(|(name, node)| {
                    child_map.borrow_mut().insert(name, node);
                });
            },
            None => {self.children = Some(RefCell::new(HashMap::from_iter(children)));}
        }
    }
}





/*

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: Option<Weak<Node>>,
    pub children: Option<HashMap<String, Rc<Node>>>
}

impl Node {
    pub fn new(value: i32, parent: Option<Weak<Node>>, children: Option<(String, Rc<Node>)>) -> Node {
        Node {
            value,
            parent,
            children: match children {
                Some((string, node)) => Some(HashMap::from([(
                    string,
                    node
                )])),
                None => None
            }
        }
    }
}
*/