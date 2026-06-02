# Right Triangles with Integer Coordinates (91)

In [Problem 91](https://projecteuler.net/problem=91), we're asked to find right triangles on a grid of integer coordinates.

## Brute force

The coordinates in the grid extend from 0 to 51 for both $x$ and $y$, one point is fixed at the origin. Hence we have $51^4 = 6\,765\,201$. If we can make the check fast enough, we can just brute force this.

The check whether something is a right triangle is rather easy: Compute the three side lengths squared:
$$
\begin{aligned}
s_1^2 &:= x_1^2 + y_1^2 \,, \\\\
s_2^2 &:= x_2^2 + y_2^2 \,, \\\\
s_3^2 &:= (x_1 - x_2)^2 + (y_1 - y_2)^2 \,.
\end{aligned}
$$

Then find the longest side length and call it $h$, call the other side lengths $a$ and $o$. One has a right triangle if $h^2 = a^2 + o^2$.

One has to be a bit careful not to count triangles twice when $(x_1, y_1)$ and $(x_2, y_2)$ are exchanged.

My Rust implementation runs in 39 ms, so clearly fast enough. Perhaps there is a more clever way to this, but one doesn't need to.