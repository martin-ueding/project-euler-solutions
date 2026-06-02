# Product-Sum Numbers (88)

In [Problem 88](https://projecteuler.net/problem=88), we're asked to find numbers that can be represented both as a sum and product of a sequence of numbers.

We can reframe the question with more math notation like so: For every $k \in \mathbb N$ with $2 \le k \le 12\,000$, find the positive integer sequence $(a_i)_{i=1}^k$ such that
$$ n_k := \prod_{i=1}^k a_i = \sum_{i=1}^k a_i $$
is minimized. The final solution then is $\sum_{k=2}^{12\,000} n_k$.

## Naive complexity

We have to iterate through 12,000 values of $k$. Then we have to iterate through $n_k$ until we find one where we can find a decomposition $(a_i)_i$ that fulfils the product-sum-relation. In order to find this decomposition, we need to iterate through all possible decompositions.

Even for fixed $k$ and a fixed candidate number, there are lots of factorizations that one could try out. Let us take 12. That can be decomposed as $2^2 \cdot 3$.

If we look at $k = 2$, then this can already be these configurations:

- 1 × 12
- 2 × 6
- 3 × 4

At $k = 3$, we have these:

- 1 × 1 × 12
- 1 × 2 × 6
- 1 × 3 × 4
- 2 × 2 × 3

We essentially have a triple loop here. That is not clever enough to solve this problem.

## Organizing into grids

Looking at $k = 2$, we can organize the two numbers in a grid and see that the sum of the two numbers stays the same on diagonal lines:

![](images/drawing-88-000.png)

