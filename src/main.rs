use std::{cell::RefCell, rc::Rc};

use avl::TreeNode;

mod avl;

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(2)));
 

    println!("Hello, world!{:#?}", root);
}
