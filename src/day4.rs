pub struct Day4Data {
  data: String,
  rows: usize
}

fn day4_tokenize(input: String) -> Day4Data {
  let split = input.split_ascii_whitespace();
  let rows = split.clone().count();
  let data = split.collect();
  return Day4Data{ data, rows };
}

fn day4_check(index: usize, input: &str, o1: isize, o2: isize, o3: isize, o4: isize) -> bool {
  let buffer = input.as_bytes();
  let si = index as isize;
  let offset1 = si + o1;
  let offset2 = si + o2;
  let offset3 = si + o3;
  let offset4 = si + o4;
  let max = buffer.len() as isize;
  if offset1 >= max || offset2 >= max || offset3 >= max || offset4 >= max {
    return false;
  }
  if offset1 < 0 || offset2 < 0 || offset3 < 0 || offset4 < 0 {
    return false;
  }

  let x = buffer[offset1 as usize] == 'X' as u8;
  let m = buffer[offset2 as usize] == 'M' as u8;
  let a = buffer[offset3 as usize] == 'A' as u8;
  let s = buffer[offset4 as usize] == 'S' as u8;

  return x && m && a && s;
}

// 1. Forwards (N, N+1, N+2, N+3)
fn day4_normal(index: usize, input: &Day4Data) -> bool {
  return day4_check(index, input.data.as_str(), 0, 1, 2, 3);
}

// 2. Backwards (N, N-1, N-2, N-3)
fn day4_backwards(index: usize, input: &Day4Data) -> bool {
  return day4_check(index, input.data.as_str(), 0, -1, -2, -3);
}

// 3. Rotated -90 Degrees (N, N-M, N-2M, N-3M)
fn day4_ccw(index: usize, input: &Day4Data) -> bool {
  let delta = input.rows as isize;
  let s2 = -delta;
  let s3 = -2*delta;
  let s4 = -3*delta;
  return day4_check(index, input.data.as_str(), 0, s2,s3, s4);
}

// 4. Rotated +90 Degrees (N, N+M, N+2M, N+3M)
fn day4_cw(index: usize, input: &Day4Data) -> bool {
  let delta = input.rows as isize;
  let s2 = delta;
  let s3 = 2*delta;
  let s4 = 3*delta;
  return day4_check(index, input.data.as_str(), 0, s2,s3, s4);
}

// 5. Diagonal 1 (N, N+(M+1), N+2*(M+1), N+3*(M+1))
fn day4_d1(index: usize, input: &Day4Data) -> bool {
  let delta = (input.rows + 1) as isize;
  let s2 = delta;
  let s3 = 2*delta;
  let s4 = 3*delta;
  return day4_check(index, input.data.as_str(), 0, s2,s3, s4);
}

// 6. Diagonal 2 (N, N+(M-1), N+2*(M-1), N+3*(M-1))
fn day4_d2(index: usize, input: &Day4Data) -> bool {
  let delta: isize = (input.rows - 1) as isize;
  let s2 = delta;
  let s3 = 2*delta;
  let s4 = 3*delta;
  return day4_check(index, input.data.as_str(), 0, s2,s3, s4);
}

// 7. Diagonal 3 (N, N-(M+1), N-2*(M+1), N-3*(M+1))
fn day4_d3(index: usize, input: &Day4Data) -> bool {
  let delta: isize = (input.rows + 1) as isize;
  let s2 = -delta;
  let s3 = -2*delta;
  let s4 = -3*delta;
  return day4_check(index, input.data.as_str(), 0, s2,s3, s4);
}

// 8. Diagonal 4 (N, N-(M-1), N-2*(M-1), N-3*(M-1))
fn day4_d4(index: usize, input: &Day4Data) -> bool {
  let delta: isize = (input.rows - 1) as isize;
  let s2 = -delta;
  let s3 = -2*delta;
  let s4 = -3*delta;
  return day4_check(index, input.data.as_str(), 0, s2,s3, s4);
}

pub fn day4_tokenize_sample() -> Day4Data {
  return day4_tokenize("XMASXMAS
XMASXMAS
XMASXMAS
XMASXMAS
XMASXMAS
XMASXMAS
XMASXMAS
XMASXMAS
".to_string());
}

pub fn day4_tokenize_example() -> String {
  return "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
".to_string();
}

#[test]
fn day4_tokenize_test() {
  let sample = day4_tokenize_sample();
  assert!(sample.rows == 8);
}

#[test]
fn day4_normal_test() {
  let sample = day4_tokenize_sample();
  let result_1 = day4_normal(0, &sample);
  let result_2 = day4_normal(1, &sample);
  let result_3 = day4_normal(2, &sample);
  let result_4 = day4_normal(3, &sample);
  assert!(result_1 && (result_2 == false) && (result_3 == false) && (result_4 == false));
}

pub fn main(input: String) -> usize {
  let tokens = day4_tokenize(input);

  let mut i = 0;
  let mut result = 0;
  let max = tokens.data.len();
  while i < max {
    if day4_normal(i, &tokens) { result += 1; }
    if day4_backwards(i, &tokens) { result += 1; }
    if day4_ccw(i, &tokens) { result += 1; }
    if day4_cw(i, &tokens) { result += 1; }
    if day4_d1(i, &tokens) { result += 1; }
    if day4_d2(i, &tokens) { result += 1; }
    if day4_d3(i, &tokens) { result += 1; }
    if day4_d4(i, &tokens) { result += 1; }
    i += 1;
  }

  return result;
}

#[test]
fn day4_example_pretest() {
  let input = day4_tokenize_example();

  let data = day4_tokenize(input);
  println!("Rows: {}", data.rows);
  assert!(data.rows == 10);

  let pretest = day4_d1(4, &data);
  println!("Index 4 Diagonal 1: {}", pretest);
  assert!(pretest);
}

#[test]
fn day4_example_test() {
  let input = day4_tokenize_example();

  let result = main(input);
  println!("Result: {}", result);
  assert!(result == 18);
}

pub fn part2(_input: String) -> usize {
  return 0;
}