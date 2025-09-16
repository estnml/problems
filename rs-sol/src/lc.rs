use core::num;
use std::{cell::RefCell, env::current_exe, process::id, rc::Rc};

use crate::Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    /// 1.e Two Sum [n,n]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map =
            std::collections::HashMap::with_capacity(nums.len());
        let mut result_vec = Vec::with_capacity(2);

        for i in 0..nums.len() {
            if num_map.contains_key(&(target - nums[i])) {
                result_vec.push(i as i32);
                result_vec.push(num_map[&(target - nums[i])]);
                break;
            } else {
                num_map.insert(nums[i], i as i32);
            }
        }

        result_vec
    }

    /// 2.m Add Two Numbers [?,?]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, Some(node)) | (Some(node), None) => {
                Some(Box::new(ListNode {
                    val: node.val,
                    next: Solution::add_two_numbers(node.next, None),
                }))
            }
            (Some(l1_node), Some(l2_node)) => {
                let sum = l1_node.val + l2_node.val;

                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: if sum >= 10 {
                        let carry_node = Some(Box::new(ListNode::new(1)));
                        Solution::add_two_numbers(
                            Solution::add_two_numbers(
                                carry_node,
                                l1_node.next,
                            ),
                            l2_node.next,
                        )
                    } else {
                        Solution::add_two_numbers(
                            l1_node.next,
                            l2_node.next,
                        )
                    },
                }))
            }
        }
    }

    /// 3.m Longest Substring Without Repeating Characters
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut chars_in_window: std::collections::HashMap<char, usize> =
            std::collections::HashMap::with_capacity(s.len());

        // with these type of problems, the sliding window technique can be used.
        // sliding window is a contiguous slice/subarray in an array that meets the condition.
        // in this problem, the condition is: characters must not repeat in window.

        todo!()
    }

    /// 4.h Median of Two Sorted Arrays
    pub fn find_median_sorted_arrays(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    ) -> f64 {
        todo!()
    }

    /// 5. Longest Palindromic Substring
    pub fn longest_palindrome(s: String) -> String {
        todo!()
    }

    /// 6. Zigzag Conversion
    pub fn convert(s: String, num_rows: i32) -> String {
        todo!()
    }

    /// 7. Reverse Integer
    pub fn reverse(x: i32) -> i32 {
        todo!()
    }

    /// 8. String to Integer (atoi)
    pub fn my_atoi(s: String) -> i32 {
        todo!()
    }

    /// 9. Palindrome Number
    pub fn is_palindrome(x: i32) -> bool {
        todo!()
    }

    /// 10. Regular Expression Matching
    pub fn is_match(s: String, p: String) -> bool {
        todo!()
    }

    /// 94. Binary Tree Inorder Traversal
    pub fn inorder_traversal(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        todo!()
    }

    /// 1768.e Merge Strings Alternately [m+n,m+n]
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let total_len = word1.len() + word2.len();
        let mut result_str = String::with_capacity(total_len);

        let mut i = 0;

        let mut word1_ch_iter = word1.chars();
        let mut word2_ch_iter = word2.chars();

        // total_len = m+n hence O(m+n)
        while i < total_len {
            let maybe_word1_ch = word1_ch_iter.next();
            let maybe_word2_ch = word2_ch_iter.next();

            if maybe_word1_ch.is_some() {
                result_str.push(maybe_word1_ch.unwrap());
                i += 1;
            }

            if maybe_word2_ch.is_some() {
                result_str.push(maybe_word2_ch.unwrap());
                i += 1;
            }
        }

        result_str
    }

    /// 1071 Greatest Common Divisor of Strings
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        todo!()
    }

    /// 1431.e Kids With the Greatest Number of Candies [n,n]
    pub fn kids_with_candies(
        candies: Vec<i32>,
        extra_candies: i32,
    ) -> Vec<bool> {
        let max_candies = candies.iter().max();

        if max_candies.is_none() {
            return Vec::new();
        }

        candies
            .iter()
            .map(|c| *c + extra_candies >= max_candies.copied().unwrap())
            .collect()
    }

    /// 605. Can Place Flowers
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        todo!()
    }

    /// 345.e Reverse Vowels of a String [n,1]
    pub fn reverse_vowels(s: String) -> String {
        let vowels = std::collections::HashSet::from([
            'a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U',
        ]);

        let mut bytes = s.into_bytes();

        let mut left = 0;
        let mut right = bytes.len() - 1;

        loop {
            while left < right && !vowels.contains(&(bytes[left] as char)) {
                left += 1;
            }

            // ai
            while left < right && !vowels.contains(&(bytes[right] as char))
            {
                right -= 1;
            }

            if left >= right {
                break;
            }

            bytes.swap(left, right);
            left += 1;
            right -= 1;
        }

        String::from_utf8(bytes).unwrap()
    }

    /// 151. Reverse Words in a String
    pub fn reverse_words(s: String) -> String {
        todo!()
    }

    /// 238. Product of Array Except Self
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }

    /// 334. Increasing Triplet Subsequence
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        todo!()
    }

    /// 443. String Compression
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        todo!()
    }

    /// 283.e Move Zeroes [n,1]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut placement_idx = 0;
        let mut ptr = 0;

        while ptr < nums.len() {
            if nums[ptr] != 0 {
                nums.swap(ptr, placement_idx);
                placement_idx += 1;
            }

            ptr += 1;
        }
    }

    /// 392. Is Subsequence
    pub fn is_subsequence(s: String, t: String) -> bool {
        todo!()
    }

    /// 11. Container With Most Water
    pub fn max_area(height: Vec<i32>) -> i32 {
        todo!()
    }

    /// 1679. Max Number of K-Sum Pairs
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}
