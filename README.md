# Project Leetcode
Leetcode tutorial in rust

## Extension #region folding for VS Code

```
The extension also installs a command to wrap a region comment around the current selection.

    regionfolder.wrapWithRegion (Ctrl+M Ctrl+R)
```
## Example #region
```
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
    
    /* #endregion */
```
