# Special Sum Sets

In multiple related problems, we work with *special sum sets* that are quite non-trivial to understand and we need auxiliary definitions to define all the properties.

Let us first define the sum:

::: definition Set Sum
Let $A$ be a finite set of natural numbers, $A \subseteq \mathbb N$. Then $S(A)$ is the sum of all numbers in that set.
:::

Then we define a *sum-distinct* set:

::: definition Sum-Distinct Set
Let $A \subseteq \mathbb N$ be a finite set. In a *sum-distinct set* (SDS), for any two non-empty disjoint subsets $B$ and $C$, their set sums are different:
$$ S(B) \neq S(C) \,. $$
:::

And the size-monotone set:

::: definition Size-Monotone Set
Let $A \subseteq \mathbb N$ be a finite set. In a *size-monotone set* (SMS), for any two disjoint subsets $B$ and $C$, the set with more elements has a higher sum:
$$ |B| > |C| \implies S(B) > S(C) \,. $$
:::

With these parts, we can then assemble the main definition:

::: definition Special Sum Set
A *special sum set* (SSS) is a set that is both a sum-distinct and size-monotone set.
:::

## Implications

These definitions have various implications. Let us consider a few things to get a better understanding of the matter.

The definitions of SDS and SMS are restricted to disjoint sets. Dropping this rule wouldn't make a difference because common elements would just increase both sums and could be cancelled. Let's take $A = \{1, 2, 3\}$. If we take $B = \{1, 2\}$ and $C = \{2, 3\}$, we are comparing whether $1 + 2 \neq 2 + 3$, which simplifies to $1 \neq 3$ by subtracting the common elements from both sides.

