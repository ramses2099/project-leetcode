fn main() {
    let nums = vec![2, 7, 11, 15];
    println!("Vec: {:?}", nums);
    let target = 9;
    println!("Target: {}", target);
    let result = leetcode_lib::Solution::two_sum_bruteforce(nums, target);
    assert_eq!(result, vec![0,1]);
    println!("Indices: {:?}", result);
}
