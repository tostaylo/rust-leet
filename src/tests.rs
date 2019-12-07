use crate::easy_arrays::{max_profit, remove_duplicates_sorted_array, rotate_array, two_sum};
use crate::easy_strings::{is_anagram, reverse_string, str_str};
// Note this useful idiom: importing names from outer (for mod tests) scope.
//  use super::*;

// Easy Arrays
#[test]
fn test_remove_duplicates_sorted_array() {
  assert_eq!(
    remove_duplicates_sorted_array(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
    5
  );
  assert_eq!(remove_duplicates_sorted_array(&mut vec![0, 0, 1]), 2);
  assert_eq!(remove_duplicates_sorted_array(&mut vec![1, 1]), 1);
}

#[test]
fn test_two_sum() {
  assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  assert_eq!(two_sum(vec![2, 7, 11, 15], 26), vec![2, 3]);
  assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn test_rotate_array() {
  let mut test_collection = vec![1, 2, 3, 4, 5, 6, 7];
  rotate_array(&mut test_collection, 3);
  assert_eq!(test_collection, vec![5, 6, 7, 1, 2, 3, 4]);
  let mut test_collection2 = vec![1, 2];
  rotate_array(&mut test_collection2, 3);
  assert_eq!(test_collection2, vec![2, 1]);
}

// #[test]
// fn test_max_profit() {
//   assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7)
// }

// Easy Strings

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

#[test]
fn test_str_str() {
  assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
  assert_eq!(str_str(String::from("aaaa"), String::from("ba")), -1);
}
