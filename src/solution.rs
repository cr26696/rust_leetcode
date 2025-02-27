pub mod rotate;
pub struct Solution;
impl Solution {
	pub fn rotate(nums: &mut Vec<i32>, k: i32) {
		rotate::rotate(nums,k);
	}
}
