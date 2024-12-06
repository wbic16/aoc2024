# Advent of Code: 2024


## Day 1: Historian Hysteria
https://adventofcode.com/2024/day/1

## Lessons

The first day provides us with a text list of 2,000 numbers arranged into 1,000 pairs of lines.

* Gold Star 1: We sort the lists and calculate the distance between the elements: 765,748.
* Gold Star 2: Next up, we need to calculate a similarity score: 27,732,508.

In order to solve today's puzzle, you need to make use of conversions from text. I earned two gold stars with 68 lines of Rust.

* `fs::read_to_string`: Slurps content from disk into RAM
* `phext::fetch`: Only being used to allow me to shill Phext so far (problems.phext)
* `split_whitespace`: Splits an input string
* `Vec<i32>`: Dynamic array of integers
* `parse::<i32>`: Converts from text to a number


## Day 2: Red-Nosed Reports
https://adventofcode.com/2024/day/2

## Lessons

The second day asks us to start reviewing reactor reports for safe/unsafe operation.

In order to solve today's puzzle, you need to continue learning how to parse/split text into integers.
The second gold star is a bit of an exercise in frustration with edge cases.

* Gold Star 1: In our data set, only 660 of the 1,000 reports are marked safe!
* Gold Star 2: Enter the Problem Dampener - currently unsolved (687-701)


## Day 3: Mull It Over
https://adventofcode.com/2024/day/3

The third day starts to deal with data corruption and more tokenizing.
To solve this puzzle, you need to build a typical tokenizer that tracks state changes.
The problem description is pretty clear - you reset whenever you run across foul input.

The second star ensures that you built a tokenizer.

* Gold Star 1: 183,669,043
* Gold Star 2:  59,097,164

## Day 4: Ceres Search
https://adventofcode.com/2024/day/4

Ceres Search asks us to perform input masking - determining how many instances of XMAS appear.
One way to solve this problem is to generate a few transformations of the input.
Given an input matrix with M columns and rows...we need to transform the input into a normalized form.
We can short-circuit this process by computing offsets from the first character as shown below.

1. Forwards (N, N+1, N+2, N+3)
2. Backwards (N, N-1, N-2, N-3)
3. Rotated -90 Degrees (N, N-M, N-2M, N-3M)
4. Rotated +90 Degrees (N, N+M, N+2M, N+3M)
5. Diagonal 1 (N, N+(M+1), N+2*(M+1), N+3*(M+1))
6. Diagonal 2 (N, N+(M-1), N+2*(M-1), N+3*(M-1))
7. Diagonal 3 (N, N-(M+1), N-2*(M+1), N-3*(M+1))
8. Diagonal 4 (N, N-(M-1), N-2*(M-1), N-3*(M-1))

A given input character is viable if it appears in one of the sequences above.
We'll compute a score for each position in the input.
'X' marks the spot - we only need to search from those points.

Step 1: Scan the input and generate a mask of all potential X positions.
Step 2: For each potential location, perform a lookup to see if we can match an 'MAS' sequence to it.
Step 3: Whenever we identify an XMAS sequence, we'll add it to the X location.

Here's an example:

```
XMASXMAS    2...2... Rules 1 & 5
XMASXMAS => 1...1... Rule 1
XMASXMAS    1...1... Rule 1
XMASXMAS    2...2... Rules 1 & 7
```

In this simplified example, we can see that there are 12 instances of XMAS.

The problem statement was nice enough to hand us a decent case with 18 instances. This is the first problem that I broke out unit tests for...

```
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```

## Day 5
https://adventofcode.com/2024/day/5

Today's puzzle focuses on parsing and validation in two passes.

* Gold Star 1: 5955
* Gold Star 2: 