pub fn capitalize_first(input: &str) -> String {
    let mut ch =input.chars();
    match ch.next(){
        Some(f)=>{
            f.to_uppercase().collect::<String>()+ch.as_str()
        }
        None =>String::new()
    }
}

pub fn title_case(input: &str) -> String {
    let mut res= String::new();
    let mut upp= true;
    for c in input.chars(){
        if c.is_whitespace(){
            res.push(c);
            upp=true;

        }else if upp{
            res.push_str(&c.to_uppercase().to_string());
            upp=false;
        }else{
            res.push(c);
        }
    }
    res
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|c|{
        if c.is_uppercase(){
            c.to_lowercase().collect::<String>()
        }else{
            c.to_uppercase().collect::<String>()
        }
    }).collect()
}
