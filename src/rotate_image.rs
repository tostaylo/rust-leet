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

#[test]
fn test_rotate_image() {
  let mut tester = vec![
    vec![43, 39, 3, 33, 37, 20, 14],
    vec![9, 18, 9, -1, 40, 22, 38],
    vec![14, 42, 3, 23, 12, 14, 32],
    vec![18, 31, 45, 11, 8, -1, 31],
    vec![28, 44, 14, 23, 40, 24, 13],
    vec![29, 45, 33, 45, 20, 0, 45],
    vec![12, 23, 35, 32, 22, 39, 8],
  ];
  rotate_image(&mut tester);
  assert_eq!(
    tester,
    vec![
      vec![12, 29, 28, 18, 14, 9, 43],
      vec![23, 45, 44, 31, 42, 18, 39],
      vec![35, 33, 14, 45, 3, 9, 3],
      vec![32, 45, 23, 11, 23, -1, 33],
      vec![22, 20, 40, 8, 12, 40, 37],
      vec![39, 0, 24, -1, 14, 22, 20],
      vec![8, 45, 13, 31, 32, 38, 14]
    ]
  );
}
