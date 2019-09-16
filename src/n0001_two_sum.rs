extern crate test;
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        let left = target - n;
        if let Some(j) = nums[i + 1..].iter().position(|&val| val == left){
            return vec![i as i32, (j + i + 1) as i32];
        }
    }
    vec![]
}

pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_dict = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        num_dict.insert(n, i);
    }
    for (i, n) in nums.iter().enumerate() {
        if let Some(&j) = num_dict.get(&(target - n)) {
            return vec![i as i32, j as i32];
        }
    }
    vec![]
}

pub fn two_sum_v3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return vec![j as i32, i as i32];
        }
        seen.insert(n, i);
    }
    vec![]
}

pub fn two_sum_v4(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            return vec![j as i32, i as i32];
        }
        seen.insert(n, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![2, 7, 11, 15], 18), vec![1, 2]);
        assert_eq!(two_sum_v2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_v2(vec![2, 7, 11, 15], 18), vec![1, 2]);
        assert_eq!(two_sum_v3(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_v3(vec![2, 7, 11, 15], 18), vec![1, 2]);
        assert_eq!(two_sum_v4(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_v4(vec![2, 7, 11, 15], 18), vec![1, 2]);
    }

    #[bench]
    fn bench_1(b: &mut Bencher) {
        b.iter(|| two_sum(vec![2, 7, 11, 15, 16, 17, 18, 19, 20], 27));
    }

    #[bench]
    fn bench_1_v2(b: &mut Bencher) {
        b.iter(|| two_sum_v2(vec![2, 7, 11, 15, 16, 17, 18, 19, 20], 27));
    }

    #[bench]
    fn bench_1_v3(b: &mut Bencher) {
        b.iter(|| two_sum_v3(vec![2, 7, 11, 15, 16, 17, 18, 19, 20], 27));
    }

    #[bench]
    fn bench_1_v4(b: &mut Bencher) {
        b.iter(|| two_sum_v4(vec![2, 7, 11, 15, 16, 17, 18, 19, 20], 27));
    }
}
