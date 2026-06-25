pub fn fibonacci(num: u32) -> u32 {
    if num==0{return 0}
    let mut res:u32=1;
    let mut x:u32= 0;
    for _ in 1..num{
        let y= res+x;
        x = res ;
        res =y;
    } 
    res
    
}
