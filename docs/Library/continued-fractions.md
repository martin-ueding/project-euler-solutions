# Continued Fractions

A continued fraction is something that looks like
$$ a_0 + \frac 1 {a_1 + \frac 1 {a_2 + \frac 1 {a_3 + \dots}}} \,. $$

These are useful for approximating irrational numbers by a rational one. One can just cut off the continued fraction at any point and simplify the fraction.

## Expansion algorithm

The algorithm to expand a number works like the following. We will take $\sqrt 3$ as an example. As it is approximately 1.732, we know $1 < \sqrt 3 < 2$. We then subtract the full integer like so:
$$ \sqrt 3 = 1 + (\sqrt 3 - 1) \,. $$

We now know that $\sqrt 3 - 1 < 1$ and hence the inverse will be larger than one. We write the above more complicated, like so:
$$ \sqrt 3 = 1 + \frac{1}{\frac{1}{\sqrt 3 - 1}} \,. $$

The $\frac{1}{\sqrt 3 - 1}$ is 1.366, hence we can split off another 1.
$$ \sqrt 3 = 1 + \frac{1}{1 + \frac{1}{\sqrt 3 - 1} - 1} \,. $$

We take $\frac{1}{\sqrt 3 - 1} - 1$, which is smaller than 1, and simplify the fraction first. We first multiply the fraction with $\sqrt 3 + 1$ such that this gives $3 - 1^2 = 2$.
$$ \frac{1}{\sqrt 3 - 1} - 1 = \frac{\sqrt 3 + 1}{2} - 1 \,. $$

This expression evaluates to 0.366, so let us write it again as a double inverse.
$$ \frac{1}{\sqrt 3 - 1} - 1 = \frac{1}{\frac{1}{\frac{\sqrt 3 + 1}{2} - 1}} \,. $$

We simplify the innermost fraction again:
$$ \frac{\sqrt 3 + 1}{2} - 1 = \frac{\sqrt 3 - 1}{2} \,. $$

Such that we have 
$$ \frac{1}{\sqrt 3 - 1} - 1 = \frac{1}{\frac{1}{\frac{\sqrt 3 - 1}{2}}} \,. $$

Then we can resolve one level of fractions again:
$$ \frac{1}{\sqrt 3 - 1} - 1 = \frac{1}{\frac{2}{\sqrt 3 - 1}} \,. $$

We can then split off a 2 because the following sub-expression evaluates to 2.732:
$$ \frac{2}{\sqrt 3 - 1} = 2 + \frac{2}{\sqrt 3 - 1} - 2 \,. $$

We then need to simplify the latter part:
$$ \frac{2}{\sqrt 3 - 1} - 2 = 2 \frac{\sqrt 3 + 1}{2} - 2 = \sqrt 3 - 1 \,. $$

Let us collect the terms that we have so far:
$$ \sqrt 3 = 1 + \frac{1}{1 + \frac{1}{2 + (\sqrt 3 - 1)}} \,. $$

$\sqrt 3 - 1$ is an expression that we have already encountered. The pattern repeats itself and is periodic. The next step would bring us here:
$$ \sqrt 3 = 1 + \frac{1}{1 + \frac{1}{2 + \frac{1}{1 + \frac{1}{\sqrt 3 - 1} - 1}}} \,. $$

We're only interested in the coefficients. There is a non-periodic part of $(1)$ and a periodic part of $(1, 2)$. Project Euler suggests the notation $[1; (1, 2)]$ for the prefix and the periodic part.

## Convergents

The continued fractions are infinitely deep and hence equal the irrational number. When we cut off the expansion at some point, we can get a rational approximation. Let us again take $\sqrt 3$ and cut off the expansion at $(1, 1, 2, 1)$. The continued fraction then takes this finite form:
$$ \sqrt 3 \approx 1 + \frac{1}{1 + \frac{1}{2 + \frac{1}{1 + \frac{1}{2}}}} \,. $$

We need to simplify this fraction. We start on the inside and simplify:
$$ 1 + \frac{1}{2} = \frac 32 \,. $$

Then we need to invert this and simplify with the next coefficient:
$$ 2 + \frac 23 = \frac 83 \,. $$

Invert again and simplify with the next coefficient:
$$ 1 + \frac 38 = \frac{11}{8} \,. $$

And again:
$$ 1 + \frac{8}{11} = \frac{19}{11} = 1.\overline{72} \,. $$
That is a good approximation to $\sqrt{3} \approx 1.73205$.

The algorithm here is just taking a list of coefficients and simplifying the fractions in reverse.