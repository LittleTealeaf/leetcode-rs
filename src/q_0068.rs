struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut result = Vec::with_capacity(digits.len());
        let mut carry_over = false;

        if let Some(digit) = digits.pop() {
            if digit == 9 {
                result.push(0);
                carry_over = true;
            } else {
                result.push(digit + 1);
            }
        }

        while let Some(digit) = digits.pop() {
            if carry_over {
                if digit == 9 {
                    result.push(0);
                } else {
                    result.push(digit + 1);
                    carry_over = false;
                }
            } else {
                result.push(digit);
            }
        }

        if carry_over {
            result.push(1);
        }

        result.reverse();

        result
    }
}

#[test]
fn example_1() {
    let digits = vec![1, 2, 3];
    let solution = Solution::plus_one(digits);
    assert_eq!(solution, vec![1, 2, 4]);
}

#[test]
fn example_2() {
    let digits = vec![4, 3, 2, 1];
    let solution = Solution::plus_one(digits);
    assert_eq!(solution, vec![4, 3, 2, 2]);
}

#[test]
fn example_3() {
    let digits = vec![9];
    let solution = Solution::plus_one(digits);
    assert_eq!(solution, vec![1, 0]);
}
