# Series

A *sequence* is an ordered collection of numbers. We can, for instance, define a sequence of square numbers $(s_i)_i$ as $s_i := i^2$.

Using this definition, we can define the next interesting concept:

::: definition Series
A *series* is the sequence of partial sums of a sequence. The series $(t_n)_n$ of the sequence $(s_i)_i$ is defined as
$$ t_n := \sum_{i = 1}^n s_i \,. $$
:::

## Natural numbers

A trivial sequence is formed by the natural numbers themselves, $s_i = i$. The series is interesting, because each term is the sum of $n$ natural numbers:
$$ S_n = \sum_{i = 1}^n i \,. $$

There is a clever way to compute this for arbitrary $n$ in closed form:

::: theorem Gauss's Formula
The value of $S_n$ is given by
$$ S_n = \frac{n (n+1)}{2} \,. $$
:::

**Proof:** Writing out the sum, we have
$$ S_n = 1 + 2 + \cdots + (n-1) + n \,. $$

As summation is commutative, we can also write this series out in reverse:
$$ S_n = n + (n-1) + \cdots + 2 + 1 \,. $$

Both sums have the same number of terms and the same value. We can add both lines and match up the element:
$$ 2 S_n = (n+1) + (n+1) + \cdots + (n+1) + (n+1) \,. $$

We have $n$ elements which are $(n+1)$. Hence we can simplify to
$$ 2 S_n = n (n+1) \,. $$

We divide by $2$ to isolate $S_n$ to get the claimed result:
$$ S_n = \frac{n (n+1)}{2} \,. $$

■