use std::time::Instant;

// Can this be made more generic?
// Or do I need to make a seperate test function for each function parameter and return type
// Do I instead need to create a struct which compares has methods for compare_perF_vec, compare_perf_string, etc?
// Or do I cast the function being passed in as a specific type I create? Or implements a trait I create?
// Many possibilities here.
// functions: Vec<fn(Vec<i32>) -> Vec<i32>> this works but isn't flexible
// generic type where T implements a clone trait?
// If I figure this out, this could be a blog post
// So now I have it for Clone trait but do strings implement Clone trait?
// May still need to make a struct for this with methods for strings vs vecs
// Also this is expecting T to be a value and not a reference, can it be made to accept values and references?
// Can it be made to expect a return type to the function or no return type?
// Writing this performance struct and implement traits for it would be a good exercise.

// First I only wanted to write a function which could take in two functions and run performance tests on them.
// Show what that function looked like.
// I soon realized this approach was going to be limiting. I wanted to be able to time functions for functions which accepted different parameter types and returned different types.
// Which led me to generics.
// Pros and Cons of this approach
// can i get the actual name of the function being passed in?

// Intro - As a way to learn rust I am working through LeetCode problems in rust. I should probably rename this repo to rust-leetcode
// Some of the problems in leetcode did not have timing
// So here we are.
pub fn compare_perf<T: Clone>(functions: Vec<fn(T) -> T>, arg: T, iterations: i64) {
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
