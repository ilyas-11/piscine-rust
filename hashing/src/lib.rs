use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
let sum: i32 = list.iter().sum();

    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut arr = list.to_vec();
    arr.sort();
    if list.len()%2==1{
        return arr[list.len()/2]
    }
    (arr[list.len()/2-1]+arr[list.len()/2])/2

}

pub fn mode(list: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for &num in list {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut result = list[0];
    let mut max = 0;

    for (num, count) in map {
        if count > max {
            max = count;
            result = num;
        }
    }

    result

}
