use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

// O(n) time and space
// Could have used a stack or hashset to push/pop since every element appears exactly twice but one.
pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut map = HashMap::new();
  for num in nums {
    let val = match map.entry(num) {
      Occupied(entry) => entry.into_mut(),
      Vacant(entry) => entry.insert(0),
    };
    *val += 1;
  }
  for (key, value) in map {
    if value == 1 {
      return key;
    }
  }
  0
}

#[test]
fn test_single_number() {
  assert_eq!(single_number(vec![2, 2, 1]), 1);
  assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}
