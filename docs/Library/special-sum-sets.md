# Special Sum Sets

In multiple related problems, we work with *special sum sets* that are quite non-trivial to understand and we need auxiliary definitions to define all the properties.

Let us first define the sum:

::: definition Set Sum
Let $A$ be a finite set of natural numbers, $A \subset \mathbb N$. Then $S(A)$ is the sum of all numbers in that set.
:::

Then we define a *sum-distinct* set:

::: definition Sum-Distinct Set
Let $A \subset \mathbb N$ be a finite set. In a *sum-distinct set* (SDS), for any two non-empty disjoint subsets $B$ and $C$, their set sums are different:
$$ S(B) \neq S(C) \,. $$
:::

And the size-monotone set:

::: definition Size-Monotone Set
Let $A \subset \mathbb N$ be a finite set. In a *size-monotone set* (SMS), for any two disjoint subsets $B$ and $C$, the set with more elements has a higher sum:
$$ |B| > |C| \implies S(B) > S(C) \,. $$
:::

With these parts, we can then assemble the main definition:

::: definition Special Sum Set
A *special sum set* (SSS) is a set that is both a sum-distinct and size-monotone set.
:::

In [Problem 103](https://projecteuler.net/problem=103), we also find the definition of the optimal SSS:

::: definition Optimal Special Sum Set
For given $n$, the set $A$ with $|A| = n$ and minimal $S(A)$ is called *optimal special sum set* (OSSS).
:::

This is not necessarily unique for large $n$, though.

## Restriction to disjoint sets

The definitions of SDS and SMS are restricted to disjoint sets. That restriction doesn't restrict the sets much, it comes naturally. Common elements would just increase both sums and could be cancelled. 

::: theorem Disjoint Sets Equivalent
We could drop the rule that sets $B$ and $C$ needs to be disjoint and it wouldn't make a difference.
:::

::: proof
Let's take $A = \{1, 2, 3\}$. If we take $B = \{1, 2\}$ and $C = \{2, 3\}$, we are comparing whether $1 + 2 \neq 2 + 3$, which simplifies to $1 \neq 3$ by subtracting the common elements from both sides.
:::

## Easier size monotonicity

Testing whether a set is size-monotone is hard with the definition. There is an equivalent and easier way:

::: theorem Largest vs Smallest
If and only if for all $k$ with $2k + 1 \leq |A|$, the sum of the $k+1$ smallest elements is larger than the sum of the $k$ largest elements, the set is size-monotone.
:::

::: proof
Let $A \subset \mathbb N$ be a finite set with $n = |A|$ elements where for all $k$, the sum of the $k+1$ smallest elements is larger than the sum of the $k$ largest elements. Let $(a_i)_{i=1}^n$ be the sorted series of the elements, so $a_i < a_{i+1}$ for all $i$.

Let $B \subset A$ be the non-empty subset with the $k+1$ smallest elements and $C \subset A$ the non-empty disjoint subset with the $k$ largest elements. By construction, we have $|B| > |C|$.

$B$ contains the smallest elements in the set. We can form a new set $B'$ by swapping elements of $B$ one-to-one with other ones. As only larger elements are available, we have $S(B') \geq S(B)$. $|B'| = k+1$ remains.

We do the same with $C$, where we may substitute elements with smaller ones. We therefore have $C'$ with $S(C') \leq S(C)$. $|C'| = k$ remains.

