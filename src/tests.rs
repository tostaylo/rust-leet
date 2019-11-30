use crate::easy_arrays::{max_profit, remove_duplicates_sorted_array};
use crate::easy_strings::{is_anagram, reverse_string};
// Note this useful idiom: importing names from outer (for mod tests) scope.
//  use super::*;

#[test]
fn test_remove_duplicates_sorted_array() {
  assert_eq!(
    remove_duplicates_sorted_array(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
    5
  );
  assert_eq!(remove_duplicates_sorted_array(&mut vec![0, 0, 1]), 2);
  assert_eq!(remove_duplicates_sorted_array(&mut vec![1, 1]), 1);
}

// #[test]
// fn test_max_profit() {
//   assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7)
// }

#[test]
fn test_reverse_string() {
  let mut hello: Vec<char> = "hello".chars().collect();
  let mut goodbye: Vec<char> = "goodbye".chars().collect();
  reverse_string(&mut hello);
  reverse_string(&mut goodbye);
  assert_eq!(hello, "olleh".chars().collect::<Vec<char>>());
  assert_eq!(goodbye, "eybdoog".chars().collect::<Vec<char>>());
}

#[test]
fn test_is_anagram() {
  assert_eq!(is_anagram("aavb".to_owned(), "bb".to_owned()), false);
  assert_eq!(is_anagram("rat".to_owned(), "art".to_owned()), true);
  assert_eq!(is_anagram("rating".to_owned(), "artink".to_owned()), false);
  assert_eq!(is_anagram("aa".to_owned(), "bb".to_owned()), false);
}
