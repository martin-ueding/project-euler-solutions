# Cube Digit Pairs (Problem 90)

We have the square numbers below 100, namely 01, 04, 09, 16, 25, 36, 49, 64, 81. Then there is the idea of representing the first and second digit with the faces of a cube, where we can pick six digits to print on. The question now is how many distinct sets there are.

And then there is the additional complication that one can use a 6 as a 9, but one has to count them as distinct sets.

## Thoughts

For the first digit, we have {0, 1, 2, 3, 4, 6, 8}. For the second digit we have {1, 4, 5, 6, 9}. As the first set already has 7 elements, we must be allowed to exchange the cubes. Ah, this is what makes the problem difficult! Initially I thought that I would just take these sets and then fill them up with other numbers.

So what we really need to do is figure out two sets such that all the digits are there but the ones that need together are not both in the same set.

Let's think differently of the numbers, let's to tuples (0, 1), (0, 4), (0, 9), (1, 6), (2, 5), (3, 6), (4, 9). We can also reverse these tuples. So perhaps this is a way?

- Decide which of the tuples to flip. That fixes how we represent all the numbers with the left and right cube.
- Collect all the first and second digits into a set each.
- If any of the sets exceeds six elements (while considering the 6-9-rule), skip this combination.
- See which digits are not included. If a 6 is present, we can surely also add a 9, even though we don't need it. Do all possible subsets of the other digits to fill up the cubes.
- Record that particular combination

We can choose 6 out of 10 digits per cube. That makes 210 combinations that we can have. With two cubes, that gives 44,100 combinations in total. that isn't that much.