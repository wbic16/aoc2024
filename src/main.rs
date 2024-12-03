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
  println!("Day 2: {} + {}", result2_1, 0);
}
