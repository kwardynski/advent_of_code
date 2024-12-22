# [Day 2](https://adventofcode.com/2024/day/2)

## Part 1

This differs from Day 1 in that we're not guaranteed the same _number_ of entries per line, so the best bet will be to read the data in line by line as a **string** (_not_ as a csv), then split the string on the `b' '` space delimiter and store that in a vector.