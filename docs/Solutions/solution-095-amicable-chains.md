# Amicable Chains (95)

In [Problem 95](https://projecteuler.net/problem=95), we look into the numbers that arise when we sum the proper divisors of another number.

Getting the sum of the proper divisors is easy as we already have the [functionality for divisors](../Library/primes.md#divisors). We omit the number itself and sum them up.

## Tracking chains

A number like 6 comes directly back to itself because $1 + 2 + 3 = 6$, so it is a _perfect number_. 25 comes down to 6 because $1 + 5 = 6$. So 25 doesn't start a chain, but it ends up in a chain.

Most other numbers just end up at 0. Once they hit a prime number, the only proper divisor is 1. And as 1 doesn't have proper divisors, we hit 0.

What we can do is just track the numbers that were already visited. Once we hit a number that we already had visited, we know that we have hit a chain. But only when we hit the starting number we really have a closed chain, otherwise we have reached a different chain.

Once we hit 0, we know that all the numbers that we visited will eventually end up with 0. When we try a different starting number and hit such a known number, we can directly abort tracking that chain.

This naive approach took 56 s with some nice Rust code on a CPU from 2019. Given that the problem is from 2005, this would have clearly violated the one-minute-rule.

## Sieve-like approach

Similarly to the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) for primes, we can construct an array where at each index we have the sum of divisors. In order to fill it, we can use a simple algorithm:

- Create an array up to the upper bound of 1,000,000 with zeros.
- For every possible divisor between 1 and 1,000,000:
    - For every possible multiplicator starting from two:
        - Compute the index as divisor × multiplicator.
        - If the index is beyond the array, stop going through the multiplicators.
        - Add the current divisor to the array position at the given index.

In the end one will have the sum of proper divisors at each array position.

In a next step, we can then find the chain lengths using a similar algorithm as before. We track which numbers we have visited. If we reach zero, we know that all these numbers will be eventually zero. If we end up at a previously visited number, we can already stop.

This more intelligent approach runs in 451 ms, which is a factor 125 faster than before.