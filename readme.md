# Collection of solutions for Advent of Code 2025
This year I decided to make the solutions in Rust. The biggest challenge will probably be getting used to the syntax and using Rust on Linux (just getting started here as well).

## Day 1
This is a simple exercise for circular buffer. For part 1 just checking if there is a `zero` in the sequence is enough. But for part 2, you need to check all overflows.

The naive solution would be implementing it step by step and each step check if we are at 0. Perfectly doable for the size of the input. Instead though, we can get how many revolutions we did and then use the remainder to move (and again check if cross the boundary).

## Day 2
Here we have to find sequences in a number. For the first part it's easy - to be a duplicated sequence, it needs even number of digits. To get digits from a decimal number, we can just divide it by 10 till we hit 0. Then modulo to get one part and divide to get the second. Compare and if equals, that is a sequence.

The part 2 is similar, but we have to find all occurences. Aha! So this is recursive. We don't split in middle but try to find the pattern. For me the biggest gotcha was even number, because for example 1_001_001 was getting matched. I only needed to check the remaining length to match the pattern (here it's 001).

## Day 3
Part 1 was about finding two largest numbers. The order might be an issue, so we can find the largest and if it's at the very end (like 8119) then take the first second largest number.

So here for part 2 I decided to get smart. That was a bad decision. If you wanna get smart with these tasks, make sure to handle all edge cases in your algorithm. That wasn't the case, so just use memoization and brute force it. Which works on test data, but is slow for input data...

Turns out, that sorting it lexicographicly (linear time) is the correct solution here. So you basically go over the values in order, and if you encounter bigger value then the last one, try to propagate it (if using stack, then it's popping the stack) until the value in front is not bigger and if we have enough remaining values.