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

#[test]
fn test_str_str() {
  assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
  assert_eq!(str_str(String::from("aaaa"), String::from("ba")), -1);
}
