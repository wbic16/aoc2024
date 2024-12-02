Advent of Code: 2024
--------------------

# Day 1: Historian Hysteria
https://adventofcode.com/2024/day/1

## Lessons

The first day provides us with a text list of 2,000 numbers arranged into 1,000 pairs of lines.

Gold Star 1: We sort the lists and calculate the distance between the elements: 765748.
Gold Star 2: Next up, we need to calculate a similarity score: 27732508.

In order to solve today's puzzle, you need to make use of conversions from text. I earned two gold stars with 68 lines of Rust.

* `fs::read_to_string`: Slurps content from disk into RAM
* `phext::fetch`: Only being used to allow me to shill Phext so far (problems.phext)
* `split_whitespace`: Splits an input string
* `Vec<i32>`: Dynamic array of integers
* `parse::<i32>`: Converts from text to a number
