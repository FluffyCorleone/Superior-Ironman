mod leetcode ;
mod codewars ;

use crate::leetcode::easy::{ factorial, hammingWeight } ;
use crate::codewars::{ unique_in_order, sum_dig_pow, tribonacci } ;

fn main() {    

    let a = [ 1f64, 1f64, 1f64 ] ;
    let b = tribonacci( &a, 10 ) ;
    
    println!( "{b:?}") ;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}