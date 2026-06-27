pub fn delete_and_backspace(s: &mut String) {
    let st=s.chars();
    let mut res= String::new();
    let mut count=0;
    for  x in st{
        if x=='-'{
            res.pop();
        }else if  x =='+'{
            count+=1;
        }else if count>0{
            count-=1;
        }else{
            res.push(x);
        }
    }
    *s=res
}

pub fn do_operations(v: &mut [String]) {
    for x in v{
        let mut res=0;
        if let Some(_)=x.find('+'){
            let parts: Vec<&str> = x.split('+').collect();

            let a: i32 = parts[0].parse().unwrap();
            let b: i32 = parts[1].parse().unwrap();
            res=a+b;

        } else if let Some(_)=x.find('-'){
            let parts: Vec<&str> = x.split('-').collect();

            let a: i32 = parts[0].parse().unwrap();
            let b: i32 = parts[1].parse().unwrap();

            res=a-b;
        }
        *x=res.to_string();
    }
}
