use crate::easy_arrays::{max_profit, remove_duplicates_sorted_array};
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

#[test]
fn test_max_profit() {
  assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7)
}
