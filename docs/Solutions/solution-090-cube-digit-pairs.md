# Cube Digit Pairs (90)

In [Problem 90](https://projecteuler.net/problem=90), we're asked to find ways to print digits onto two cubes to represent two-digit numbers.

We can print the digits 0 to 9 onto each cube, so we need to chose 6 out of 10 without replacement. This gives us a rather sensible number of combinations:
$$ \binom{10}{6} = 210 \,. $$

With two cubes, we have $210^2 = 44\,100$ combinations. That is not too much to simply try all of them.

## Formulating the criterion

The numbers that we have to represent is the set {01, 04, 09, 16, 25, 36, 49, 64, 81}, the square numbers below 100. To represent these numbers, we need to make sure that for every number, each digit is present on one of the cubes. We can swap the cubes if needed, there is no need to have all the first digits on the first cube.

And then there is the additional complication that one can use a 6 as a 9, but one has to count them as distinct sets.

I end up with the following algorithm:

- Iterate through all ways we can take 6 out of 10 digits for each cube …
    - Discard the combination if we already had the same cubes but swapped.
    - If one of the digit sets contains the 6 or 9, add the other one to the set. This accounts for the 6-9-rule.
    - Iterate through all the numbers to represent …
        - Check whether the first digit of the number is in the first cube set and the second digit is in the second cube set (or the other way around).
        - If we cannot represent this number with the current cubes, discard this cube combination.
    - If we could represent all the numbers, count the combination towards the result.

## Bit masks

In my Rust implementation, I have chosen to represent the cube sets using bit masks. There are 10 digits, so a 16-bit integer is already sufficient. Checking for membership of a digit in the set is a binary _and_ operation. Adding numbers to the set is an _assign-or_ operation.