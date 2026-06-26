pub fn initials(names: Vec<&str>) -> Vec<String> {
    
    let mut res= Vec::new();
    for name in names {
        let mut n=String::new();
        for s in name.split(' '){
           if let Some(c)= s.chars().next(){
               n.push(c);
               n.push('.');
               n.push(' ');
           }
        }
        n.pop();
        res.push(n);
    }
    res

}
