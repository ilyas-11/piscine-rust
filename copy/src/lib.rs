pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let c64=c as f64;
    (c,c64.exp(),(c.abs() as f64).ln(),)
}

pub fn str_function(a: String) -> (String, String) {
    let ori= a.clone();
    let  mut res = String::new();
     for x in a.split(' '){
        let n:f64 =x.parse().unwrap();
        res.push_str(&n.exp().to_string());
        res.push(' ');
    }
    res.pop();
    (ori, res)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let  mut res:Vec<f64>= Vec::new();
    for x in b.iter(){
        res.push((*x as f64).ln());
    }
    (b,res)
}
