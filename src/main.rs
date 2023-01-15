mod leetcode ;
mod codewars ;
mod projecteuler ;

use crate::leetcode::easy::{ factorial, hammingWeight, single_number, majority_element } ;
use crate::codewars::{ unique_in_order, sum_dig_pow, tribonacci, min_value, wave } ;
use crate::projecteuler::{ find_factors, factorize } ;

fn main() {

    let a = vec![ 4, 1, 2, 1, 2 ] ;    
    let b = vec![2,2,1] ;
    let c = vec![1] ;
    let d = vec![2,2,1,1,1,2,2] ;
    
    let e = majority_element( d ) ;
    
    println!( "e: {e}" ) ;        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}