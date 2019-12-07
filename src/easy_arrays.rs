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

pub fn max_profit(prices: Vec<i32>) -> i32 {
  let mut total = 0;
  for index in 0..prices.len() - 1 {
    let start_day = index;
    let mut end_day = start_day + 1;
    let mut largest = 0;
    while end_day <= prices.len() - 1 {
      if prices[start_day] >= prices[end_day] {
        end_day = end_day + 1;
      } else {
        let diff = prices[end_day] - prices[start_day];
        println!(
          "start {:?} end {:?}  largest {:?} ",
          prices[start_day], prices[end_day], largest
        );
        if diff > largest {
          largest = diff;
          end_day = end_day + 1;
          total = total + largest;
          println!("{:?} ", largest);
        } else {
          end_day = end_day + 1;
        }
      }
    }
  }
  total
}

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
