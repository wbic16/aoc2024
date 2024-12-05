use libphext::phext;
use std::fs;

fn day1(input: String) -> i32 {
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();

  let lines = input.split('\n').into_iter();
  for line in lines {
    let pair: Vec<&str> = line.split_whitespace().collect();
    left.push(pair[0].parse::<i32>().expect("error 1"));
    right.push(pair[1].parse::<i32>().expect("error 2"));
  }
  // println!("Loaded Left: {}, Right: {} items.", left.len(), right.len());

  left.sort();
  right.sort();

  let mut i: usize = 0;
  let mut total = 0;
  while i < left.len() {
    let a = left[i];
    let b = right[i];
    let distance = b - a;
    total += distance.abs();
    // un-comment to audit the process
    //if i < 10 { println!("{}: {} vs {}: {} -> {}", i, a, b, distance, total); }
    i += 1;
  }

  return total;
}

fn day1_part2(input: String) -> usize {
  let mut left: Vec<i32> = Vec::new();
  let mut right: Vec<i32> = Vec::new();

  let lines = input.split('\n').into_iter();
  for line in lines {
    let pair: Vec<&str> = line.split_whitespace().collect();
    left.push(pair[0].parse::<i32>().expect("error 1"));
    right.push(pair[1].parse::<i32>().expect("error 2"));
  }
  // println!("Loaded Left: {}, Right: {} items.", left.len(), right.len());

  let mut similarity: usize = 0;
  //let mut i: usize = 0;
  for number in left {
    let count = right.iter().filter(|s| **s == number).count();
    // un-comment to audit the process
    //if i < 10 && count > 0 { println!("{}: {} vs {}: {}", i, number, count, similarity); i += 1; }
    similarity += (number as usize) * count;
  }

  return similarity;
}

fn day2(input: String) -> usize {
  let lines = input.split('\n').into_iter();
  //let mut i = 0;
  let mut result = 0;

  for line in lines {
    let row: Vec<&str> = line.split_whitespace().collect();
    let mut prior = 0;
    let mut value = 0;
    let mut is_decreasing = false;
    let mut is_increasing = false;
    let mut no_change_observed = false;
    let mut big_delta_observed = false;
    let mut started = false;    
    for element in row.clone() {
      if started { prior = value; }
      value = element.parse::<i32>().expect("error 1");
      if !started { started = true; continue; }

      let delta = value - prior;
      if delta.abs() > 3 {
        big_delta_observed = true;
      }
      if delta == 0 {
        no_change_observed = true;
      } else if delta > 0 {
        is_increasing = true;
      } else if delta < 0 {
        is_decreasing = true;
      }
    }

    let both_directions = is_increasing && is_decreasing; // not safe
    let safe = (both_directions == false) && (no_change_observed == false) && (big_delta_observed == false) && row.len() > 1;
    if safe {
      result += 1;
    //  if i < 10 {
    //    println!("Safe: {}", line);
    //    i += 1;
    //  }
    //} else {
    //  if i < 10 {
    //    println!("Unsafe: {} (both: {}, NC: {}, BD: {})", line, both_directions, no_change_observed, big_delta_observed);
    //}
    }
  }

  return result;
}

#[derive(Default)]
pub struct Day2Node {
  pub value: i32,
  pub enabled: bool
}
impl Day2Node {
  pub fn new() -> Day2Node {
    Day2Node { value: 0, enabled: false }
  }
}

fn day2_tokenize(values: &str) -> Vec<Day2Node> {
  let mut tokens: Vec<Day2Node> = Default::default();

  let row: Vec<&str> = values.split_whitespace().collect();
  for item in row {
    let value = item.parse::<i32>().expect("error 1");
    let mut next = Day2Node::new();
    next.value = value;
    next.enabled = true;
    tokens.push(next);
  }

  return tokens;
}

fn day2_validate(line: &str, report: &Vec<Day2Node>) -> bool {
  let mut prior;
  let mut started = false;
  let mut value = 0;
  let mut increasing = 0;
  let mut result = false;
  let mut index = 0;
  for node in report {
    index += 1;
    if !node.enabled {
      println!("{}: Skipping index {}", line, index);
      continue;
    }

    if !started {
      started = true;
      value = node.value;
      continue;
    }
    
    prior = value;
    value = node.value;
    if increasing == 0 {
      if value > prior { increasing = 1;  }
      if value < prior { increasing = -1; }
    }

    let delta = (value - prior).abs();
    if delta > 3 {
      println!("{}: Step Failure at index {} ({}-{}) due to delta > 3.", line, index, value, prior);
      return false;
    }
    if delta == 0 {
      println!("{}: Step Failure at index {} ({}-{}) due to delta == 0.", line, index, value, prior);
      return false;
    }
    if delta < 0 && increasing == 1  {
      println!("{}: Step Failure at index {} ({}-{}) due to -/+ mismatch.", line, index, value, prior);
      return false;
    }
    if delta > 0 && increasing == -1 {
      println!("{}: Step Failure at index {} ({}-{}) due to +/- mismatch.", line, index, value, prior);
      return false;
    }

    result = true;
  }

  return result;
}

