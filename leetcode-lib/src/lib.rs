pub struct Solution;

impl Solution {
    /* #region two sum */   
    pub fn two_sum_bruteforce(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num) in nums.iter().enumerate() {
            // println!("index {}, {} value", i, num);
            for (j, num2) in nums
                .iter()
                .skip(i + 1)
                .enumerate() {
                // println!("index {}, {} value", j, num2);
                if num + num2 == target {
                    return vec![i as i32, (j + 1 + i) as i32];
                }
            }
        }

        vec![]
    }
   
    pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num) in nums.iter().enumerate() {
            // println!("index {}, {} value", i, num);
            for (j, num2) in nums
                .iter()
                .skip(i + 1)
                .enumerate() {
                // println!("index {}, {} value", j, num2);
                if num + num2 == target {
                    return vec![i as i32, (j + 1 + i) as i32];
                }
            }
        }

        vec![]
    }
    /* #endregion */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_bruteforce() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum_bruteforce(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
