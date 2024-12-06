use libphext::phext;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
  println!("Advent of Code 2024");

  let filename = "problems.phext";
  let problems = fs::read_to_string(filename).expect("Unable to locate problem set.");
  let scroll1 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.1"));
  let result1_1 = day1::main(scroll1.clone());
  let result1_2 = day1::part2(scroll1);
  println!("Day 1: {} + {}", result1_1, result1_2);

  let scroll2 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.2"));
  let result2_1 = day2::main(scroll2.clone());
  let result2_2 = day2::part2(scroll2); // 687 too low, 701 too high
  println!("Day 2: {} + {}", result2_1, result2_2);

  let scroll3 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.3"));
  let result3_1 = day3::main(scroll3.clone());
  let result3_2 = day3::part2(scroll3); // 84,836,598 too high
  println!("Day 3: {} + {}", result3_1, result3_2);

  let scroll4 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.4"));
  let result4_1 = day4::main(scroll4.clone()); // 1442 too low, 2529 too high
  let result4_2 = day4::part2(scroll4);
  println!("Day 4: {} + {}", result4_1, result4_2);

  let scroll5 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.5"));
  let result5_1 = day5::main(scroll5.clone());
  let result5_2 = day5::part2(scroll5);
  println!("Day 5: {} + {}", result5_1, result5_2);

  let scroll6 = phext::fetch(problems.as_str(), phext::to_coordinate("1.1.1/1.1.1/1.1.6"));
  let result6_1 = day6::main(scroll6.clone());
  let result6_2 = day6::part2(scroll6);
  println!("Day 6: {} + {}", result6_1, result6_2);
}
