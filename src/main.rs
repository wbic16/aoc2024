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

fn day3_tokenize(input: String) -> Vec<i64> {
  let mut result: Vec<i64> = Default::default();

  let mut left: i64 = 0;
  let mut right: i64 = 0;
  let mut stage = 0;
  const ZERO: u8 = '0' as u8;
  const NINE: u8 = '9' as u8;
  let mut left_len = 0;
  let mut right_len = 0;
  for char in input.as_bytes() {
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
  let records = day3_tokenize(input);
  let mut sum: usize = 0;
  for value in records {
    sum += value as usize;
  }

  return sum;
}

fn day3_part2(_input: String) -> usize {
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
  let result2_2 = day2_part2(scroll2); // 679,687 too low, 701 too high
  println!("Day 2: {} + {}", result2_1, result2_2);

  let scroll3 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.3"));
  let result3_1 = day3(scroll3.clone()); // 188,331,080 too high
  let result3_2 = day3_part2(scroll3);  // 183,669,043
  println!("Day 3: {} + {}", result3_1, result3_2);
}
