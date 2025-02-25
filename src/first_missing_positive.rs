use std::collections::HashSet;

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut table = HashSet::new();
    for num in nums{
        if num>0{
            table.insert(num);
        }
    }
    let mut i = 1;
    loop {
        if table.contains(&i){
            i+=1;
        }else {
            return i
        }
    }
    return i;
}

fn main() {
    let mut nums = vec![2,3,4];
    let r = first_missing_positive(nums);
    println!("{:?}", r);
}