extern crate test;

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows <= 1 {
        return s;
    }
    let s = s.as_bytes();
    let mut output: Vec<u8> = Vec::new();
    let offset = (2 * num_rows - 2) as usize;
    for level in 0..num_rows {
        let mut i = level as usize;
        while i < s.len() {
            output.push(s[i]);
            // println!("{}", std::str::from_utf8(&output).unwrap().to_owned());
            if level != 0 && level != num_rows - 1 {
                let mid_offset = 2*(num_rows-level-1) as usize;
                let j = i + mid_offset;
                if j < s.len() {
                    output.push(s[j]);
                    // println!("{}", std::str::from_utf8(&output).unwrap().to_owned());
                }
            }
            i += offset;
        }
    }
    std::str::from_utf8(&output).unwrap().to_owned()
}

pub fn convert_v2(s: String, num_rows: i32) -> String {
    let mut chars: Vec<_> = (0..num_rows)
        .chain((1..num_rows - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();
    chars.sort_by_key(|&(row, _)| row);
    chars.iter().map(|(_, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_6() {
        assert_eq!(convert("PAYPALISHIRING".to_owned(), 3), "PAHNAPLSIIGYIR".to_owned());
        assert_eq!(convert("PAYPALISHIRING".to_owned(), 4), "PINALSIGYAHRPI".to_owned());
        assert_eq!(convert("A".to_owned(), 1), "A".to_owned());
        assert_eq!(convert_v2("PAYPALISHIRING".to_owned(), 3), "PAHNAPLSIIGYIR".to_owned());
        assert_eq!(convert_v2("PAYPALISHIRING".to_owned(), 4), "PINALSIGYAHRPI".to_owned());
        assert_eq!(convert_v2("A".to_owned(), 1), "A".to_owned());
    }

    #[bench]
    fn bench_6(b: &mut Bencher) {
        b.iter(||convert("PAYPALISHIRING".to_owned(), 4))
    }

    #[bench]
    fn bench_6_v2(b: &mut Bencher) {
        b.iter(||convert_v2("PAYPALISHIRING".to_owned(), 4))
    }
}