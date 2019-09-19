use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut start = 0;
    let mut end = 0;
    let mut max = 0;
    let mut seen = HashMap::new();
    for (i, b) in s.as_bytes().iter().enumerate() {
        match seen.get_mut(&b) {
            Some(j) => {
                // 旧数据，当没查到处理
                if *j < start {
                    end = i;
                    max = std::cmp::max(end - start + 1, max);
                    *j = i;
                    continue;
                }
                max = std::cmp::max(end - start + 1, max);
                start = *j + 1;
                *j = i;
                end = i;
            }
            None => { 
                end = i;
                max = std::cmp::max(end - start + 1, max);
                seen.insert(b, i);
            }
        }
    }
    max as i32
}

pub fn length_of_longest_substring_v2(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }
    let s = s.as_bytes();
    let mut max = 1usize;
    let mut start = 0usize;
    let mut end = 1usize;
    let mut sub = &s[start..end];
    for i in 1..s.len() {
        if let Some(j) = sub.iter().position(|&b| b == s[i]) {
            start = start + j + 1;
        }
        end = i + 1;
        max = std::cmp::max(end - start, max);
        sub = &s[start..end];
    }
    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_owned()), 3);
        assert_eq!(length_of_longest_substring(" ".to_owned()), 1);
        assert_eq!(length_of_longest_substring("".to_owned()), 0);
        assert_eq!(length_of_longest_substring("tmmzuxt".to_owned()), 5);

        assert_eq!(length_of_longest_substring_v2("abcabcbb".to_owned()), 3);
        assert_eq!(length_of_longest_substring_v2("bbbbb".to_owned()), 1);
        assert_eq!(length_of_longest_substring_v2("pwwkew".to_owned()), 3);
        assert_eq!(length_of_longest_substring_v2(" ".to_owned()), 1);
        assert_eq!(length_of_longest_substring_v2("".to_owned()), 0);
        assert_eq!(length_of_longest_substring_v2("tmmzuxt".to_owned()), 5);
    }
}