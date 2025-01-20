# [Day 3](https://adventofcode.com/2024/day/3)

## Part 1

Yikes. So knee jerk reaction is that regex is not going to work since there's lots of "permutations" of correct inputs, so once again I'm going to just take a brute force approach to this and see what happens.

The input is 19,256 characters long, we're not really at risk of hurting ourselves. If it was longer, or we didn't know what the input was actually going to be, I would take a different approach (think streaming character by character until the `mul(` pattern is reached)

What I think I'm going to do is:

- Split the input string by `mul(`
- Iterate over those substrings, split them by `)` and take the first substring
- Split that substring by a `,`, and check if it has exactly 2 elements
- If so, check if both elements can safely be parsed into integers
- If so, multiply them and add to the accumulator

For example, if one of the substrings after the `mul(` split is `"123,456)someOtherS)tuff"`, processing it would look like:

1. Split by `)` -> `["123,456", "SomeOTherS", "tuff"]`
2. Take the first element -> `"123,456"`
3. Split by `,` -> `["123", "456"]`
4. This has exactly 2 elements which can safely be parsed into ints, so multiply them