The set $B'$ contains $k+1$ elements, the set $C'$ contains $k$ elements, hence $|B'| > |C'|$. We may drop elements from $C'$ to form $C''$. We then still have $|B'| > |C''|$ and also $S(C'') \leq S(C')$.

Using the transitivity of the comparison, we can derive
$$ S(B') \geq S(B) > S(C) \geq S(C') \geq S(C'') \,, $$
which simplifies to
$$ S(B') > S(C'') \,. $$

Also we have held the constraint
$$ |B'| = |B| > |C| = |C'| \geq |C''| \,, $$
which simplifies to
$$ |B'| > |C''| \,. $$

We could even add unused elements to $B'$ to make it even larger and the sum larger.

Because we allowed to swap all elements in both sets and allowed $C''$ to be smaller than $k$, and we allowed adding elements to $B'$, we can form any set $B'$ with $k+1$ or more elements and set $C''$ with $k$ or less elements. We are able to choose any $k$. Therefore this applies to all sets. Hence the forward direction of the equivalence is proven.

**Backwards direction:** We assume that we have a size-monotone set by the initial definition. That means we can pick any two sets $B$ and $C$ with $|B| > |C|$ and know that $S(B) > S(C)$. We can pick $B$ to contain the $k+1$ smallest elements and $C$ to contain the $k$ largest elements.

As either condition implies the other, the equivalence is proven.
:::

## Sum-distinctness follows partially from size-monotonicity

We have defined sum-distinctness and size-monotonicity separately. But there is one implication that we can make.

::: theorem
If and only if a set is an SSS, it will be size-monotone for all subsets and sum-distinct for equal size subsets.
:::

::: proof
The forward direction is trivial because the definition of SSS includes size-monotonicity and sum-distinctness for all subsets, therefore equal size subsets are included.

**Backward direction:** Let $A$ be a set that is size-monotone for all subsets and sum-distinct for equal size subsets. Let $B$ and $C$ be arbitrary-sized disjoint non-empty subsets. If $|B| = |C|$, we know that these are sum-distinct by construction. If $|B| \neq |C|$, the size-monotonicity trait tells us that $S(B) \neq S(C)$. Therefore the set $A$ is sum-distinct for *all* subsets.

As either condition implies the other, the equivalence is proven.
:::

## Construction of larger special sum sets

Within the problem statement of [Problem 103](https://projecteuler.net/problem=103), we are given a way to use a SSS with $k$ elements to construct a SSS with $k+1$ elements. There a rough definition of a “middle element” that we are going to make more explicit:

::: definition Pivot Element
In an SSS with $2k$ or $2k+1$ elements, the pivot is the $k+1$ smallest element.
:::

They then suggest this theorem without proof.

::: theorem Next Larger Special Sum Set
Given an SSS with $n$ elements $(a_i)_i$ sorted by size and a pivot element $b$, we can construct an SSS with $n+1$ elements as
$$ \{ b, a_1 + b, \ldots, a_n + b \} \,. $$
:::

We can prove this theorem:

::: proof
Let $A_n$ be an SSS and $A_{n+1}$ a candidate SSS from the prescription.

**Size-monotonicity:**
Let $(a_i)_i$ be the members of an SSS $A_n$. By construction we know that
$$ \forall k \colon \sum_{i=1}^{k+1} a_i > \sum_{j=1}^k a_{n-j+1} \,. $$

We can split off the largest summand from that sum and have
$$ \sum_{i=1}^k a_i + a_{k+1} > \sum_{j=1}^k a_{n-j+1} \,. $$

For a set $A_n$ with uneven $n$, when $k$ takes its largest possible value, $a_{k+1}$ is the pivot element. Otherwise it is smaller than the pivot element. As the left side is already bigger than the right, we can use $b \geq a_{k+1}$ and then write this like so:
$$ b + \sum_{i=1}^k a_i > \sum_{j=1}^k a_{n-j+1} \,. $$

On both sides, we can add $kb$ by adding it to the summands in the sum:
$$ b + \sum_{i=1}^k (a_i + b) > \sum_{j=1}^k (a_{n-j+1} + b) \,. $$

And this is the definition for the size-monotone expression of set $A_{n+1}$. Therefore $A_{n+1}$ is size-monotone as well.

**Sum-distinctness:**
As $A_{n+1}$ is size-monotone, we only need to look at the equal size subset case for the sum-distinctness. Let $B$ and $C$ be disjoint non-empty subsets of size $m$.

*Pivot outside subsets:*
If $b$ is in neither $B$ and $C$, we can subtract $b$ from each element to form the subtracted sets $B'$ and $C'$. With the sums, we know that
$$ S(B) = S(B') + |B| b \,, \quad S(C) = S(C') + |C| b \,. $$

As we only need to look at $|B| = |C|$, we can subtract the same multiple of $b$ from both sides and then use $S(B') \neq S(C')$ to show that also $S(B) \neq S(C)$.

*Pivot in subset*:
Let $b$ in $B$ without loss of generality. We define $B'$ to be the set without $b$ and with $b$ subtracted from the other elements. Let $C'$ be $C$ with $b$ subtracted from all elements. We then have
$$ S(B) = b + S(B') + (m-1) b = S(B') + m b \,, \quad S(C) = S(C') + m b \,. $$

We can write the difference between the set sums as
$$ S(B) - S(C) = S(B') - S(C') \,. $$

As $|C'| = m$ and $|B'| = m-1$, we can apply the size-monotonicity of $A_n$ and know that $S(B') - S(C') < 0$ due to $|B'| < |C'|. This proves the sum-distinctness.
:::

As we have used $b \geq a_{k+1}$, we learned that we could have taken a bigger $b$ and the prove would have still succeeded. The choice of the pivot element is the smallest element to generate a new SSS.

In the problem statement, it is made clear that although this prescription provides an SSS, it does not necessarily yield an optimal SSS. Nevertheless, we can use this prescription as an upper bound for the sum of elements $S(A_n)$ an optimal $A_n$ would have.

## Conway-Guy sequence

Through some AI interaction, I was informed about the Conway-Guy sequence that is also registered as [OEIS A005318](https://oeis.org/A005318):

::: definition Conway-Guy Sequence
Let $a_0 := 0$, $a_1 := 1$, and
$$a_{n+1} := 2 a_n - a_t \,, \quad t := n - \left\lfloor \tfrac12 + \sqrt{2n} \right\rfloor \,. $$
:::

It starts with 0, 1, 2, 4, 7, 13, 24, 44, 84, 161, 309, 594.

From this, for a target size $n$, we can build the set:

::: theorem
The prescription
$$ A_n = \{ a_n - a_{n-i} | 1 \le i \le n \} $$
builds an SSS $A_n$ which is *close to* optimal.
:::

However, this doesn't yield better results than the extension with the pivot element. It seems to be just a different way of constructing it.