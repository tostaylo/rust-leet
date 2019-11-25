// mutate in place with O(1) space complexity
pub fn reverse_string(s: &mut Vec<char>) {
  let mut left_pointer = 0;
  let mut right_pointer = s.len() - 1;
  while left_pointer < right_pointer {
    let temp = s[left_pointer];
    s[left_pointer] = s[right_pointer];
    s[right_pointer] = temp;
    left_pointer = left_pointer + 1;
    right_pointer = right_pointer - 1;
  }
}
