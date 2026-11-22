pub fn is_armstrong_number(num: u32) ->bool {
    let a = num.to_string();
    let mut vec:Vec<u32> = Vec::new();
    for i in a.chars(){
        let no = i.to_digit(10).unwrap_or(0);
        let power_no = no.saturating_pow(a.len() as u32);
        vec.push(power_no);

    }
    let sum: u32= vec.iter().sum();
    if sum == num {
        return true
    }
    false
    
    
}