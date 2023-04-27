import collections

from tqdm import tqdm
from problem_7 import prime_sieve


def solution() -> int:
    ceiling = 1_000_000
    summands = collections.defaultdict(lambda: 0)
    primes = prime_sieve(ceiling)
    prime_set = set(primes)
    max_prime = primes[-1]
    for length in tqdm(range(len(primes) - 1, 0, -1)):
        rolling_sum = sum(primes[:length])
        for begin in range(len(primes) - length):
            end = begin + length
            rolling_sum += primes[end] - primes[begin]
            if rolling_sum > max_prime:
                break
            if rolling_sum in prime_set:
                return rolling_sum


if __name__ == "__main__":
    import runner

    runner.run(globals())
