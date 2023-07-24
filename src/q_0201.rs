struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left & right;
        };
        let left = format!("{:b}", left)
            .chars()
            .filter_map(|c| c.to_digit(2))
            .collect::<Vec<_>>();

        let right = format!("{:b}", right)
            .chars()
            .filter_map(|c| c.to_digit(2))
            .collect::<Vec<_>>();

        let (offset_left, offset_right) = if left.len() > right.len() {
            (0, left.len() - right.len())
        } else {
            (right.len() - left.len(), 0)
        };

        let mut number = Vec::new();
        let mut stop = false;

        let len = left.len().max(right.len());

        for i in 0..len {
            let l = if i >= offset_left {
                left[i - offset_left]
            } else {
                0
            };

            let r = if i >= offset_right {
                right[i - offset_right]
            } else {
                0
            };

            if !stop {
                if l == r {
                    number.push(if l & r == 0 { "0" } else { "1" });
                } else {
                    stop = true;
                }
            }

            if stop {
                number.push("0");
            }
        }

        i32::from_str_radix(number.join("").as_str(), 2).unwrap()
    }
}

#[test]
fn example_1() {
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
}

#[test]
fn example_2() {
    assert_eq!(Solution::range_bitwise_and(0, 0), 0);
}

#[test]
fn example_3() {
    assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
}

#[test]
fn test_1() {
    assert_eq!(Solution::range_bitwise_and(2, 6), 0);
}
