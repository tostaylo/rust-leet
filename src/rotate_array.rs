pub fn rotate_array(nums: &mut Vec<i32>, k: i32) {
  let len = nums.len() as i32;
  if len == k {
    return;
  }

  // Here I am just determining how many times to push the front number to the rear of the array.
  // The modulus will give the remainder of a division of k by len
  // So that can be plugged in place of k when the number to rotate is greater than the length of array
  let steps = match k > len {
    true => len - (k % len),
    false => len - k,
  };

  for _i in 0..steps {
    let value_to_move = nums.remove(0);
    nums.push(value_to_move)
  }
}

#[test]
fn test_rotate_array() {
  let mut test_collection = vec![1, 2, 3, 4, 5, 6, 7];
  rotate_array(&mut test_collection, 3);
  assert_eq!(test_collection, vec![5, 6, 7, 1, 2, 3, 4]);
  let mut test_collection2 = vec![1, 2, 3];
  rotate_array(&mut test_collection2, 4);
  assert_eq!(test_collection2, vec![3, 1, 2]);
}
