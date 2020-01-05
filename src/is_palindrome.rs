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

#[test]
fn test_is_palindrome() {
  assert_eq!(
    is_palindrome("A man, a plan, a canal: Panama".to_owned()),
    true
  );
}
