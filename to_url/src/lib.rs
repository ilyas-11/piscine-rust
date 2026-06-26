pub fn to_url(s: &str) -> String {
    let mut result=String::new();
    for c in s.chars(){
        if c ==' '{
            result.push_str("%20")
        }else{
            result.push(c)
        }

    }
    result
}
