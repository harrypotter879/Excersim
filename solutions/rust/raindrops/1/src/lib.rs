pub fn raindrops(n: u32) -> String {
    let mut s:Vec<String> = Vec::new();
    if n%3 == 0{
        s.push("Pling".to_string())
    }
    if n%5 == 0 {
        s.push("Plang".to_string())
    }
    if n%7 == 0 {
        s.push("Plong".to_string())
    }
    if s.is_empty(){
        return format!("{n}")
    }
    
    s.join("")
}
