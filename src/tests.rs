use crate::easy_arrays::{max_profit, remove_duplicates_sorted_array};
use crate::easy_strings::{reverse_string, is_palindrome};
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
fn test_reverse_string(){
  let mut input_arr : Vec<char> = "hello".chars().collect();
  reverse_string(&mut input_arr);
  assert_eq!(input_arr,"olleh".chars().collect::<Vec<char>>());
}

#[test]
fn test_is_palindrome(){
  assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_owned()), true);
}