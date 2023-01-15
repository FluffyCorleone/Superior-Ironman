pub fn find_factors( n:u128 ) -> Vec<u128> {
    
    let mut ret = vec![ 1, n ] ;

    for i in 2..n/2 {
        if n % i == 0 {
            ret.push( i ) ;
            ret.push( n/i ) ;
        }
    }

    ret.sort() ;
    ret.dedup() ;    
    ret
}

// 1440
pub fn factorize( n:u128 ) -> Vec<u128> {

    let primes = vec![ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41 ] ;
    let mut idx = 0 ;

    let mut ret = vec![ 1, n ] ;
    let mut prod = n ;    

    while prod != 1 {

        if prod % primes[ idx ] == 0 {
            prod = prod / primes[ idx ] ;
            ret.push( primes[ idx ] ) ;
            ret.push( prod ) ;
        } else {
            idx += 1 ;

            if idx == primes.len() {
                break ;
            }
        }
    }

    ret.sort() ;
    ret.dedup() ;
    ret
}