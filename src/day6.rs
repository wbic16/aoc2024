pub fn main(_input: String) -> usize {
  return 0;
}

pub fn part2(_input: String) -> usize {
  return 0;
}

fn sample_input() -> String {
  return "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();
}

fn sample_output() -> String {
  return "....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..".to_string();
}

// ^ - up
// > - right
// V - down
// < - left

struct Day6Coord {
  x: isize,
  y: isize
}

fn offset2pos(rows: usize, offset: usize) -> Day6Coord {
  return Day6Coord{ x: (offset % rows) as isize, y: (offset/rows) as isize };
}
fn pos2offset(rows: usize, coord: &Day6Coord) -> usize {
  return (coord.x + (rows as isize) * coord.y) as usize;
}

fn day6_path_transform(input: String) -> String {
  let rows = input.split('\n').count();
  let serialized = input.replace("\n", "");
  // 0-9
  let left = serialized.find('<');
  if left.is_some() {
    let mut lc = offset2pos(rows, left.expect("unexpected"));      
  }
  let up = serialized.find('^');
  let right = serialized.find('>');
  let down = serialized.find('V');

  return "".to_string();
}

#[test]
fn test_visit_locations() {
  
}