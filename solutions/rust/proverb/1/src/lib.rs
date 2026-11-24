pub fn build_proverb(list: &[&str]) -> String{
    // for (idx,&i) in list.iter().enumerate(){

    // }
    let mut i = 0;
    let mut vec:Vec<String> = Vec::new();
    loop {
        if i + 1 >= list.len(){
            break;
        }
        let s = format!("For want of a {} the {} was lost.",list[i] , list[i + 1]);
        vec.push(s);
        i += 1;
    }
    match list.get(0) {
        Some(n) => vec.push(format!("And all for the want of a {n}.")),
        None => ()
    }
    vec.join("\n")
    
}
