use std::cell::RefCell ;
use std::rc::Rc ;

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

pub fn factorial( n:u32 ) -> u32 {
    if n == 1 { 
        return 1 ; 
    } 

    n * factorial( n - 1 )
}

// https://leetcode.com/problems/number-of-1-bits/
pub fn hammingWeight (n: u32) -> i32 {

    format!( "{n:b}" )
        .chars()
        .fold( 0, |accum, value| {
            if value == '1' {
                accum + 1
            } else {
                accum
            }
        })    
}

// https://leetcode.com/problems/same-tree/
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {

    let a = inorder_traversal( p ) ;
    let b = inorder_traversal( q ) ;

    a.eq(&b)        
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    inorder(root.as_ref(), &mut result);
    result
}

fn inorder(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<Option<i32>>) {
    match node {
        Some(n) => {
            inorder(n.borrow().left.as_ref(), result);
            result.push( Some(n.borrow().val) );
            inorder(n.borrow().right.as_ref(), result);
        },
        None => result.push( None ),
    }    
}