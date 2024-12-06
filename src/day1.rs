
pub fn main(input: String) -> i32 {
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

pub fn part2(input: String) -> usize {
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