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
    let mut start_day = index;
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
