mod leetcode ;
mod codewars ;

use crate::leetcode::easy::{ factorial, hammingWeight } ;
use crate::codewars::{ unique_in_order, sum_dig_pow } ;

fn main() {    

    let a = sum_dig_pow( 10, 200 ) ;  
    
    println!( "{a:?}") ;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}