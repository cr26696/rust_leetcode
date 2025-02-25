
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    if k == 0 {return;}
    let len = nums.len();
    let j = k as usize % len;
    let copy = nums.clone();
    nums.clear();
    let (s1,s2) = copy.split_at(len-j);
    nums.extend_from_slice(s2);
    nums.extend_from_slice(s1);
}
fn main() {
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    rotate(&mut nums,k);
    println!("{:?}", nums);
}
