# [Day 2](https://adventofcode.com/2024/day/2)

First thing I want to do is organize the code into modules, this way we can copy the `adventofcode` directory over to each new day's directory as we move forward.

## Part 1

This differs from Day 1 in that we're not guaranteed the same _number_ of entries per line, so the best bet will be to read the data in line by line as a **string** (_not_ as a csv), then split the string on the `b' '` space delimiter and store that in a vector.