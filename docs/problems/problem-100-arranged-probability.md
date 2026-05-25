# Arranged Probability (Problem 100)

In a box that contains $b$ blue and $r$ red disks initially, the chance of drawing two blue disks is $P(BB) = (b/(b+r)) (b-1)/(b-1+r)$.

In the example they take $b = 15$ and $r = 6$ to yield $15/21 \times 14/20 = 1/2$.

We also get exactly 1/2 when one takes $r = 85$ and $r = 35$. We're now tasked with finding a configuration where the probability is also exactly 1/2 but the total number of disks exceeds $10^{12}$. The answer for that problem is the number of blue disks $b$ in that configuration.

# Thoughts

We can rearrange the equation a little bit such that we don't have to divide:
$$ 2 b (b-1) = (b+r) (b+r-1) \,. $$

We can also introduce $n := b + r$ to make the notation more concise and aligned with what we're actually looking for. We exchange number of red disks with the total ones as a parameter.
$$ 2 b (b-1) = n (n-1) \,. $$

This equation looks pretty symmetric, except for the 2 on the left side. That's interesting.

We now want to find the smallest $n$ above $10^{12}$ where this equation has a integer solution for $b$. The ceiling is certainly chosen such that simply trying all combinations doesn't work out.

The $n (n-1)$ reminds of the triangle number. Going back to Problem 12, we have the definition of triangle numbers as $T_a := a * (a + 1) / 2$. And we can get the divisors using the coprime technique.

We can shift the index and then we introduce $a := n - 1$. Then the equation becomes $n (n-1) / 2$ to generate triangle number $n+1$. We can also plug that into the above equation to get this:
$$ 2 T_{b-1} = T_{n-1} \,. $$

With that, the problem boils down to finding the smallest triangle number $T_n$ with $n > 10^{12}$ which is double another triangle number.