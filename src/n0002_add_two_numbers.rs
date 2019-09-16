extern crate test;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut dummy_head;
    let (mut l1, mut l2) = (l1, l2);
    let (mut l1_end, mut l2_end, mut overflow) = (false, false, false);
    loop {
        let n1 = match l1 {
            Some(node) => {l1 = node.next; node.val},
            None => {l1_end = true; 0},
        };
        let n2 = match l2 {
            Some(node) => {l2 = node.next; node.val},
            None => {l2_end = true; 0},
        };
        if l1_end && l2_end && !overflow {
            return dummy_head.unwrap().next;
        }
        let sum = n1 + n2 + if overflow {1} else {0};
        let sum = if sum >= 10 {overflow=true; sum -10} else {overflow=false; sum};
        tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
        tail = &mut tail.as_mut().unwrap().next;
    }
}

pub fn add_two_numbers_v2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn _add_two_numbers(carry: i32, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val + carry;
                Some(Box::new(ListNode{val: sum % 10, next: _add_two_numbers(sum/10, n1.next, n2.next)}))
            }
            (Some(n), None) | (None, Some(n)) => {
                let sum = n.val + carry;
                Some(Box::new(ListNode{val: sum % 10, next: _add_two_numbers(sum/10, n.next, None)}))
            }
            _ if carry > 0 => {
                Some(Box::new(ListNode::new(carry)))
            }
            _ => None
        }
    }
    _add_two_numbers(0, l1, l2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    macro_rules! list {
        ($($n:expr)=>*) => {
            {
                let mut dummy_head = Some(Box::new(ListNode::new(0)));
                let mut tail = &mut dummy_head;
                $(
                    tail.as_mut().unwrap().next = Some(Box::new(ListNode::new($n)));
                    tail = &mut tail.as_mut().unwrap().next;
                )*
                dummy_head.unwrap().next
            }
        };
    }

    #[test]
    fn test_2() {
        assert_eq!(add_two_numbers(list!{2=>4=>3}, list!{5=>6=>4}), list!{7=>0=>8});
        assert_eq!(add_two_numbers_v2(list!{2=>4=>3}, list!{5=>6=>4}), list!{7=>0=>8});
    }

    #[bench]
    fn bench_2(b: &mut Bencher) {
        b.iter(|| add_two_numbers(list!{2=>4=>3}, list!{5=>6=>4}));
    }

    #[bench]
    fn bench_2_v2(b: &mut Bencher) {
        b.iter(|| add_two_numbers_v2(list!{2=>4=>3}, list!{5=>6=>4}));
    }
}