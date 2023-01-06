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