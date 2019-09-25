    pub fn reverse(x: i32) -> i32 {
        if x == i32::min_value() {
            return 0;
        }

        let mut res: i32 = 0;

        let mut y = x.abs();
        let guard = i32::max_value() / 10;
        let mut guard_rem = i32::max_value() % 10;
        if x < 0 {
            guard_rem -= 1;
        }
        while y != 0 {
            if res > guard {
                return 0;
            }
            let rem = y % 10;
            if res == guard && rem > guard_rem {
                return 0;
            }
            res = res * 10 + rem;
            y = y / 10;
        }

        if x > 0 {
            res
        } else {
            -res
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(i32::max_value()), 0);
        assert_eq!(reverse(-2147483648), 0);
    }
}