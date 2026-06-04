# Odd Period Square Roots (64)

In [Problem 64](https://projecteuler.net/problem=64), we're asked to look at the continued fractions of the square roots of natural numbers N$N and investigate their the periodic part of the expansion. Specifically, we are asked how many expansion of $\sqrt N$ for $N \leq 10\,000$ have an odd period.

With the [library for continued fractions](../Library/continued-fractions.md), we can directly get the prefix and period part for every $N$. We just check for odd length and count these.