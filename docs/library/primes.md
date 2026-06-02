# Primes

This library contains functionality to work with primes and related concepts. Factorizing integers into their prime factors is another big thing.

## Factorizations

Let us take $n = 24$, which we can factor into $2^3 \times 3$. Then we can build the following search tree by taking any remaining divisor within the current branch. We also enforce that the following divisors must not be greater than the preceding one to avoid ordering issues.

This gives us the following search tree:

![](images/drawing-88-008.png)

Hence we have these factorizations:

- 24
- 12 × 2
- 8 × 3
- 6 × 4
- 6 × 2 × 2
- 4 × 3 × 2
- 3 × 2 × 2 × 2