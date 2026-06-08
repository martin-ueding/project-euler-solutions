# Triangle Containment (102)

In [Problem 102](https://projecteuler.net/problem=102), we're given a list of vertices of triangles. For each triangle, we're supposed to check whether the coordinate origin is contained within the triangle and count these.

There are many different ways to check whether a point is contained in a triangle. We will use cross products. Let $O$ be the coordinate origin, $A$, $B$ and $C$ the vertices of the triangle. We can define vectors along the edges, $AB$, $BC$, and $CA$. We can also define vectors from the origin to the vertices: $OA$, $OB$, $OC$.

Without loss of generality, let us assume that the vertices are arranged in a counterclockwise fashion. If the origin is within the triangle, the connections from one vertex to the origin will always be left of the connection to the next vertex. This holds for all three vertices. If the origin is outside, some of these origin connections will be on the left, others on the right.

We can express this “left or right” relation with a cross product. Since we're in two dimensions only, we only need to look at the third component of the cross product. For two vectors $\vec a$ and $\vec b$, the cross product vector $\vec c$ is defined as
$$ c_i = \epsilon_{ijk} a_j k_j \,, $$
where $\epsilon_{ijk}$ is the [Levi-Civita symbol](https://en.wikipedia.org/wiki/Levi-Civita_symbol) and [summation convention](https://en.wikipedia.org/wiki/Einstein_notation) is assumed.

In our concrete case we're interested in $c_3$:
$$ c_3 = a_1 b_2 - a_2 b_1 \,. $$

Setting $\vec a = OA$ and $\vec b = AB$, or a similar pattern, with all three vertices, we can form three such cross products. If all three cross products have the same sign, we know that the origin has to be within the triangle. If the signs differ, it must be outside.