fn day2_part2(input: String) -> usize {
  let lines = input.split('\n').into_iter();
  let mut result = 0;
  let mut blip: i32 = -1;

  for line in lines {
    let mut items = day2_tokenize(line);

    let start: i32 = 0;
    let end = items.len() as i32;

    let mut passed = day2_validate(line, &items);
    if passed {
      result += 1;
    } else {
      blip = start as i32;
      while (passed == false) && (blip < end) {
        let mut ri: usize = 0;
        while ri < end as usize {
          items[ri].enabled = ri != (blip as usize);
          ri += 1;
        }
        passed = day2_validate(line, &items);
        if passed {
          result += 1;
        }
        blip += 1;
      }
    }

    if passed == false {
      println!("{}: Failed\n\n", line);
    } else {
      let mut blipped = format!("");
      if blip >= 0 { blipped = format!(" ({})", blip); }
      println!("{}: Passed{}\n", line, blipped);
    }
  }

  return result;
}

fn day3_tokenize(input: String, enabler: bool) -> Vec<i64> {
  let mut result: Vec<i64> = Default::default();

  let mut left: i64 = 0;
  let mut right: i64 = 0;
  let mut stage = 0;
  let mut pre_stage = 0;
  let mut pre = String::new();
  const ZERO: u8 = '0' as u8;
  const NINE: u8 = '9' as u8;
  let mut left_len = 0;
  let mut right_len = 0;
  let mut enabled = true;
  for char in input.as_bytes() {
    if enabler {
      if pre_stage == 0 && *char == 'd' as u8 { pre_stage = 1; pre.push(*char as char); continue; }
      if pre_stage == 1 && *char == 'o' as u8 { pre_stage = 2; pre.push(*char as char); continue; }
      if pre_stage == 2 && *char == '(' as u8 { pre_stage = 3; continue; }
      if pre_stage == 3 && *char == ')' as u8 && pre == "do" {
        enabled = true;
        pre_stage = 0;
        stage = 0;
        pre.clear();
        continue;
      }
      if pre_stage == 2 && *char == 'n' as u8 { pre_stage = 3; pre.push(*char as char); continue; }
      if pre_stage == 3 && *char == '\'' as u8 { pre_stage = 4; pre.push(*char as char); continue; }
      if pre_stage == 4 && *char == 't' as u8 { pre_stage = 5; pre.push(*char as char); continue; }
      if pre_stage == 5 && *char == '(' as u8 { pre_stage = 6; continue; }
      if pre_stage == 6 && *char == ')' as u8 && pre == "don't" {
        enabled = false;
        pre_stage = 0;
        stage = 0;
        pre.clear();
        continue;
      }

      if enabled == false { continue; }
    }

    if stage == 0 && *char == 'm' as u8 { stage = 1; continue; }
    if stage == 1 && *char == 'u' as u8 { stage = 2; continue; }
    if stage == 2 && *char == 'l' as u8 { stage = 3; continue; }
    if stage == 3 && *char == '(' as u8 { stage = 4; continue; }    
    if stage == 4 && *char <= NINE && *char >= ZERO && left_len < 3 {
      left = left * 10 + ((*char - ZERO) as i64);
      left_len += 1;
      //println!("Left: {}", left);
      continue;
    }
    if stage == 4 && *char == ',' as u8 { stage = 5; continue; }
    if stage == 5 && *char <= NINE && *char >= ZERO && right_len < 3 {
      right = right * 10 + ((*char - ZERO) as i64);
      right_len += 1;
      //println!("Right: {}", right);
      continue;
    }
    if stage == 5 && *char == ')' as u8 {
      let mul = left * right;
      result.push(mul);
      //println!("Mul: {} x {} = {}", left, right, mul);
    }

    stage = 0;
    left = 0;
    right = 0;
    left_len = 0;
    right_len = 0;
  }

  return result;
}

fn day3(input: String) -> usize {
  let records = day3_tokenize(input, false);
  let mut sum: usize = 0;
  for value in records {
    sum += value as usize;
  }

  return sum;
}

fn day3_part2(input: String) -> usize {
  let records = day3_tokenize(input, true);
  let mut sum: usize = 0;
  for value in records {
    sum += value as usize;
  }

  return sum;
}

pub struct Day4Data {
  data: String,
  rows: usize
}

fn day4_tokenize(input: String) -> Day4Data {
  let split = input.split('\n');
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
XMASXMAS".to_string());
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
MXMXAXMASX".to_string();
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

fn day4(input: String) -> usize {
  let tokens = day4_tokenize(input);

  let mut i = 0;
  let mut result = 0;
  while i < tokens.data.len() {
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

  let result = day4(input);
  println!("Result: {}", result);
  assert!(result == 18);
}

fn day4_part2(_input: String) -> usize {
  return 0;
}

fn main() {
  println!("Advent of Code 2024");

  let filename = "problems.phext";
  let problems = fs::read_to_string(filename).expect("Unable to locate problem set.");
  let scroll1 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.1"));
  let result1_1 = day1(scroll1.clone());
  let result1_2 = day1_part2(scroll1);
  println!("Day 1: {} + {}", result1_1, result1_2);

  let scroll2 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.2"));
  let result2_1 = day2(scroll2.clone());
  let result2_2 = day2_part2(scroll2); // 687 too low, 701 too high
  println!("Day 2: {} + {}", result2_1, result2_2);

  let scroll3 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.3"));
  let result3_1 = day3(scroll3.clone());
  let result3_2 = day3_part2(scroll3); // 84,836,598 too high
  println!("Day 3: {} + {}", result3_1, result3_2);

  let scroll4 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.4"));
  let result4_1 = day4(scroll4.clone()); // 1442 too low, 2529 too high
  let result4_2 = day4_part2(scroll4);
  println!("Day 4: {} + {}", result4_1, result4_2);
}
