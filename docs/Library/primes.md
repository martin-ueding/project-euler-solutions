# Primes

This library contains functionality to work with primes and related concepts. Factorizing integers into their prime factors is another big thing.

## Computing primes

### Iterative method

If we don't have an upper bound for primes, we need to compute them as we go. The algorithm for that is very straightforward:

- Create an empty list that will hold the primes.
- Iterate through numbers from 2, these will be our candidate $c$.
    - Take all previous primes $p$:
        - When $c$ is divisible by $p$, $c$ cannot be a prime number and we proceed to the next candidate.
        - When we have reached $p$ such that $p^2 > c$, then $c$ doesn't have any divisors and must be prime.
    - Add the candidate to the list of primes.

As we're testing division by already known primes, we can proceed rather quickly. The $p^2 \lt c$ constraint makes it even faster.

If one needs to go through the primes a second time, one can reuse the list with primes. Only when that is exhausted, one needs to compute more of them.

### Sieve

If the ceiling is known, one can use the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).

## Prime factor decomposition

We can decompose every integer into a product of primes. For instance, $24 = 2^3 \cdot 3^1$. An efficient way to compute the decomposition is by taking the number and dividing out the primes, starting from the smallest prime:

- Start with 24.
- Test whether 24 is divisible by 2, the smallest prime.
- It is divisible, hence we write this as $24 = 2^1 \cdot 12$.
- We test 12 for divisibility by 2 again, extracting another one to yield $24 = 2^2 \cdot 6$.
- We can pull out another factor of 2: $24 = 2^3 \cdot 3$.
- Now the remainder is 3, we cannot divide by 2 again. We go to the next prime, 3.
- We find that the remainder is divisible by this prime and pull that out: $24 = 2^3 \cdot 3^1 \cdot 1$.
- The remainder is 1, hence we're done.

## Divisors

In order to get all possible divisors of a number, we first do the prime factor decomposition. For instance we take $24 = 2^3 \cdot 3^1$.

In order to construct all divisors, we take all possible combinations of multiplicities. We construct these with the Cartesian product of all counts. Let the number of distinct primes be $k$. Let $p_i$ be the primes that are present (here $p_1 = 2$, $p_2 = 3$). Let $m_i$ be their multiplicities ($m_1 = 3$, $m_2 = 1$). Then we compute all divisors as
$$ \left\{ \prod_{i = 1}^k p_i^{l_i} \middle| 0 \leq l_i \leq m_i \text{ for } i = 1, \ldots, k \right\} \,. $$

## Factorizations

Let us take $n = 24$, which we can factor into $2^3 \times 3$. Then we can build the following search tree by taking any remaining divisor within the current branch. We also enforce that the following divisors must not be greater than the preceding one to avoid duplicates.

This gives us the following search tree:

![](factorization-search-tree.png)

This can be traversed by using a recursive function.

Hence we have these factorizations:

- 24
- 12 × 2
- 8 × 3
- 6 × 4
- 6 × 2 × 2
- 4 × 3 × 2
- 3 × 2 × 2 × 2

We can see that there is a recurring tail in these factorizations. Here we have factorized 4 multiple times. Hence it makes sense to cache the results if one wants to factorize a lot of numbers.