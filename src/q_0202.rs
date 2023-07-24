struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut values = Vec::new();
        let mut n = n;

        while !values.contains(&n) {
            values.push(n);
            n = n
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .map(|d| d * d)
                .sum();
        }

        n == 1
    }
}

#[test]
fn example_1() {
    assert!(Solution::is_happy(19));
}

#[test]
fn example_2() {
    assert!(!Solution::is_happy(2));
}
