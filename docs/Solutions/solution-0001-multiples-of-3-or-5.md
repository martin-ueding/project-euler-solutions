# Multiples of 3 or 5 (1)

In [Problem 1](https://projecteuler.net/problem=1), we're asked to find the sum of all multiple of 3 or 5 below 1000.

There are many ways to approach this problem. A simple one is to iterate through all numbers below 1000, check whether they are divisible by 3 or 5, and sum them up.

Another way is to sum all the multiple of 3 below 1000, add all the multiples of 5 below 1000. But then we have counted the numbers that are divisible by both 3 and 5 twice. Luckily 3 and 5 are both primes and have no common denominator, hence all numbers that are divisible by both are divisible by their product, 15. We can subtract all multiples of 15 to correct for the overcounting.

But there are [closed-form expressions](https://www.cuemath.com/sum-of-natural-numbers-formula/) for the sum of natural numbers. We have a first element $f=1$, an end $e$ (exclusive) and a step $s$. We then define the number of terms in our series as
$$ n := \left\lfloor \frac{(e - 1)}{s} \right\rfloor \,. $$

And then we can define the sum $S$ as
$$ S(e, s) := \frac{n (n - 1) s}{2} \,. $$

The solution to the problem then is adding the multiples of 3 and 5, then subtracting the multiples of 15:
$$ S(1000, 3) + S(1000, 5) - S(1000, 15) \,. $$