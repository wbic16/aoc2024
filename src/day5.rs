pub struct Day5Node {
  first: u32,
  second: u32
}
#[derive(Default)]
pub struct Day5Data {
  pub reqs: Vec<Day5Node>,
  pub updates: Vec<Vec<u32>>
}

fn day5_tokenize(input: String) -> Day5Data {
  let mut result:Day5Data = Default::default();
  let rows: Vec<&str> = input.split_whitespace().collect();
  for row in rows {
    if row.contains("|") {
      let pair:Vec<&str> = row.split('|').collect();
      let first = pair[0].parse::<u32>().expect("left error");
      let second = pair[1].parse::<u32>().expect("right error");
      result.reqs.push(Day5Node{first, second});
    }

    if row.contains(",") {
      let nums:Vec<&str> = row.split(',').collect();
      let mut numbers:Vec<u32> = Default::default();
      for number in nums {
        numbers.push(number.parse::<u32>().expect("update err"));
      }
      result.updates.push(numbers);
    }
  }
  return result;
}

fn day5_validate(data: &Day5Data) -> usize {
  
  // step 1: build valid rows
  let mut valid:Vec<Vec<u32>> = Default::default();
  for row in data.updates.iter() {
    let mut passed = true;
    for req in data.reqs.iter() {
      let mut first_index = -1;
      let mut second_index = -1;
      let mut i = 0;

      for number in row {
        if *number == req.first {
          first_index = i;
        }
        if *number == req.second {
          second_index = i;
        }
        i += 1;
      }

      if first_index > -1 && second_index > -1 {
        if first_index > second_index {
          passed = false;
          //let summary = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
          //println!("Disallowed {} due to {}|{}.", summary, req.first, req.second);
          break;
        }
      }
    }

    if passed {
      valid.push(row.clone());
    }
  }

  let mut total: usize = 0;
  for row in valid {
    let max = row.len();
    let middle = (max + 1)/2 - 1;
    total += row[middle] as usize;
  }
  // step 2: count the middles (assume odd?)

  return total;
}

fn day5_fixup(data: &Day5Data) -> usize {
  
  // step 1: build invalid rows
  let mut invalid:Vec<Vec<u32>> = Default::default();
  for row in data.updates.iter() {
    let mut passed = true;
    for req in data.reqs.iter() {
      let mut first_index = -1;
      let mut second_index = -1;
      let mut i = 0;

      for number in row {
        if *number == req.first {
          first_index = i;
        }
        if *number == req.second {
          second_index = i;
        }
        i += 1;
      }

      if first_index > -1 && second_index > -1 {
        if first_index > second_index {
          passed = false;
          //let summary = row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
          //println!("Disallowed {} due to {}|{}.", summary, req.first, req.second);
          break;
        }
      }
    }

    if passed == false {
      invalid.push(row.clone());
    }
  }

  // fixup invalid rows
  //for row in invalid.iter() {
  //  for req in data.reqs.iter() {

  //  }
  //}

  let mut total: usize = 0;
  for row in invalid {
    let max = row.len();
    let middle = (max + 1)/2 - 1;
    total += row[middle] as usize;
  }
  // step 2: count the middles (assume odd?)

  return total;
}

pub fn main(input: String) -> usize {
  let tokens = day5_tokenize(input);
  let result = day5_validate(&tokens);

  return result;
}

pub fn part2(input: String) -> usize {
  let tokens = day5_tokenize(input);
  let result = day5_validate(&tokens);

  return result;
}

pub fn sample_data() -> String {
  return "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
}

#[test]
fn test_parse() {
  let test = day5_tokenize(sample_data());

  println!("Loaded {} requirements and {} updates.", test.reqs.len(), test.updates.len());
  assert!(21 == test.reqs.len());
  assert!(6 == test.updates.len());

  // 47|53
  // ...
  // 53|13
  assert!(test.reqs[0].first == 47);
  assert!(test.reqs[0].second == 53);
  assert!(test.reqs[20].first == 53);
  assert!(test.reqs[20].second == 13);

  // 75,47,61,53,29
  assert!(test.updates[0][0] == 75);
  assert!(test.updates[0][1] == 47);
  assert!(test.updates[0][2] == 61);
  assert!(test.updates[0][3] == 53);
  assert!(test.updates[0][4] == 29);
}

#[test]
fn test_calculation() {
  let tokens = day5_tokenize(sample_data());
  let result = day5_validate(&tokens);

  println!("Sample: {}", result);
  assert!(result == 143);
}

#[test]
fn test_reorder_1() {
  let tokens = day5_tokenize(sample_data());


}