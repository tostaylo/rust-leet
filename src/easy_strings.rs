use std::collections::HashMap;
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

// Sorting and comparing would be faster
pub fn is_anagram(s: String, t: String) -> bool {
  let mut combined_map: HashMap<char, HashMap<usize, u8>> = HashMap::new();
  let input_as_arr: Vec<String> = vec![s, t];
  for (idx, input_string) in input_as_arr.iter().enumerate() {
    for letter in input_string.chars() {
      match combined_map.get(&letter) {
        Some(hash_map) => {
          let mut new_map = hash_map.clone();
          match hash_map.get(&idx) {
            Some(index) => new_map.insert(idx, index + 1),
            None => new_map.insert(idx, 1),
          };
          combined_map.insert(letter, new_map);
        }
        None => {
          let mut values = HashMap::new();
          values.insert(idx, 1);
          combined_map.insert(letter, values);
        }
      };
    }
  }

  for value in combined_map.values() {
    if value.values().len() <= 1 || value[&1] != value[&0] {
      return false;
    }
  }
  true
}
