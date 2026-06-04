# Diophantine Equations

In surprisingly many of the problems, we encounter equations of the kind
$$ x^2 - D y^2 = c \,, $$
where $x, y, D, c \in \mathbb Z$. This type of equation essentially describes a hyperbola and we're interested in the intersections with the integer “grid” on the graph. These (and slightly more general ones) are called [Diophantine equations](https://en.wikipedia.org/wiki/Diophantine_equation).

## Finding the minimal solution

We can find the first solution just by trying values of $y$. We manipulate the equation to become
$$ x^2 = D y^2 + c \,. $$

Then we just iterate through $y \in \mathbb N$ and check whether the right side is a perfect square. This can be checked by requiring that all prime factors within that number occur an even number.

Example: For $D = 12$ and $c = 4$, we already find $y = 0$ to be a solution with $x = 2$.

### Special case c = 1

In the special case that we have $c = 1$, we can find the minimal solution faster. Instead of trying out every possible $y$, we instead the [convergents of $\sqrt D$](continued-fractions.md#convergents). Let us denote the $i$-th such convergent as $h_i/k_i$. Then we can test whether $x = h_i$ and $y = k_i$ is a solution. This way we have way less to check.

## Finding more solutions

There is a way to construct more solutions from the initial solution. We will need find a recursion relation. For that we need to introduce a _field norm_ that will help us construct the recursion relation.

### Field norm

Apparently in _algebraic number theory_, there is the concept of a [_field norm_](https://en.wikipedia.org/wiki/Field_norm). At the time of writing, this is all new to me. I will focus on introducing the important concepts here.

Complex numbers can be written as $a + \mathrm i b$ and have a norm $|(a + \mathrm i b)| := a^2 - b^2$. The same concept can be generalized such that we would call this norm $\mathbb Z[\mathrm i]$. The general norm for $\mathbb Z[\sqrt D]$ is defined as
$$ N_{\sqrt D} (a + \sqrt D b) := a^2 - D b^2 \,. $$

We can now reformulate the initial Diophantine equation not as finding a pair $(x, y)$ that solves the equation but rather finding an element $x + \sqrt D y$ that has norm $c$.

### Multiplicative property

Let us take an element $\alpha := (a + \sqrt D b)$ and another $\beta := (c + \sqrt D d)$. We can now multiply these and regroup the terms:
$$ (a + \sqrt D b) (c + \sqrt D d) = (ac + D bd) + \sqrt D (bc + ad) \,. $$

Then we can introduce the following theorem that says that the norm is multiplicative:
$$ N(\alpha) N(\beta) = N(\alpha \beta) \,. $$

This can be shown by inserting everything and simplifying. As an intermediate step we get
$$ (a^2 - D b^2) (c^2 - D d^2) = (ac + D bd)^2 - D (bc + ad)^2 \,. $$

Further simplification shows that both sides are equal and hence the theorem is proven.

### Recursion relation

We had reformulated finding a solution to the Diophantine equation by finding elements $x + \sqrt D y$ that have norm $c$. If we had elements with norm 1, we could multiply our solution element with that element and get a different element with norm $c$. This would also solve the Diophantine equation.

Therefore we want to find solutions to the following equation, which is a special case and called [Pell's equation](https://en.wikipedia.org/wiki/Pell%27s_equation):
$$ x^2 - D y^2 = 1 \,, $$

We can use the more efficient approach with the convergents to find the initial solution here. With $y = 2$ we have $4 \times 12 + 1 = 49$, which is a perfect square. So our solution is $x = 7$ and $y = 2$.

By construction, we know that $N(7 + \sqrt{12} \cdot 2) = 1$. Let us define our initial solution to the Diophantine equation as $\alpha := 2 + \sqrt{12} \cdot 0$ and our non-trivial solution to Pell's equation as $\beta := 7 + \sqrt{12} \cdot 2$. By construction we know that $N(\alpha) = 4$ and $N(\beta) = 1$. Using the multiplicative property, we know that $N(\alpha \beta) = 4$. Actually, $N(\alpha \beta^n) = 4$ for all $n \in \mathbb N_0$ and hence are solution to the Diophantine equation.

We can therefore construct all solutions by computing $\alpha \beta^n$. Using the notation $\alpha := (x + \sqrt D y)$ and $\beta := (\hat x + \sqrt D \hat y)$, we had already defined the multiplication as this:
$$ (x + \sqrt D y) (\hat x + \sqrt d) = (x \hat x + D \hat y y) + \sqrt D (\hat x y + \hat y x) \,. $$

From here, we can formulate a recursion relation:
$$
\begin{aligned}
    x &\to \hat x x + D \hat y y \,, \\\\
    y &\to \hat y x + \hat x y \,.
\end{aligned}
$$

With that we can generate all the solutions from the initial one.