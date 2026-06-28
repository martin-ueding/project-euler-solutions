# Multiples of 3 or 5 (1)

In [Problem 1](https://projecteuler.net/problem=1), we're asked to find the sum of all multiple of 3 or 5 below 1000.

There are many ways to approach this problem. A simple one is to iterate through all numbers below 1000, check whether they are divisible by 3 or 5, and sum them up.

## Gauss formula

Another way is to sum all the multiple of 3 below 1000, add all the multiples of 5 below 1000. But then we have counted the numbers that are divisible by both 3 and 5 twice. Luckily 3 and 5 are both primes and have no common denominator, hence all numbers that are divisible by both are divisible by their product, 15. We can subtract all multiples of 15 to correct for the overcounting.

This lets us reframe the problem as
$$ (3 + 6 + \ldots + 999) + (5 + 10 + \ldots + 995) - (15 + 30 + \ldots + 990) \,. $$

From each parentheses, we can factor out the common multiple and we get
$$ 3 (1 + 2 + \ldots + 333) + 5 (1 + 2 + \ldots + 199) - 5 (1 + 2 + \ldots + 66) \,. $$

We can take [Gauss's formula](../Library/series.md#natural-numbers) to write this more concise as
$$ 3 S_{333} + 5 S_{199} - 15 S_{66} \,. $$

That then solves the problem in closed form.