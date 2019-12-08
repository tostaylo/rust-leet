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

//Is this O(nm) or O(n^2)?
pub fn str_str(haystack: String, needle: String) -> i32 {
  if needle.is_empty() || haystack == needle {
    return 0;
  }
  let step = needle.len();
  let mut index = 0;
  while index <= haystack.len() - 1 {
    // Probably slow and takes up a ton of memory because of this line.
    // I'm creating a new collection every loop. It takes time to allocate memory.
    let haystack_segment: String = haystack.chars().skip(index).take(step).collect();
    if needle == haystack_segment {
      return index as i32;
    }
    index = index + 1;
  }
  -1
}

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
