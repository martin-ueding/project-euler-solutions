# Product-Sum Numbers (Problem 88)

For every $k$ in $2 \le k \le 12,000$, we're asked to find the minimal product-sum number with $k$ factors and summands.

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