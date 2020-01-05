use std::collections::HashMap;

// O(n) time and space complexity
// Faster alternate. Sort and then dedup
// let len = nums.len();

//     nums.sort();
//     nums.dedup();

//     len != nums.len()
pub fn contains_duplicates(nums: Vec<i32>) -> bool {
  if nums.len() <= 1 {
    return false;
  }
  // Build hashtable and if hashtable already has value bail.
  let mut map = HashMap::new();
  for num in nums {
    if map.contains_key(&num) {
      return true;
    } else {
      map.insert(num.clone(), 1);
    }
  }
  false
}

#[test]
fn test_contains_duplicates() {
  assert_eq!(contains_duplicates(vec![1, 2, 3, 1]), true);
  assert_eq!(contains_duplicates(vec![1, 2, 3, 4]), false);
}
