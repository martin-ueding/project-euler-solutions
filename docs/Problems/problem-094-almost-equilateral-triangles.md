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

We need to find integer areas. 