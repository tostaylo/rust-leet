use std::cmp;

use std::collections::HashMap;
// Don't need the second hash map here. Could loop through long and decrement short_counts by 1 everytime there is a match
// If short_counts[val] has a value greater than 0 we are good.
// Thinking this is worse than O(n) Time Complexity but need to verify.
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  // Determine the shortest and longest vector
  let (short, long) = match nums1.len() >= nums2.len() {
    true => (nums2, nums1),
    false => (nums1, nums2),
  };

  // Add the count of each item to a hashmap
  let mut long_counts = HashMap::new();
  for number in long {
    *long_counts.entry(number).or_insert(0) += 1;
  }
  let mut short_counts = HashMap::new();
  for number in &short {
    *short_counts.entry(number).or_insert(0) += 1;
  }

  let mut intersections = vec![];
  // Only loop through the shorter hashmap since it is more efficient
  // Push to the result the item which is in both hashmaps but has the smallest count.
  for (key, value) in short_counts {
    if long_counts.contains_key(&key) {
      let count = cmp::min(long_counts.get(&key).unwrap(), &value);
      for _index in 0..*count {
        intersections.push(*key);
      }
    }
  }
  intersections
}

#[test]
fn test_intersect() {
  assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), [2, 2]);
}
