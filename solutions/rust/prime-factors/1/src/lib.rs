pub fn factors(n: u64) -> Vec<u64> {
    let mut no = n;
    let mut i = 2 ;
    let mut vec:Vec<u64> = Vec::new();
    while no != 1 as u64{
        loop {
            if no == 1 {
                break;
            }
            if no%i == 0 {
                no = no / i;
                vec.push(i);
                continue;
            }
            i += 1;
            
        }
    }
    vec
}
