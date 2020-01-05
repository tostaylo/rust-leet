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

#[test]
fn test_reverse_string() {
  let mut hello: Vec<char> = "hello".chars().collect();
  let mut goodbye: Vec<char> = "goodbye".chars().collect();
  reverse_string(&mut hello);
  reverse_string(&mut goodbye);
  assert_eq!(hello, "olleh".chars().collect::<Vec<char>>());
  assert_eq!(goodbye, "eybdoog".chars().collect::<Vec<char>>());
}
