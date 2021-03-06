mod contains_duplicates;
mod count_and_say;
mod first_uniq_char;
mod intersect;
mod is_anagram;
mod is_palindrome;
mod longest_common_prefix;
mod move_zeroes;
mod performance;
mod plus_one;
mod remove_duplicates_sorted_array;
mod reverse_string;
mod rotate_array;
mod rotate_image;
mod single_number;
mod str_str;
mod two_sum;

// Identify all of these mods as easy somehow
fn main() {
  test_perf_plus_one();
}

fn test_perf_plus_one() {
  use plus_one::{plus_one, plus_one_faster};
  performance::compare_perf(
    vec![plus_one, plus_one_faster],
    vec![9, 9, 9, 9, 9, 9, 9],
    1000000,
  );
}
