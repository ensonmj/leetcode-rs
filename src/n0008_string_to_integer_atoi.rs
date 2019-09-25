    pub fn my_atoi(str: String) -> i32 {
        let s = &str.trim_start();
        if s.len() == 0 {
            return 0;
        }
        let mut s = s.as_bytes();
        let mut sign:i32 = 1;
        let guard = i32::max_value() / 10;
        let mut guard_rem = i32::max_value() % 10;
        if s[0] == b'-' || s[0] == b'+' {
            if s[0] == b'-' {
                sign = -1;
                guard_rem += 1;
            }
            s = &s[1..];
            if s.len() == 0 {
                return 0;
            }
        }
        let mut res = 0i32;
        for b in s {
            match b {
                b'0'..=b'9' => {
                    if res > guard {
                        return if sign == 1 {i32::max_value()} else {i32::min_value()};
                    }
                    let rem = (b - b'0') as i32;
                    if res == guard && rem > guard_rem {
                        return if sign == 1 {i32::max_value()} else {i32::min_value()};
                    }
                    res = res * 10 + rem;
                }
                _ => break
            }
        }
        return sign * res;
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(my_atoi("xx".to_owned()), 0);
        assert_eq!(my_atoi("42".to_owned()), 42);
        assert_eq!(my_atoi("   -42".to_owned()), -42);
        assert_eq!(my_atoi("4193 with words".to_owned()), 4193);
        assert_eq!(my_atoi("words and 987".to_owned()), 0);
        assert_eq!(my_atoi("-91283472332".to_owned()), -2147483648);
        assert_eq!(my_atoi("+1".to_owned()), 1);
    }
}