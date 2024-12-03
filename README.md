# Advent of Code 2024

Issue with Day 2 part 2 is that I'm determining if the sequence of numbers is increasing or decreasing off of the first two values, but if the problem number to remove is in the 2nd slot it won't detect it because its actually setting the problem number as the 3rd slot i.e., the opposite direction it determined initiality

[2 1 3 5 8] -> Currently removes 3rd slot (3), giving us an unsafe input of [2 1 5 8] (unsafe because its changing from desc to asc)
[2 1 3 5 8] -> Should Remove 2nd slot (1) would be a safe input [2 3 5 8]