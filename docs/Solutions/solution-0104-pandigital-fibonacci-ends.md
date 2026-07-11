# Pandigital Fibonacci Ends (104)

In [Problem 104](https://projecteuler.net/problem=104), we're asked to find the first Fibonacci number where the first and last 9 digits respectively are pandigital (containing the digits 1 to 9 exactly once).

The Fibonacci numbers grow exponentially, their number of digits grows linearly. Keeping track of the whole numbers seems hopeless.

For the lower 9 digits, we can just make use of the addition with modulus. We are only interested in $F_n \operatorname{mod} 10^{10}$. As $F_n = F_{n-2} + F_{n-1}$, we use the trick
$$ F_n \operatorname{mod} 10^{10} = (F_{n-2} \operatorname{mod} 10^{10} + F_{n-1} \operatorname{mod} 10^{10}) \operatorname{mod} 10^{10} \,. $$

We only need to store the low part of the Fibonacci numbers and discard the high part. This lets us iterate easily through all the numbers, no matter their actual size.

Iterating through the low part means that we can check the pandigital property there. This already makes the list of candidates much shorter. For these candidates, we need to check the high part.

Using the [logarithmic approximation to the Fibonacci numbers](../Library/fibonacci-numbers.md#numerical-approximation), we do the following:

- Compute $f_n := \log_{10}(F_n)$ using the approximation.
- Isolate the fractional part, $b := f_n - \lfloor f_n \rfloor$ as we don't care about the absolute magnitude of the number, only the digits.
- As $b \in [0, 1)$, we can make a nine digit number via $10^{9 + b}$.
- We check whether the that nine digit number is pandigital.

The compute performance of the check for a fixed $n$ does not depend on $n$. Therefore this runs in $\mathcal O(N)$ until we have checked $N$ numbers before we find the desired one.

Computing the full Fibonacci numbers using a big number library will take time linear to $n$ as the digits grow linearly with $n$. Isolating the base-ten digits is also linear. This makes a naive approach $\mathcal O(N^2)$, which doesn't work for the $N$ that is the answer here.