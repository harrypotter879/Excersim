/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let trim_no = code.replace(' ',"");
    for i in trim_no.chars(){
        if !i.is_ascii_digit(){
            return false;
        }
    }
    if trim_no.len() <= 1 {return false};
    let rev_no = trim_no.chars().rev().collect::<String>();
    let mut vec:Vec<u32> = Vec::new();
    for (idx,i) in rev_no.chars().enumerate(){
        let mut num = i.to_digit(10).unwrap_or(0);
        if idx %2 != 0{
            num = num * 2 ;
            if num > 9{
                num -= 9
            }
        }
        vec.push(num);
    }
    let sum: u32 = vec.iter().sum();

    sum % 10 == 0
}
