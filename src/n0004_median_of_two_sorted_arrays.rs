pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut s1 = &nums1[..];
    let mut s2 = &nums2[..];
    let (mut min, mut max) = (0, 0);
    while s1.len() > 0 && s2.len() > 0 {
        // println!("min: {:?} {:?}", s1, s2);
        if s1.first().unwrap() <= s2.first().unwrap() {
            min = *s1.first().unwrap();
            s1 = &s1[1..];
            if s1.len() == 0 {
                max = *s2.last().unwrap();
                s2 = &s2[0..s2.len()-1];
                break;
            }
        } else {
            min = *s2.first().unwrap();
            s2 = &s2[1..];
            if s2.len() == 0 {
                max = *s1.last().unwrap();
                s1 = &s1[0..s1.len()-1];
                break;
            }
        }

        // println!("max: {:?} {:?}", s1, s2);
        if s1.last().unwrap() >= s2.last().unwrap() {
            max = *s1.last().unwrap();
            s1 = &s1[0..s1.len()-1];
        } else {
            max = *s2.last().unwrap();
            s2 = &s2[0..s2.len()-1];
        }
    }
    if s1.len() == 0 && s2.len() == 0 {
        return (min + max) as f64 / 2.0;
    }
    let s = if s1.len() > 0 { s1 } else { s2 };
    let i = s.len()/2;
    if s.len() % 2 == 0 {
        return (s[i-1] + s[i]) as f64 / 2.0;
    } else {
        return s[i] as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1,3], vec![1,2,3]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1,2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![0,0], vec![0, 0]), 0.0);
        assert_eq!(find_median_sorted_arrays(vec![100001], vec![100000]), 100000.5);
    }
}