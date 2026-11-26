use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32{
    let mut fac_vec = factors.to_vec();
    fac_vec.retain(|&x| x != 0);
    let mut vec:Vec<u32> = Vec::new();
    for (idx,i) in fac_vec.iter().enumerate(){
        let mut m = *i;
        while m < limit{
            vec.push(m);
            m = m + fac_vec[idx];
        }
    }
    let mut h_set:HashSet<u32> = HashSet::new();
    vec.retain(|x| h_set.insert(*x));
    let sum:u32 = vec.iter().sum();
    sum

}

