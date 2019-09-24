pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let s = s.as_bytes();
        let mut max = &s[..1];
        let mut i = 1usize;
        while i < s.len() {
            let mut j = 0usize;
            while i + j <= s.len() - 1 && i >= j + 1 {
                // println!("right: {:?}, {}<-{}->{}",
                //     std::str::from_utf8(&s[i-j-1..=i+j]).unwrap().to_string(),
                //     s[i-j-1] as char, s[i] as char, s[i+j] as char);
                if s[i+j] == s[i-1-j] {
                    if max.len() < 2*j + 2 {
                        max = &s[i-1-j..=i+j];
                    }
                    j += 1;
                    continue;
                }
                break;
            }

            let mut j = 1usize;
            while i + j <= s.len() - 1 && i >= j {
                // println!("mid: {:?}, {}<-{}->{}",
                //     std::str::from_utf8(&s[i-j..=i+j]).unwrap().to_string(),
                //     s[i-j] as char, s[i] as char, s[i+j] as char);
                if s[i+j] == s[i-j] {
                    if max.len() < 2*j + 1 {
                        max = &s[i-j..=i+j];
                    }
                    j += 1;
                    continue;
                }
                break;
            }
            i += 1;
        }

        std::str::from_utf8(max).unwrap().to_owned()
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(longest_palindrome("bab".to_owned()), "bab".to_owned());
        assert_eq!(longest_palindrome("babad".to_owned()), "bab".to_owned());
    }
}