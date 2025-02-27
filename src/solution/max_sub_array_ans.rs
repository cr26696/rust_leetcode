
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold((0, i32::MIN), |(cur, ans), &val| {
            (0.max(cur + val), ans.max(cur + val))
        })
        .1
}
fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    let t = "aba".to_string();
    let r = max_sub_array(nums);
    println!("{:?}", r);
}
