# Special Subset Sums: Optimum (103)

In [Problem 103](https://projecteuler.net/problem=103), we're asked to find a an [optimal special sum set](../Library/special-sum-sets.md) (SSS).

My solution doesn't seem elegant as it is just a depth-first search in the members of the set. Whenever the sum of the set exceeds the sum of the constructed SSS, it is clear that the current search path is not on the optimal SSS.

I start with the solution $A_2 = \{1, 2\}$, compute the candidate $A_3$ with the pivot element to get an upper bound for the optimal $S(A_3)$. With that ceiling, I start a set with $\{a_1, a_2\}$ and try all possible $a_3$ that would make the set size-monotone and sum-distinct. For the first promising $a_3$, I recursively try all $a_4$. This is done until either the ceiling is crossed or the desired size $n$ is reached. In the former case, the recursion is aborted, in the latter the candidate is returned. At each step, only the candidate with the lowest sum is retained.

This takes quite a while to compute and hence doesn't feel like the right approach here.