struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let a = nums[0] * nums[1] * nums.last().unwrap();
        let b = nums.pop().unwrap() * nums.pop().unwrap() * nums.pop().unwrap();
        a.max(b)
    }
}

#[test]
fn example_1() {
    let nums = vec![1, 2, 3];
    assert_eq!(Solution::maximum_product(nums), 6);
}

#[test]
fn example_2() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::maximum_product(nums), 24);
}

#[test]
fn example_3() {
    let nums = vec![-1, -2, -3];
    assert_eq!(Solution::maximum_product(nums), -6);
}
