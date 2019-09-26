pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    if x < 10 {
        return true;
    }
    let mut nums = Vec::new();
    let mut rem = x % 10;
    nums.push(rem);
    let mut x = x / 10;
    while x > 0 {
        rem = x % 10;
        nums.push(rem);
        x = x / 10;
    }
    nums.iter().zip(nums.iter().rev()).all(|(x,y)| x==y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}