///  str: "acdac",  pattern: "a.*dac"
///  p   a . * d a c
///    --------------
/// s |1|0| | | | | |
///   |-------------|
///  a|0| | | | | | |
///   |-------------|
///  c|0| | | | | | |
///   |-------------|
///  d|0| | | | | | |
///   |-------------|
///  a|0| | | | | | |
///   |-------------|
///  c|0| | | | | | |
///    --------------
pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let sn = s.len();
    let pn = p.len();
    let mut dp = vec![vec![false; pn+1]; sn+1];
    // let mut dp = vec![false; (sn+1)*(pn+1)];
    // let mut dp: Vec<_> = dp.as_mut_slice().chunks_mut(pn+1).collect();
    // let dp: &mut [&mut [_]] = dp.as_mut_slice();

    // dp[i][j] describe whether s[0,i-1] match p[0,j-1]
    // empty str matches empty pattern
    dp[0][0] = true;
    // dp[i][j] = dp[i-1][j-1], if (s[i-1] == p[j-1] || p[j-1] == '.')
    //            || dp[i][j-2], if p[j-1] == '*'
    //            || dp[i-1][j], if ((s[i-1] == p[j-2] || p[j-2] == '.') && p[j-1] == '*')
    for i in 0..=sn {
        for j in 1..=pn {
            if j >= 2 && p[j-1] == b'*' {
                dp[i][j] = dp[i][j-2] || (i>0 && (s[i-1]==p[j-2] || p[j-2]==b'.') && dp[i-1][j]);
            } else {
                dp[i][j] = i >= 1 && dp[i-1][j-1] && (s[i-1]==p[j-1] || p[j-1]==b'.')
            }
        }
    }
    // println!("{:?}", dp);
    dp[sn][pn]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert_eq!(is_match("".to_owned(), ".*".to_owned()), true);
        assert_eq!(is_match("a".to_owned(), ".".to_owned()), true);
        assert_eq!(is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(is_match("ab".to_owned(), ".*".to_owned()), true);
        assert_eq!(is_match("aab".to_owned(), "c*a*b".to_owned()), true);
        assert_eq!(is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()), false);
    }
}