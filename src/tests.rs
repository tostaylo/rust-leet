// Note this useful idiom: importing names from outer (for mod tests) scope.
//  use super::*;

// Easy Strings

#[test]
fn test_is_palindrome() {
  #[cfg(test)]
  use crate::easy_strings::is_palindrome;

  assert_eq!(
    is_palindrome("A man, a plan, a canal: Panama".to_owned()),
    true
  );
}

#[test]
fn test_reverse_string() {
  #[cfg(test)]
  use crate::easy_strings::reverse_string;

  let mut hello: Vec<char> = "hello".chars().collect();
  let mut goodbye: Vec<char> = "goodbye".chars().collect();
  reverse_string(&mut hello);
  reverse_string(&mut goodbye);
  assert_eq!(hello, "olleh".chars().collect::<Vec<char>>());
  assert_eq!(goodbye, "eybdoog".chars().collect::<Vec<char>>());
}

#[test]
fn test_is_anagram() {
  #[cfg(test)]
  use crate::easy_strings::is_anagram;

  assert_eq!(is_anagram("aavb".to_owned(), "bb".to_owned()), false);
  assert_eq!(is_anagram("rat".to_owned(), "art".to_owned()), true);
  assert_eq!(is_anagram("rating".to_owned(), "artink".to_owned()), false);
  assert_eq!(is_anagram("aa".to_owned(), "bb".to_owned()), false);
}

#[test]
fn test_str_str() {
  #[cfg(test)]
  use crate::easy_strings::str_str;

  assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
  assert_eq!(str_str(String::from("aaaa"), String::from("ba")), -1);
}

#[test]
fn test_first_uniq_char() {
  #[cfg(test)]
  use crate::easy_strings::first_uniq_char;

  assert_eq!(first_uniq_char("loveleetcode".to_owned()), 2);
  assert_eq!(first_uniq_char("cc".to_owned()), -1);
}
