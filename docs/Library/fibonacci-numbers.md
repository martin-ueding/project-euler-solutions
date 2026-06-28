# Fibonacci Numbers

There are various problems that play with the Fibonacci numbers, so we need to look into these as well. They form a [series](series.md) and are defined like this:

::: definition Fibonacci Numbers
The Fibonacci numbers for them infinite natural number sequence $(F_i)_i$ which is defined as
$$ F_1 := 1 \, \quad F_2 := 1, \quad F_n := F_{n-2} + F_{n-1} \,. $$
:::

## Efficient computation

Although the definition is recursive, one shouldn't compute them in a recursive way. Otherwise one will have to recompute terms that one had computed before.

**Example:** Let's compute $F_4$. Then we have the following recursive expansion:

- $F_4$
- $F_2 + F_3$
- $(F_0 + F_1) + (F_1 + F_2)$
- $(F_0 + F_1) + (F_1 + (F_0 + F_1))$

You can see that we need to compute $F_2$ twice and look up $F_0$ twice, $F_1$ three times. When we would compute $F_5$ in this way, we would have to compute $F_3$ and $F_4$ again.

### Iterative algorithm

It is much better to use an iterative algorithm:

- Set $a := 0$, $b := 1$.
- Iterate …
    - Yield $b$.
    - Set $c := a + b$.
    - Set $a := b$.
    - Set $b := c$.

This will yield 1, 1, 2, 3, 5, … as we want. It never recomputes any number.

### Eigenvalue trick

There is an eigenvalue trick that one can use to directly compute any Fibonacci number in closed form:

::: theorem n-th Fibonacci number
The $n$-th Fibonacci number is
$$
F_n =
\frac{1}{\sqrt 5}
\begin{pmatrix}
\frac{\sqrt 5 - 1}{2} & -\frac{\sqrt 5 + 1}{2}
\end{pmatrix}
\begin{pmatrix}
\left( \frac{1 + \sqrt 5}{2} \right)^n & 0 \\
0 & \left( \frac{1 - \sqrt 5}{2} \right)^n \\
\end{pmatrix}
\begin{pmatrix}
1 & \frac{\sqrt 5 + 1}{2} \\
-1 & \frac{\sqrt 5 - 1}{2}
\end{pmatrix}
\begin{pmatrix} 0 \\ 1 \end{pmatrix}
\,.
$$
:::

**Proof:** The above iterative algorithm can also be expressed in matrix form:
$$
\begin{pmatrix} a \\ b \end{pmatrix}
:=
\begin{pmatrix} 0 & 1 \\ 1 & 1 \end{pmatrix}
\begin{pmatrix} a \\ b \end{pmatrix}
$$

We can apply this matrix over and over in order to advance in the algorithm. Hence we can write a closed-form expression:
$$
\begin{pmatrix} F_n \\ F_{n+1} \end{pmatrix}
=
\begin{pmatrix} 0 & 1 \\ 1 & 1 \end{pmatrix}^n
\begin{pmatrix} 0 \\ 1 \end{pmatrix}
$$

In order to compute the power, one could use [exponentiation by squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring). But we can do even better with the eigensystem of the matrix.

Let's call the matrix in the middle $M$. We can find its eigenvalues
$$ \lambda_1 = \frac{1 + \sqrt 5}{2} \,, \quad \lambda_2 = \frac{1 - \sqrt 5}{2} \,. $$
The associated eigenvectors are
$$ v_1 = \begin{pmatrix} \frac{\sqrt 5 - 1}{2} \\ 1 \end{pmatrix} \,, \quad v_2 = \begin{pmatrix} -\frac{\sqrt 5 + 1}{2} \\ 1 \end{pmatrix} \,. $$

We collect these into an eigenvalue matrix $\Lambda$ and an eigenvector matrix $V$:
$$
\Lambda = \begin{pmatrix}
\frac{1 + \sqrt 5}{2} & 0 \\
0 & \frac{1 - \sqrt 5}{2}
\end{pmatrix}
\,, \quad
V = 
\begin{pmatrix}
\frac{\sqrt 5 - 1}{2} & -\frac{\sqrt 5 + 1}{2} \\
1 & 1
\end{pmatrix}
\,.
$$

Hence we can write the matrix $M$ using the eigensystem as
$$ M = V \Lambda V^{-1} \,. $$

The inverse of $V$ is
$$ V^{-1} = \frac{1}{\sqrt 5}
\begin{pmatrix}
1 & \frac{\sqrt 5 + 1}{2} \\
-1 & \frac{\sqrt 5 - 1}{2}
\end{pmatrix} \,. $$

Putting all of that together gives us

$$
\begin{pmatrix} F_n \\ F_{n+1} \end{pmatrix}
=
(V \Lambda V^{-1})^n
\begin{pmatrix} 0 \\ 1 \end{pmatrix} \,.
$$

The real simplification comes from writing out $(V \Lambda V^{-1})^n$ and realizing that $V^{-1} V = 1$. Then we can simplify
$$ (V \Lambda V^{-1})^n = V \Lambda^n V^{-1} \,. $$

Because $\Lambda$ is a diagonal matrix, applying the power to the matrix is the same as applying the power to the elements on the diagonal.

We can then plug in all the expressions and we arrive at
$$
\begin{pmatrix} F_n \\ F_{n+1} \end{pmatrix} =
\frac{1}{\sqrt 5}
\begin{pmatrix}
\frac{\sqrt 5 - 1}{2} & -\frac{\sqrt 5 + 1}{2} \\
1 & 1
\end{pmatrix}
\begin{pmatrix}
\left( \frac{1 + \sqrt 5}{2} \right)^n & 0 \\
0 & \left( \frac{1 - \sqrt 5}{2} \right)^n \\
\end{pmatrix}
\begin{pmatrix}
1 & \frac{\sqrt 5 + 1}{2} \\
-1 & \frac{\sqrt 5 - 1}{2}
\end{pmatrix}
\begin{pmatrix} 0 \\ 1 \end{pmatrix}
\,.
$$

As we're only interested in $F_n$ and not $F_{n+1}$, it is sufficient to use the first row of $V$. And then we arrive at the claimed formula.

■

This expression doesn't work well numerically as one has to project high powers of an irrational number (the $\sqrt 5$) back onto an integer. Eventually, even 64-bit floating point will be insufficient and lead to rounding errors. Using a arbitrary precision library doesn't help either because one is dealing with irrational numbers here.