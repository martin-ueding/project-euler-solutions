# Diophantine Equations

In surprisingly many of the problems here, we encounter a certain kind of equation. Let us give it a name:

::: definition Generalized Pell Equation
$$ x^2 - D y^2 = c \quad \text{where} \quad x, y, D, c \in \mathbb Z $$
:::

This type of equation essentially describes a hyperbola and we're interested in the intersections with the integer “grid” on the graph. More general types of this equation have the umbrella term [Diophantine equation](https://en.wikipedia.org/wiki/Diophantine_equation).

## Finding the minimal solution

We can find the first solution just by trying values of $y$. We manipulate the equation to become
$$ x^2 = D y^2 + c \,. $$

Then we just iterate through $y \in \mathbb N$ and check whether the right side is a perfect square. This can be checked by requiring that all prime factors within that number occur an even number.

**Example:** For $D = 12$ and $c = 4$, we already find $y = 0$ to be a solution with $x = 2$.

### Special case c = 1

In the special case that we have $c = 1$, the equation is claled [Pell's equation](https://en.wikipedia.org/wiki/Pell%27s_equation). For this one, we can find the minimal solution faster. Instead of trying out every possible $y$, we instead use the [convergents of $\sqrt D$](continued-fractions.md#convergents). Let us denote the $i$-th such convergent as $h_i/k_i$. Then we can test whether $x = h_i$ and $y = k_i$ is a solution. This way we have way less to check.

## Finding more solutions

There is a way to construct more solutions from the initial solution. We will need find a recursion relation. For that we need to introduce a _field norm_ that will help us construct the recursion relation.

### Field norm

Apparently in _algebraic number theory_, there is the concept of a [_field norm_](https://en.wikipedia.org/wiki/Field_norm). At the time of writing, this is all new to me. I will focus on introducing the important concepts here.

Complex numbers can be written as $a + \mathrm i b$ and have a norm $|(a + \mathrm i b)| := a^2 - b^2$. The same concept can be generalized such that we would call this norm $\mathbb Z[\mathrm i]$. This generalized:

::: definition Field Norm
The general norm for $\mathbb Z[\sqrt D]$ is defined as
$$ N_{\sqrt D} (a + \sqrt D b) := a^2 - D b^2 \,. $$
:::

We can now reformulate the initial Diophantine equation not as finding a pair $(x, y)$ that solves the equation but rather finding an element $x + \sqrt D y$ that has norm $c$.

### Multiplicative property

For the field norm to be useful, we need to look at the multiplication of the elements.

::: theorem Element Multiplication
Let $\alpha := (a + \sqrt D b)$ and $\beta := (c + \sqrt D d)$ be elements. The product of $\alpha$ and $\beta$ gives a new element
$$ \gamma = (ac + D bd) + \sqrt D (bc + ad) \,. $$
:::

**Proof:** We write out the elements and regroup the terms with and without $\sqrt D$ to get the claimed result:
$$ \alpha \beta = (a + \sqrt D b) (c + \sqrt D d) = (ac + D bd) + \sqrt D (bc + ad) \,. $$

---

The field norm has a multiplicative property that will become useful to use later on:

::: theorem Field Norm Multiplication
Let $\alpha := (a + \sqrt D b)$ $\beta := (c + \sqrt D d)$ be such elements. Then the norm of the product is the product of the norms:
$$ N(\alpha \beta) = N(\alpha) N(\beta) \,. $$
:::

**Proof:** This can be shown by inserting everything on both sides of the claim and simplifying. As an intermediate step we get
$$ (a^2 - D b^2) (c^2 - D d^2) = (ac + D bd)^2 - D (bc + ad)^2 \,. $$

Further simplification shows that both sides are equal and hence the theorem is proven.

### Recursion relation

We had reformulated finding a solution to the Diophantine equation by finding elements $x + \sqrt D y$ that have norm $c$. We can use this to obtain a recursion relation.

::: theorem Next Solution
Let (x, y) be a solution to $x^2 - D y^2 = c$. Let $(\hat x, \hat y)$ be a solution to $\hat x^2 - D \hat y^2 = 1$. Then $(\hat x x + D \hat y y, \hat y x + \hat x y)$ is also a solution to the first equation, $x^2 - D y^2 = c$.
:::

**Proof:** Let $\alpha := (x + \sqrt D y)$ be an element with $N_{\sqrt D}(\alpha) = c$, which is equivalent to being a solution to $x^2 - D y^2 = c$. Let $\beta := (\hat x + \sqrt D \hat y)$ be an element with $N_{\sqrt D}(\beta) = 1$ (a solution to Pell's equation with $c = 1$).

By the *field norm multiplication* and using $N(\beta) = 1$, we have
$$ N(\alpha\beta) = N(\alpha) N(\beta) = N(\alpha) \,. $$

As $N(\alpha\beta) = N(\alpha)$, both $\alpha$ and $\alpha\beta$ are solutions to the generalized Pell equation.

Using the *element multiplication, we can write the product of $\alpha\beta$ as
$$ (x + \sqrt D y) (\hat x + \sqrt D \hat y) = (\hat x x + D \hat y y) + \sqrt D (\hat x y + \hat y x) \,. $$

From this we can read off the second solution as claimed.

---

We can apply this approach as many times as we want. This brings us to the recursion relation:

::: theorem Recursion Relation
Let $\alpha$ be a solution to the generalized Pell equation with arbitrary $c$, let $\beta$ be a solution to Pell's equation with $c = 1$. Then we can generate solutions from $\alpha \beta^n$.
:::

**Proof:** We have already shown that $\alpha\beta$ is a solution to the generalized Pell equation as well. We can redefine $\alpha := \alpha\beta$ as the new solution and apply the *next solution* theorem again.

Alternatively we can go back to the *field norm multiplication* and see that
$$ N(\alpha\beta^n) = N(\alpha) N(\beta^n) = N(\alpha) N(\beta)^n = N(\alpha) \cdot 1^n = N(\alpha) \,. $$

This also proves the theorem.

---

This gives us the recursion relation to easily construct arbitrary many solutions to the generalized Pell equation.

We haven't shown that we can generate *all* solutions by using the minimal solution to Pell's equation in the recursion relation. If one applies the recursion with a non-minimal $N(\beta) = 1$ solution, one would skip solutions. It appears as if we do get all solutions, but I don't have a proof.