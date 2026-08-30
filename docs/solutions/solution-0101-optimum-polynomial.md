# Optimum Polynomial (101)

In [Problem 101](https://projecteuler.net/problem=102), we're asked to work with polynomials that are defined by $k$ points.

The main difficulty here seems to be obtaining the polynomial of degree $k-1$ that takes $k$ points into account. One can do this by solving systems of equations, but one can also just take the [Lagrange polynomials](https://en.wikipedia.org/wiki/Lagrange_polynomial).

Given points $(x_i, y_i)$, one defines the polynomial as
$$ L(x) = \sum_{i=1}^k y_i l_i(x) \,, \quad l_i(x) = \prod_{j \neq i} \frac{x - x_j}{x_i - x_j} \,. $$

We can then take the first $k$ points and generate the polynomial $L(x)$ for that. We evaluate it for integers $x \in \mathbb N$ until we find a mismatch between the interpolating polynomial and the given full polynomial. We sum these mismatches and are done.