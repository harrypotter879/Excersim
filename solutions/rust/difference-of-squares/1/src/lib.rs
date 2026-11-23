pub fn square_of_sum(n: u32) -> u32 {
    let mut vec:Vec<u32> = vec![];
    for i in 1..=n{
        vec.push(i);
    }
    let sum: u32 = vec.iter().sum();
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut vec:Vec<u32> = vec![];
    for i in 1..=n{
        vec.push(i);
    }
    let sq_vec: u32 = vec.iter().map(|x| x.pow(2)).sum();
    sq_vec
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}