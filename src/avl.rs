use core::option::Option::{None, Some};
use std::cell::RefCell;
use std::cmp::max;
use std::fs::OpenOptions;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub height: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type OptionTreeNodeRc = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            height: 0,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, node: TreeNode) {
        if self.val > node.val {
            match &self.left {
                Some(left) => left.borrow_mut().insert(node),
                None => self.left = Some(Rc::new(RefCell::new(node))),
            }
        } else if self.val < node.val {
            match &self.right {
                Some(right) => right.borrow_mut().insert(node),
                None => self.right = Some(Rc::new(RefCell::new(node))),
            }
        } else {
            println!("Inserted {:?}", node)
        }
    }

    pub fn height(node: OptionTreeNodeRc) -> i32 {
        match node {
            Some(x) => x.borrow().height,
            None => -1,
        }
    }

    pub fn update_height(node: OptionTreeNodeRc) {
        if let Some(node) = node {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().height = max(Self::height(left), Self::height(right)) + 1
        }
    }

    pub fn balance_factor(node: OptionTreeNodeRc) -> i32 {
        match node {
            Some(node) => {
                Self::height(node.borrow().left.clone()) - Self::height(node.borrow().right.clone())
            }
            None => 0,
        }
    }

    fn right_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            Some(node) => {
                let child = node.borrow().left.clone().unwrap();
                let grand_clild = child.borrow().right.clone();

                node.borrow_mut().left = grand_clild;
                child.borrow_mut().right = Some(node.clone());
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));

                Some(child)
            }
            Node => None,
        }
    }

    fn left_rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        match node {
            Some(node) => {
                let child = node.borrow().right.clone().unwrap();
                let grand_clild = child.borrow().left.clone();

                node.borrow_mut().right = grand_clild;
                child.borrow_mut().left = Some(node.clone());
                Self::update_height(Some(node));
                Self::update_height(Some(child.clone()));

                Some(child)
            }
            Node => None,
        }
    }

    pub fn rotate(node: OptionTreeNodeRc) -> OptionTreeNodeRc {
        let balance_factor = Self::balance_factor(node.clone());

        if balance_factor > 1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().left.clone()) >= 0 {
                Self::right_rotate(Some(node))
            } else {
                let left = node.borrow().left.clone();
                node.borrow_mut().left = Self::left_rotate(left);
                Self::right_rotate(Some(node))
            }
        } else if balance_factor < -1 {
            let node = node.unwrap();
            if Self::balance_factor(node.borrow().right.clone()) <= 0 {
                // 左旋
                Self::left_rotate(Some(node))
            } else {
                // 先右旋后左旋
                let right = node.borrow().right.clone();
                node.borrow_mut().right = Self::right_rotate(right);
                Self::left_rotate(Some(node))
            }
        } else {
            node
        }
    }
}