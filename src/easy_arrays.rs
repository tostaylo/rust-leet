use std::cmp;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

//Requirements: Modify in-place with 0(1) space complexity
pub fn remove_duplicates_sorted_array(nums: &mut Vec<i32>) -> i32 {
  if nums.len() > 0 {
    for index in 1..nums.len() {
      while index <= nums.len() - 1 && nums[index] == nums[index - 1] {
        nums.remove(index);
      }
    }
  }
  nums.len() as i32
}

// pub fn max_profit(prices: Vec<i32>) -> i32 {
//   let mut total = 0;
//   for index in 0..prices.len() - 1 {
//     let start_day = index;
//     let mut end_day = start_day + 1;
//     let mut largest = 0;
//     while end_day <= prices.len() - 1 {
//       if prices[start_day] >= prices[end_day] {
//         end_day = end_day + 1;
//       } else {
//         let diff = prices[end_day] - prices[start_day];
//         println!(
//           "start {:?} end {:?}  largest {:?} ",
//           prices[start_day], prices[end_day], largest
//         );
//         if diff > largest {
//           largest = diff;
//           end_day = end_day + 1;
//           total = total + largest;
//           println!("{:?} ", largest);
//         } else {
//           end_day = end_day + 1;
//         }
//       }
//     }
//   }
//   total
// }

// O(n) Time and Space complexity
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut nums_map: HashMap<i32, i32> = HashMap::new();
  for (index, value) in nums.iter().enumerate() {
    let diff = target - value;
    match nums_map.get(value) {
      Some(val) => return vec![*val, index as i32],
      None => nums_map.insert(diff, index as i32),
    };
  }
  unreachable!();
}

pub fn rotate_array(nums: &mut Vec<i32>, k: i32) {
  let len = nums.len() as i32;
  if len == k {
    return;
  }

  let steps = match k > len {
    true => k,
    false => len - k,
  };

  for _i in 0..steps {
    let value_to_move = nums.remove(0);
    nums.push(value_to_move)
  }
}
// O(n) time and space
// Could have used a stack or hashset to push/pop since every element appears exactly twice but one.
pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut map = HashMap::new();
  for num in nums {
    let val = match map.entry(num) {
      Occupied(entry) => entry.into_mut(),
      Vacant(entry) => entry.insert(0),
    };
    *val += 1;
  }
  for (key, value) in map {
    if value == 1 {
      return key;
    }
  }
  0
}

// Don't need the second hash map here. Could loop through long and decrement short_counts by 1 everytime there is a match
// If short_counts[val] has a value greater than 0 we are good.
// Thinking this is worse than O(n) Time Complexity but need to verify.
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
  // Determine the shortest and longest vector
  let (short, long) = match nums1.len() >= nums2.len() {
    true => (nums2, nums1),
    false => (nums1, nums2),
  };

  // Add the count of each item to a hashmap
  let mut long_counts = HashMap::new();
  for number in long {
    *long_counts.entry(number).or_insert(0) += 1;
  }
  let mut short_counts = HashMap::new();
  for number in &short {
    *short_counts.entry(number).or_insert(0) += 1;
  }

  let mut intersections = vec![];
  // Only loop through the shorter hashmap since it is more efficient
  // Push to the result the item which is in both hashmaps but has the smallest count.
  for (key, value) in short_counts {
    if long_counts.contains_key(&key) {
      let count = cmp::min(long_counts.get(&key).unwrap(), &value);
      for _index in 0..*count {
        intersections.push(*key);
      }
    }
  }
  intersections
}

pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut index = 0;
  let mut count = 0;
  // Find the 0's in nums and remove them. Keeping the rest of the array in order.
  while index < nums.len() {
    if nums[index] == 0 {
      nums.remove(index);
      count = count + 1;
    } else {
      index = index + 1;
    }
  }
  // Add the number of 0's counted to the end of the num.
  for _index in 0..count {
    nums.push(0);
  }
}

//incorrect answer currently
//https://stackoverflow.com/questions/42519/how-do-you-rotate-a-two-dimensional-array
//matrix[row][column]
//Doing the multiplication route with holding one value at a time. Creating a lookup map with coordinates of the item which is in the rotated spot.
pub fn rotate_image(matrix: &mut Vec<Vec<i32>>) {
  let layers = matrix.len() / 2;
  let end = matrix.len() - 1;
  for current_layer in 0..layers {
    rotate(
      current_layer,
      (end as u32 - current_layer as u32) as usize,
      matrix,
    );
  }

  fn rotate(start: usize, end: usize, matrix: &mut Vec<Vec<i32>>) {
    let temp = set_temp(matrix, start, end);
    set_left(matrix, start, end);
    set_bottom(matrix, start, end);
    set_right(matrix, start, end);
    set_top(matrix, temp, start);

    fn set_temp(matrix: &mut Vec<Vec<i32>>, start: usize, end: usize) -> Vec<i32> {
      let mut temp = vec![];
      for index in start..end + 1 {
        temp.push(matrix[index][start]);
      }
      temp
    }
    fn set_left(matrix: &mut Vec<Vec<i32>>, start: usize, end: usize) {
      // Make sure not to set the start origin here
      for index in start + 1..end + 1 {
        matrix[index][start] = matrix[end][index];
      }
    }
    fn set_bottom(matrix: &mut Vec<Vec<i32>>, start: usize, end: usize) {
      let mut end_increment = 0;
      for index in start..end + 1 {
        // Need two different incrementing variables
        matrix[end][index] = matrix[end - end_increment][end];
        end_increment = end_increment + 1;
      }
    }

    fn set_right(matrix: &mut Vec<Vec<i32>>, start: usize, end: usize) {
      for index in (start..end + 1).rev() {
        matrix[index][end] = matrix[start][index];
      }
    }
    fn set_top(matrix: &mut Vec<Vec<i32>>, temp: Vec<i32>, start: usize) {
      for index in 0..temp.len() {
        matrix[start][index + start] = temp[temp.len() - 1 - index];
      }
    }
  }
}
