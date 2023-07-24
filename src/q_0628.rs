struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut low = [None; 2];
        let mut high = [None; 3];

        for num in nums {
            if low[0].is_none() || Some(num) < low[0] {
                low[1] = low[0];
                low[0] = Some(num);
            } else if low[1].is_none() || Some(num) < low[1] {
                low[1] = Some(num);
            }

            if high[2].is_none() || Some(num) > high[2] {
                high[0] = high[1];
                high[1] = high[2];
                high[2] = Some(num);
            } else if high[1].is_none() || Some(num) > high[1] {
                high[0] = high[1];
                high[1] = Some(num);
            } else if high[0].is_none() || Some(num) > high[0] {
                high[0] = Some(num);
            }
        }

        ([low[0], low[1], high[2]]
            .map(Option::unwrap)
            .iter()
            .product::<i32>())
        .max(high.map(Option::unwrap).iter().product())
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
