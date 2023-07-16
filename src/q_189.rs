struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let number = nums.pop().unwrap();
            nums.insert(0, number);
        }
    }
}

#[test]
fn test_1() {
    let mut values = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut values, k);
    assert_eq!(values, vec![5, 6, 7, 1, 2, 3, 4]);
}

#[test]
fn test_2() {
    let mut values = vec![-1, -100, 3, 99];
    let k = 2;
    Solution::rotate(&mut values, k);
    assert_eq!(values, vec![3, 99, -1, -100]);
}

#[test]
fn test_3() {
    let mut values = vec![-1];
    let k = 2;
    Solution::rotate(&mut values, k);
    assert_eq!(values, vec![-1]);
}
