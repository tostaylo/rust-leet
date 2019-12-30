// Note this useful idiom: importing names from outer (for mod tests) scope.
//  use super::*;

// Easy Arrays
#[test]
fn test_remove_duplicates_sorted_array() {
  #[cfg(test)]
  use crate::easy_arrays::remove_duplicates_sorted_array;

  assert_eq!(
    remove_duplicates_sorted_array(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
    5
  );

  assert_eq!(remove_duplicates_sorted_array(&mut vec![0, 0, 1]), 2);
  assert_eq!(remove_duplicates_sorted_array(&mut vec![1, 1]), 1);
}

#[test]
fn test_two_sum() {
  #[cfg(test)]
  use crate::easy_arrays::two_sum;

  assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  assert_eq!(two_sum(vec![2, 7, 11, 15], 26), vec![2, 3]);
  assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn test_rotate_array() {
  #[cfg(test)]
  use crate::easy_arrays::rotate_array;

  let mut test_collection = vec![1, 2, 3, 4, 5, 6, 7];
  rotate_array(&mut test_collection, 3);
  assert_eq!(test_collection, vec![5, 6, 7, 1, 2, 3, 4]);
  let mut test_collection2 = vec![1, 2, 3];
  rotate_array(&mut test_collection2, 4);
  assert_eq!(test_collection2, vec![3, 1, 2]);
}

#[test]
fn test_single_number() {
  #[cfg(test)]
  use crate::easy_arrays::single_number;

  assert_eq!(single_number(vec![2, 2, 1]), 1);
  assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
}

#[test]
fn test_intersect() {
  #[cfg(test)]
  use crate::easy_arrays::intersect;

  assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), [2, 2]);
}

#[test]
fn test_move_zeroes() {
  #[cfg(test)]
  use crate::easy_arrays::move_zeroes;

  let mut tester_vec = vec![0, 1, 0, 3, 12];
  move_zeroes(&mut tester_vec);
  assert_eq!(tester_vec, vec![1, 3, 12, 0, 0]);
}

#[test]
fn test_rotate_image() {
  #[cfg(test)]
  use crate::easy_arrays::rotate_image;
  let mut tester = vec![
    vec![43, 39, 3, 33, 37, 20, 14],
    vec![9, 18, 9, -1, 40, 22, 38],
    vec![14, 42, 3, 23, 12, 14, 32],
    vec![18, 31, 45, 11, 8, -1, 31],
    vec![28, 44, 14, 23, 40, 24, 13],
    vec![29, 45, 33, 45, 20, 0, 45],
    vec![12, 23, 35, 32, 22, 39, 8],
  ];
  rotate_image(&mut tester);
  assert_eq!(
    tester,
    vec![
      vec![12, 29, 28, 18, 14, 9, 43],
      vec![23, 45, 44, 31, 42, 18, 39],
      vec![35, 33, 14, 45, 3, 9, 3],
      vec![32, 45, 23, 11, 23, -1, 33],
      vec![22, 20, 40, 8, 12, 40, 37],
      vec![39, 0, 24, -1, 14, 22, 20],
      vec![8, 45, 13, 31, 32, 38, 14]
    ]
  );
  // let mut tester = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
  // rotate_image(&mut tester);
  // assert_eq!(tester, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
}

// #[test]
// fn test_max_profit() {
//   assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7)
// }

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
