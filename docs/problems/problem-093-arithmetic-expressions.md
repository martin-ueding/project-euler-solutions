# Arithmetic Expressions (93)

In [Problem 93](https://projecteuler.net/problem=93), we're asked to find four distinct digits such that can generate a lot of consecutive numbers using all possible arithmetic expressions.

The number of combinations seems to be overwhelming at first. We can reduce the combinatorics a lot, though. Let us denote the four digit positions with $x_1$ to $x_4$ and the three binary operations with $\circ_1$ to $\circ_3$. Essentially, there are just two structurally different expressions:
$$ (x_1 \circ_1 x_2) \circ_2 (x_3 \circ_3 x_4), \\quad ((x_1 \circ_1 x_2) \circ_2 x_3) \circ_3 x_4 \,. $$

The algorithm is the following:

- We draw four out of ten digits for our $a<b<c<d$.
- Iterate through all possible permutations of these digits …
    - Draw three binary operations with replacement.
    - Iterate through all permutations of these operations …
        - Insert numbers and operations into both expression structures and evaluate, record the results in a set, if they yield an integer.
- Start iterating from 1 and see how many consecutive numbers are in the set.
- Keep track of the maximum number of consecutive numbers so far and keep $a$, $b$, $c$, $d$ around.

## Complications

The implementation is a bit tedious as one has to work with a lot of function pointers. I ended up defining a binary closure for each of the four operations.

Another thing is that by the division operation, one might end up with a rational number. I have implemented this using a fractions library to only have integer arithmetic. Another possible way would be to use floating point numbers and see whether they are close enough to an integer to have that count.