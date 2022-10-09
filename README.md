The code I used to write blog post [Overthinking Leetcode's Two Sum with SIMD](https://darkcoding.net/software/two-sum/).

If you're trying these out in leetcode it looks like this:
```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (i, j) = two_sum_linear(target, &nums);
        vec![i as i32, j as i32]
    }
}

[paste func from blog post here]
```
