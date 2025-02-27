use core::num;


pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut l_mul:Vec<i32> = vec![0;nums.len()+2];//第 i 个元素前的积 前0个元素前没东西固定为1 
    let mut r_mul:Vec<i32> = vec![0;nums.len()+2];//第 i 个元素右侧积 不包括i
    let mut out:Vec<i32> = vec![0;nums.len()];
    l_mul[1] = 1;
    r_mul[nums.len()] = 1;
    for i in 1..nums.len()+1{
        if nums[i-1] == 0 { break;}
        l_mul[i+1] = l_mul[i] * nums[i-1];
    }
    for i in (1..nums.len()+1).rev(){
        if nums[i-1] == 0 { break;}
        r_mul[i-1] = r_mul[i] * nums[i-1];
    }

    for i in 0..nums.len() {
        out[i] = l_mul[i+1] * r_mul[i+1];
    }
    out
}
fn main() {
    let mut nums = vec![1,2,3,4];
    let r = product_except_self(nums);
    println!("{:?}", r);
}
