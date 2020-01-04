mod easy_arrays;
mod easy_strings;
mod performance;
mod plus_one;
mod tests;

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
