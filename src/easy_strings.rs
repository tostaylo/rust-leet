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

pub fn is_palindrome(s: String) -> bool {
  if s.is_empty() {
    return true;
  }
  let mut s_clone = s.clone();
  s_clone.retain(|c| c.is_alphanumeric());
  let s_as_chars: Vec<char> = s_clone.chars().collect();
  if s_as_chars.len() > 0 {
    let mut start = 0;
    let mut end = s_as_chars.len() - 1;

    while start < end {
      if s_as_chars[start].to_ascii_lowercase() == s_as_chars[end].to_ascii_lowercase() {
        start = start + 1;
        end = end - 1;
      } else {
        return false;
      }
    }
    return true;
  }
  true
}
