# Special Subset Sums: Optimum (103)

In [Problem 103](https://projecteuler.net/problem=103), we're asked to find a an [optimal special sum set](../Library/special-sum-sets.md) (SSS).

I use the pivot element construction to find an SSS with given size $n$. The sum of the elements there serves as an upper bound, no optimal set can exceed that sum.

Then I do a depth-first search. I start a set with $\{a_1, a_2\}$ and try all possible $a_3$ that would make the set size-monotone and sum-distinct. For the first promising $a_3$, I recursively try all $a_4$. This is done until either the sum ceiling is crossed or the desired size $n$ is reached. In the former case, the recursion is aborted, in the latter the candidate is returned. At each step, only the candidate with the lowest sum is retained.

The insights are really only the quick checks to discard all sets that cannot turn out size-monotone or sum-distinct.

This takes 9 s to compute and hence doesn't feel like the right approach here. In the forums, I couldn't find a more clever way to do it. With AI, I couldn't find a more clever way either. Perhaps that is really all to it.