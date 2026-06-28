# Special Sum Sets

In multiple related problems, we work with *special sum sets* that are quite are non-trivial to understand and we need auxiliary definitions to define all the properties.

Let us first define the sum:

::: definition Set Sum
Let $A$ be a finite set of natural numbers, $A \subseteq \mathbb N$. Then $S(A)$ is the sum of all numbers in that set.
:::

Then we define a *sum-distinct* set:

::: definition Sum-Distinct Set
Let $A \subseteq \mathbb N$ be a finite set. In a *sum-distinct set*, for any two disjoint subsets $B$ and $C$, their set sums are different:
$$ \forall B \subset A, C \subset A, B \cap C = \emptyset \colon S(B) \neq S(C) \,. $$
:::

And the size-monotone set:

::: definition Size-Monotone Set
Let $A \subseteq \mathbb N$ be a finite set. In a *size-monotone set*, for any two disjoint subsets $B$ and $C$, the set with more elements has a higher sum:

With that we can then come to the main definition:
$$ \forall B \subset A, C \subset A, B \cap C = \emptyset, |B| > |C| \colon S(B) > S(C) \,. $$
:::

With these parts, we can then assemble 

::: definition Special Sum Set
A *special sum set* is a set that is both a sum-distinct and size-monotone set.
:::