use std::collections::HashMap;
// byte code would've been faster.
// keeping count instead of pushing vectors would be another approach. Requires looping through s twice.
// O(n) Time and Space
pub fn first_uniq_char(s: String) -> i32 {
  // make map of chars in s as keys and vecs of their index's as values
  let mut map: HashMap<char, Vec<usize>> = HashMap::new();
  for (index, letter) in s.chars().enumerate() {
    map.entry(letter).or_insert(vec![]).push(index);
  }

  let mut min = s.len();
  // find all the vecs in map which have a length of 1 and then find the min value of those vecs.
  for (_key, value) in map {
    if value.len() == 1 {
      if value[0] < min {
        min = value[0];
      }
    }
  }
  // No uniques
  if min == s.len() {
    -1
  } else {
    min as i32
  }
}

#[test]
fn test_first_uniq_char() {
  assert_eq!(first_uniq_char("loveleetcode".to_owned()), 2);
  assert_eq!(first_uniq_char("cc".to_owned()), -1);
}
