//Requirements: Modify in-place with 0(1) space complexity
pub fn remove_duplicates_sorted_array(nums: &mut Vec<i32>) -> i32 {
  if nums.len() > 0 {
    for index in 1..nums.len() {
      while index <= nums.len() - 1 && nums[index] == nums[index - 1] {
        nums.remove(index);
      }
    }
  }
  nums.len() as i32
}

#[test]
fn test_remove_duplicates_sorted_array() {
  assert_eq!(
    remove_duplicates_sorted_array(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
    5
  );

  assert_eq!(remove_duplicates_sorted_array(&mut vec![0, 0, 1]), 2);
  assert_eq!(remove_duplicates_sorted_array(&mut vec![1, 1]), 1);
}
