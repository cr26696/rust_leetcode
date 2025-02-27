
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_presum: Vec<i32> = vec![0;nums.len()];
    for (index,num) in nums.iter().enumerate(){
        if index == 0{
            max_presum[0] = *num;
        }else{
            max_presum[index] = (max_presum[index-1]+*num).max(*num);
        }
    }
    println!("{:?}",max_presum);
    let mut out = nums[0];
    // for i in [0..nums.len()]{

    // }
    out = *max_presum.iter().max().unwrap();
    return out;
}
fn main() {
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    let t = "aba".to_string();
    let r = max_sub_array(nums);
    println!("{:?}", r);
}
