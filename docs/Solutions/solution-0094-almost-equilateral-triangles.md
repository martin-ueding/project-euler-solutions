# Almost Equilateral Triangles (94)

In [Problem 94](https://projecteuler.net/problem=94), we're tasked to find “almost equilateral” triangles with side lengths either $(n, n, n-1)$ or $(n, n, n+1)$ with $n \in \mathbb N$ and an integer area. As an example, they give $(5, 5, 6)$ which has area 6. The answer will be the sum of all perimeters of all such triangles as long as the perimeter is below one billion.

## Equilateral triangle height and area

Let's collect the equations for the equilateral triangle from school. Let's say that our triangle has two sides with $a \in \mathbb N$ and a base $b \in \mathbb N$. Then the height $h$ can be constructed from the Pythagorean theorem as
$$ h^2 = a^2 - \frac{b^2}{4} \,. $$

The area of such a triangle is
$$ A = \frac 12 b h = \frac 12 b \sqrt{a^2 - \frac{b^2}{4}} \,. $$

## Integer equilateral triangle with integer area

Right as the first sentence, they give this theorem:

> It is easily proved that no equilateral triangle exists with integral length sides and integral area.

In math, “easily” doesn't necessarily mean that it is easy for the reader, just that it doesn't require more advanced math than we already have. And it turns out that it is indeed not involved.

We can just set $b = a$ and look at the area:
$$ A = \frac 12 a \sqrt{a^2 - \frac{a^2}{4}} \,. $$

This can be simplified by pulling out $a^2$ and we end up with
$$ A = \frac{\sqrt{3}}{4} a^2 \,. $$

Because $\sqrt 3$ is an irrational number and everything else are integers, we will never be able to have an integer $A$.

## The “almost equilateral triangles”

We now look at the case where $b = a \pm 1$. This brings the area to
$$ A = \frac 12 (a \pm 1) \sqrt{a^2 - \frac{a \pm 1^2}{4}} \,. $$

We need to find integer areas. Let us pull our the fractions out of the radicand such that we don't need to consider them there. This gives us the simpler expression
$$ A = \frac 14 (a \pm 1) \sqrt{4 a^2 - (a \pm 1)^2} \,. $$

In order to have an integer area $A$, we need to have the radicand be a square number. Multiplied with $a \pm 1$, it must be divisible by 4. Let us tackle the search for a square radicand first.

Let us call the radicand $y^2$ with $y \in \mathbb N$. Then we write this as
$$ 4 a^2 - (a \pm 1)^2 = y^2 \,. $$

From here we do some manipulations, multiply by 3 and complete the square. We end up with
$$ (3a \mp 1)^2 - 3y^2 = 4 \,. $$

We introduce $x := 3a \mp 1$ and then end up with
$$ x^2 - 3y^2 = 4 \,. $$

This type of equation has occurred multiple times in these problems and it is the Diophantine equation that has it's own [library page](../Library/diophantine-equations.md). In this case we have $D = 3$ and $c = 4$.

## Solutions of the Diophantine equation

As written in the library page, we can generate solutions for this equation systematically. We just need to convert the $x$ back to $a$ using
$$ a = \frac{x \pm 1}{3} $$
if $x \pm 1$ is actually divisible by 3.

Then we also need to check that $(a \pm 1) y$ is divisible by 4, otherwise we wouldn't get an integer area $A = (a \pm 1) y / 4$.

The perimeter of such a triangle is $p = 3 a \pm 1$. We sum the perimeters until we have exceeded the threshold of $10^9$.

As the solutions are generated in such an efficient way with the recursion relation, this takes 494 ns with my Rust implementation.