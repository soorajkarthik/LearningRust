use std::borrow::BorrowMut;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::panic::resume_unwind;

pub struct Solution {}

//********************************************* Two Sum ********************************************//
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(res) => return vec![*res, i as i32],
                None => { map.insert(target - *num, i as i32); }
            }
        }

        vec![-1, -1]
    }
}

//*************************************** Add Two LinkedLists ***************************************//
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                let (digit, carry) = if sum < 10 {
                    (sum, None)
                } else {
                    (sum % 10, Some(Box::new(ListNode::new(sum / 10))))
                };

                Some(Box::new(ListNode {
                    val: digit,
                    next: Solution::add_two_numbers(Solution::add_two_numbers(n1.next, carry), n2.next),
                }))
            }
        }
    }
}

//********************************** Longest Unique Substring ************************************//
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut start: usize = 0;
        let mut map: HashMap<char, usize> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                res = max(res, i - start);
                start = max(*map.get(&c).unwrap() + 1, start);
            }
            map.insert(c, i);
        }

        max(res, s.len() - start) as i32
    }
}

//******************************** Longest Palindrome Substring **********************************//
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return String::new();
        }

        let mut start = 0;
        let mut end = 0;

        for i in 1..s.len() - 1 {
            let len1 = Solution::expand_from_center(&s, i, i);
            let len2 = Solution::expand_from_center(&s, i, i + 1);
            let max = if len1.2 > len2.2 {
                len1
            } else {
                len2
            };

            println!("{:?}", max);

            if max.2 > end - start {
                start = max.0;
                end = max.1;
            }
        }

        String::from(&s[start..end + 1])
    }

    fn expand_from_center(s: &str, l: usize, r: usize) -> (usize, usize, usize) {
        let mut left = l;
        let mut right = r;

        while left >= 0 && right < s.len() && s[left..left + 1] == s[right..right + 1] {
            left -= 1;
            right += 1;
        }

        println!("{:?}", (left, right));
        (left + 1, right - 1, right - left - 1)
    }
}