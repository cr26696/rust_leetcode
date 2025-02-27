
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    let k = k as usize % n;
    nums.reverse();
    nums[..k].reverse();
    nums[k..].reverse();
}
fn main() {
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    rotate(&mut nums,k);
    println!("{:?}", nums);
}
