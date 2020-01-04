use std::time::Instant;

// Can this be made more generic?
pub fn compare_perf(functions: Vec<fn(Vec<i32>) -> Vec<i32>>, arg: Vec<i32>, iterations: i64) {
  for (item_number, item) in functions.iter().enumerate() {
    let now = Instant::now();
    for _index in 0..iterations {
      item(arg.clone());
    }
    println!(
      "{:?} function ran in {}ms",
      item_number,
      now.elapsed().as_millis()
    );
  }
}
