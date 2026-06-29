use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut  res1= HashMap::new();
    let mut  res2= HashMap::new();
    for i in s1.chars(){
        *res1.entry(i).or_insert(0)+=1;
    }
    for i in s2.chars(){
        *res2.entry(i).or_insert(0)+=1;
    }
    res1==res2
}
