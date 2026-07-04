pub fn score(input : &str)-> u64{
    let arr1 = ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T'];
    let arr2 = ['D', 'G'];
    let arr3 = ['B', 'C', 'M', 'P'];
    let arr4 = ['F', 'H', 'V', 'W', 'Y'];
    let arr5 = ['K'];
    let arr8 = ['J', 'X'];
    let arr = ['Q', 'Z'] ;

    let mut res:u64 = 0 ;

    for v in input.chars(){
        let upper_v = v.to_ascii_uppercase(); 

        if arr1.contains(&upper_v) { res += 1 }
        if arr2.contains(&upper_v) { res += 2 }
        if arr3.contains(&upper_v) { res += 3 }
        if arr4.contains(&upper_v) { res += 4 }
        if arr5.contains(&upper_v) { res += 5 }
        if arr8.contains(&upper_v) { res += 8 }
        if arr.contains(&upper_v) { res += 10 }
    }
    res

}