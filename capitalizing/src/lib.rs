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
    input.split_whitespace().map(capitalize_first).collect::<Vec<String>>().join(" ")
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
