# Arranged Probability (100)

In [Problem 100](https://projecteuler.net/problem=100), we're asked to work with probabilities and combinatorics.

In a box that contains $b$ blue and $r$ red disks initially, the chance of drawing two blue disks is
$$ P(BB) = (b/(b+r)) (b-1)/(b-1+r) \,.$$

In the example, they take $b = 15$ and $r = 6$ to yield $15/21 \times 14/20 = 1/2$.

We also get exactly 1/2 when one takes $r = 85$ and $r = 35$. We're now tasked with finding a configuration where the probability is also exactly 1/2 but the total number of disks exceeds $10^{12}$. The answer for that problem is the number of blue disks $b$ in that configuration.

## Rearranging the equation

We can rearrange the equation a little bit such that we don't have to divide:
$$ 2 b (b-1) = (b+r) (b+r-1) \,. $$

We can also introduce $n := b + r$ as the total number of disks to make the notation more concise and aligned with what we're actually looking for. We exchange number of red disks with the total ones as a parameter:
$$ 2 b (b-1) = n (n-1) \,. $$

This equation looks pretty symmetric, except for the 2 on the left side. That's interesting.

We now want to find the smallest $n$ above $10^{12}$ where this equation has a integer solution for $b$. The ceiling is certainly chosen such that simply trying all combinations doesn't work out.

## Triangle numbers

The $n (n-1)$ reminds of the triangle number. Going back to Problem 12, we have the definition of triangle numbers as $T_a := a * (a + 1) / 2$. And we can get the divisors using the coprime technique.

We can shift the index and then we introduce $a := n - 1$. Then the equation becomes $n (n-1) / 2$ to generate triangle number $n+1$. We can also plug that into the above equation to get this:
$$ 2 T_{b-1} = T_{n-1} \,. $$

With that, the problem boils down to finding the smallest triangle number $T_n$ with $n > 10^{12}$ which is double another triangle number.

I have done some research on triangle numbers, but it seems that these don't help us so much here.

## Pell's equation

What we can do instead is multiply the equation by 4:
$$ 4(n^2-n) - 8(b^2-b) = 0 \,. $$

Then we complete the square:
$$ (2n-1)^2 - 2 (2b-1)^2 = -1 \,. $$

Finally we introduce the new variables $x := 2n-1$ and $y := 2b-1$ to yield
$$ x^2 - 2y^2 = -1 \,. $$

That is the [negative Pell's equation](https://en.wikipedia.org/wiki/Pell%27s_equation#The_negative_Pell's_equation). We have the case $n = 2$. This particular equation has integer solutions which are similarly constructed as the solution for the Diophantine equation in Problem 66.

Instead of using the continued fractions and their convergents, we can actually start with the fundamental solution of $x = 1$ and $y = 1$ together with the recursion relation
$$ x \to 3 x + y, \qquad y \to 2x + 3y $$
that gives us all other solutions.

The algorithm then becomes extremely simple:

- Start with $x := 1$ and $y := 1$.
- Loop …
    - Compute $n := (x+1)/2$ and $b := (y+1)/2$.
    - If $n > 10^{12}$, return $b$.
    - Apply the recursion relation.

Formulating the solution in this way removes the need to square any of the numbers and hence we can use 64-bit integers without overflow.