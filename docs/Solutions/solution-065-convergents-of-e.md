# Convergents of e (65)

In [Problem 65](https://projecteuler.net/problem=65), we're told that Euler's number $\mathrm e$ can be represented as a continued fraction with coefficients
$$ e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, \ldots , 1, 2k, 1, \ldots] \,. $$

We're asked to find the sum of the digits in the numerator of the 100th convergent of this number.

Using the [library functions for continued fractions](../Library/continued-fractions.md), we only need to implement the sequence of coefficients to give us the 100 coefficients needed. Then we simplify the fraction, take the numerator and then take the digit sum.