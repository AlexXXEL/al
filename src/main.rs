mod avl;
use avl::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(2)));
    let left = TreeNode::new(1);
    let right = TreeNode::new(3);

    root.borrow_mut().insert(left);
    root.borrow_mut().insert(right);
    println!("Hello, world!{:#?}", root);
}
