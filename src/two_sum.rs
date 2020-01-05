use std::collections::HashMap;

// O(n) Time and Space complexity
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut nums_map: HashMap<i32, i32> = HashMap::new();
  for (index, value) in nums.iter().enumerate() {
    let diff = target - value;
    match nums_map.get(value) {
      Some(val) => return vec![*val, index as i32],
      None => nums_map.insert(diff, index as i32),
    };
  }
  unreachable!();
}

#[test]
fn test_two_sum() {
  assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  assert_eq!(two_sum(vec![2, 7, 11, 15], 26), vec![2, 3]);
  assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}
