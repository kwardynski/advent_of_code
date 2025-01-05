# [Day 2](https://adventofcode.com/2024/day/2)

## Part 1

This differs from Day 1 in that we're not guaranteed the same _number_ of entries per line, so the best bet will be to read the data in line by line as a **string** (_not_ as a csv), then split the string on the `b' '` space delimiter and store that in a vector.

### Notes:
- We can use the `windows()` function to slice up our vectors safely
- Since we're doing subtraction on integers, we can't store them as unsigned types or else the compiler will panic (and rightfully so)
- Converting the `diff_vector` into an iterator first, then doing 3 `all()` checks on that yields invalid results, I think the `all()` function actually mutates the iterator. To work around that, I just called `diff_vector.clone().to_iter().all()` for each check, that seems like a weird thing to do but it provides the right answer.

## Part 2

Now we can consider a previously "unsafe" level "safe" if it passes all checks after removing 1 number.

My initial assumption is that we can calculate the number of offending unsafe steps, if there are 1 or 2 violations then we can try and remove the values _around_ the violations to see if the result passes.

This might be a little tougher to do because something like `[4 5 0 6 7]` has 2 violations depending on how you look at it so I think to start what I'll do is the brute force solution of attempting a safety check for each removed element and short circuit on the first passed combination.