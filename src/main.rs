use std::{cell::RefCell, rc::Rc};

use avl::TreeNode;

mod avl;

fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root = TreeNode::insert(root.clone(), 4);
    root = TreeNode::insert(root.clone(), 6);
    root = TreeNode::insert(root.clone(), 8);



    println!("{:#?}", root)
}
