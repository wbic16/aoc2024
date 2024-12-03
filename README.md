Advent of Code: 2024
--------------------

# Day 1: Historian Hysteria
https://adventofcode.com/2024/day/1

## Lessons

The first day provides us with a text list of 2,000 numbers arranged into 1,000 pairs of lines.

Gold Star 1: We sort the lists and calculate the distance between the elements: 765,748.
Gold Star 2: Next up, we need to calculate a similarity score: 27,732,508.

In order to solve today's puzzle, you need to make use of conversions from text. I earned two gold stars with 68 lines of Rust.

* `fs::read_to_string`: Slurps content from disk into RAM
* `phext::fetch`: Only being used to allow me to shill Phext so far (problems.phext)
* `split_whitespace`: Splits an input string
* `Vec<i32>`: Dynamic array of integers
* `parse::<i32>`: Converts from text to a number

# Day 2: Red-Nosed Reports
https://adventofcode.com/2024/day/2

## Lessons

The second day asks us to start reviewing reactor reports for safe/unsafe operation.

In order to solve today's puzzle, you need to continue learning how to parse/split text into integers.
The second gold star is a bit of an exercise in frustration with edge cases.


Gold Star 1: In our data set, only 660 of the 1,000 reports are marked safe!
Gold Star 2: Enter the Problem Dampener - currently unsolved (687-701)

# Day 3: Pending