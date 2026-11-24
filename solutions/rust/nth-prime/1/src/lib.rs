fn is_prime(n:u32) -> bool {
    if n < 2 {return false}
    if n == 2 {return true}
    if n%2 == 0 {return false;}
    for i in (3..n).step_by(2){
        if n%i == 0{
            return false;
        }
    }
    true

}

pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut i = 2;
    loop {
        if is_prime(i){
            if count == n {
                return  i;
            }
            count += 1
        }
        i += 1
    }
}
