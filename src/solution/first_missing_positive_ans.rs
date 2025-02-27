pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
    let n = nums.len();

    // 原地哈希: 将正整数 x 放到索引 x-1 的位置上
    (0..n).for_each(|i| {
        let mut x = nums[i] as usize;
        while x >= 1 && x <= n && x as i32 != nums[x - 1] {
            nums.swap(i, x - 1);
            x = nums[i] as usize; // x 赋值为交换后的 i 位置的新值
        }
    });

    (0..n).position(|i| nums[i] != i as i32 + 1).unwrap_or(n) as i32 + 1
}
fn main() {
    let mut nums = vec![2,3,4];
    let r = first_missing_positive(nums);
    println!("{:?}", r);
}
