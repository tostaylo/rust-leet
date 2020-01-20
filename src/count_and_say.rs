use std::char;
use std::collections::HashMap;

pub fn count_and_say(n: i32) -> String {
  return count_chars("1".to_owned(), n);
}
// Comment this out better
pub fn count_chars(s: String, n: i32) -> String {
  if n == 1 {
    return s;
  }

  let mut map = HashMap::new();
  let mut result = "".to_owned();
  for item in s.chars() {
    match map.contains_key(&item) {
      true => {
        // this could be better
        let counter = map.entry(item).or_insert(0);
        *counter += 1;
      }
      false => {
        if map.keys().len() >= 1 {
          // make this reusable
          let value_count = map.get_key_value(map.keys().nth(0).unwrap()).unwrap();
          result.push(char::from_digit(*value_count.1 as u32, 10).unwrap());
          result.push(*value_count.0);
        }
        map = HashMap::new();
        map.insert(item, 1);
      }
    }
  }
  // make this reusable
  let value_count = map.get_key_value(map.keys().nth(0).unwrap()).unwrap();
  result.push(char::from_digit(*value_count.1 as u32, 10).unwrap());
  result.push(*value_count.0);
  return count_chars(result, n - 1);
}

#[test]
fn testcount_and_say() {
  assert_eq!(count_and_say(5), "111221".to_owned());
  assert_eq!(count_and_say(4), "1211".to_owned());
  assert_eq!(count_and_say(3), "21".to_owned());
  assert_eq!(count_and_say(2), "11".to_owned());
  assert_eq!(count_and_say(1), "1".to_owned())
}
