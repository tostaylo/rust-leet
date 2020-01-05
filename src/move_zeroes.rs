pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut index = 0;
  let mut count = 0;
  // Find the 0's in nums and remove them. Keeping the rest of the array in order.
  while index < nums.len() {
    if nums[index] == 0 {
      nums.remove(index);
      count = count + 1;
    } else {
      index = index + 1;
    }
  }
  // Add the number of 0's counted to the end of the num.
  for _index in 0..count {
    nums.push(0);
  }
}

#[test]
fn test_move_zeroes() {
  let mut tester_vec = vec![0, 1, 0, 3, 12];
  move_zeroes(&mut tester_vec);
  assert_eq!(tester_vec, vec![1, 3, 12, 0, 0]);
}
