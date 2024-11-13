use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use node_struct::Node;

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: None,
    });
    println!("leaf parent: {:?}", leaf.parent);

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: Some(RefCell::new(HashMap::from([(
            String::from("First"),
            Rc::clone(&leaf)
        )]))),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


    let leaf2 = Rc::new(Node {
        value: 25,
        parent: RefCell::new(Weak::new()),
        children: None,
    });

    let leaf3 = Rc::new(Node {
        value: 14,
        parent: RefCell::new(Weak::new()),
        children: None,
    });


    branch.add_child(String::from("Second"), Rc::clone(&leaf2));


}
