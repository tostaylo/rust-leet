// Easy Strings
// https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/887/
// Could have potentially looked for the shortest string in the vector as my test string?

pub fn longest_common_prefix(strings: Vec<String>) -> String {
  if strings.len() < 1 {
    return "".to_string();
  }
  if strings.len() == 1 {
    return strings[0].to_owned();
  }

  fn compare(char_index: usize, strings: Vec<String>, prefix: &mut String) -> String {
    for index in 1..strings.len() {
      let current = strings[index].chars().nth(char_index);
      let prev = strings[index - 1].chars().nth(char_index);
      match current {
        Some(_x) => {
          if current != prev {
            return prefix.to_owned();
          // Are on the last item of the vector?
          // If so add to prefix, recurse and move on to the next char_index for all items.
          } else if index == strings.len() - 1 {
            prefix.push(current.unwrap());
            return compare(char_index + 1, strings, prefix);
          }
        }
        None => return prefix.to_owned(),
      }
    }
    return "".to_string();
  }
  return compare(0, strings, &mut "".to_string());
}

// Nice idiomatic solution
// pub fn longest_common_prefixs(strs: Vec<String>) -> String {
//   let mut iter = strs.into_iter();
//   match iter.next() {
//     Some(prefix) => iter.fold(prefix, |pref, string| common_prefix(pref, string)),
//     None => "".to_string(),
//   }
// }

// fn common_prefix(str1: String, str2: String) -> String {
//   str1
//     .chars()
//     .zip(str2.chars())
//     .take_while(|(a, b)| a == b)
//     .map(|(a, _)| a)
//     .collect::<String>()
// }

#[test]
fn test_longest_common_prefix() {
  assert_eq!(
    longest_common_prefix(vec![
      "flower".to_string(),
      "flow".to_string(),
      "flight".to_string()
    ]),
    "fl"
  );
  assert_eq!(
    longest_common_prefix(vec![
      "dog".to_string(),
      "racecar".to_string(),
      "flight".to_string()
    ]),
    ""
  );

  assert_eq!(
    longest_common_prefix(vec!["a".to_string(), "b".to_string(),]),
    ""
  );
}
