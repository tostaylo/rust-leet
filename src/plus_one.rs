// Reverse Input
// Loop through reversed input
// If item < 9 add 1 to item and break
// Else set item to 0 and continue
// If last item is 0 push 1 to end of array
// Reverse input again and return
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
  let mut reversed_digits: Vec<i32> = digits.iter().rev().cloned().collect();
  for index in 0..reversed_digits.len() {
    if reversed_digits[index] < 9 {
      reversed_digits[index] = reversed_digits[index] + 1;
      break;
    } else {
      reversed_digits[index] = 0;
    }
  }
  if reversed_digits[reversed_digits.len() - 1] == 0 {
    reversed_digits.push(1);
  }
  reversed_digits.iter().rev().cloned().collect::<Vec<i32>>()
}

// Faster due to not reversing and cloning.
pub fn plus_one_faster(digits: Vec<i32>) -> Vec<i32> {
  let mut digits = digits;
  for idx in (0..digits.len()).rev() {
    if digits[idx] == 9 {
      digits[idx] = 0;
    } else {
      digits[idx] += 1;
      return digits;
    }
  }
  digits.insert(0, 1);
  digits
}

#[test]
fn test_plus_one() {
  #[cfg(test)]
  assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
}
