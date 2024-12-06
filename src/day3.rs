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

pub fn main(input: String) -> usize {
  let records = day3_tokenize(input, false);
  let mut sum: usize = 0;
  for value in records {
    sum += value as usize;
  }

  return sum;
}

pub fn part2(input: String) -> usize {
  let records = day3_tokenize(input, true);
  let mut sum: usize = 0;
  for value in records {
    sum += value as usize;
  }

  return sum;
}