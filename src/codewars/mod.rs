// https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust
pub fn unique_in_order<T>( sequence: T ) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    
    let mut ret = Vec::new() ;        

    for itr in sequence {

        match ret.is_empty() {
            true => ret.push( itr ),
            false => {
                let last = ret.last() ;

                if itr != *(last.unwrap()) {
                    ret.push(itr) ;
                }
            },
        }
    }

    ret
}

// https://www.codewars.com/kata/5626b561280a42ecc50000d1/train/rust
pub fn sum_dig_pow( a:u64, b:u64 ) -> Vec<u64> {

    let digit_square_sum = |x:u64| {        

        x
            .to_string()
            .chars()
            .enumerate()
            .fold( 0u64, |accum,  value| {
                
                let a = value.1.to_digit( 10 ).unwrap() as u64;
                let b = (value.0+1).try_into().unwrap() ;
                
                a.pow( b ) + accum                
            })
    } ;

    (a..=b)
        .into_iter()
        .filter( |x| {
            *x == digit_square_sum(*x).try_into().unwrap() 
        })
        .collect()          
}

// https://www.codewars.com/kata/556deca17c58da83c00002db
pub fn tribonacci( signature: &[f64; 3], n: usize ) -> Vec<f64> {

    match n {
        e @ 0..=3 => Vec::from( &signature[ ..e ] ),        
        _ => {
            let mut ret = Vec::from( &signature[..] ) ;            

            for _ in 3..n {
                
                let s = ret.iter().rev().take(3).sum() ;                
                ret.push(s) ;
            }

            ret
        },
    }    
} 

// https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust
pub fn min_value(mut digits: Vec<i32>) -> i32 {

    digits.sort() ;
    digits.dedup() ;

    digits
        .iter()
        .map( |x| x.to_string() )
        .collect::<String>()
        .parse::<i32>()
        .unwrap()    
}

// https://www.codewars.com/kata/58f5c63f1e26ecda7e000029/train/rust
pub fn wave(s: &str) -> Vec<String> {

    let sz = s.len() ;    

    (0..sz)
        .into_iter()
        .map( |x| {
            
            s.char_indices()
                .filter( |y| y.1 != ' ' )
                .map( |y| {
                    if y.0 == x {
                        y.1.to_uppercase().to_string()                                            
                    } else {
                        y.1.to_string()
                    }
                })
                .collect()            
            
        } )
        .collect::<Vec<String>>()
}

// https://www.codewars.com/kata/587731fda577b3d1b0001196
pub fn camel_case( str:&str ) -> String {

    if str.is_empty() {
        return "".to_string() ;
    }

    str.split( ' ' )
        .into_iter()
        .filter( |x| !x.is_empty() )
        .map( |x| x[ 0..1 ].to_uppercase() + &x[ 1.. ].to_string() )
        .collect()
}