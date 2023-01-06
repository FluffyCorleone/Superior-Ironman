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