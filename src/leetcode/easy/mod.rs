use std::collections::HashMap ;
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

// https://leetcode.com/problems/single-number/
pub fn single_number(nums: Vec<i32>) -> i32 {

    match nums.len() {
        0 => 0,
        1 => nums[0],
        _ => {
            let mut bucket = HashMap::new() ;

            for num in nums {
                bucket.entry( num )
                    .and_modify( |count| *count += 1 )
                    .or_insert( 1 ) ;
            }
        
            for (key, value) in bucket.iter() {
                if *value == 1 {
                    return *key ;
                }
            }

            0
        },
    }    
}

pub fn majority_element(nums: Vec<i32>) -> i32 {

    match nums.len() {        
        1 => nums[0],
        e @ 2.. => {
            let mut bucket = HashMap::new() ;

            for num in nums {
                bucket.entry( num )
                    .and_modify( |count| *count += 1 )
                    .or_insert( 1 ) ;
            }

            let threshold = e / 2 ;
        
            for (key, value) in bucket.iter() {
                if *value > threshold {
                    return *key ;
                }
            }

            0
        },
        _ => 0
    }    
}
