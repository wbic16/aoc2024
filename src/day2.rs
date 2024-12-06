
pub fn main(input: String) -> usize {
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

pub fn part2(input: String) -> usize {
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