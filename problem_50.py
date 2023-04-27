import collections

from tqdm import tqdm
from problem_7 import prime_sieve


def solution() -> int:
    ceiling = 1_000_000
    summands = collections.defaultdict(lambda: 0)
    primes = prime_sieve(ceiling)
    for begin in tqdm(range(len(primes))):
        accumulator = 0
        num_summands = 0
        for prime in primes[begin:]:
            accumulator += prime
            if accumulator >= ceiling:
                break
            num_summands += 1
            if accumulator in primes:
                summands[accumulator] = max(num_summands, summands[accumulator])
    return max((num, prime) for prime, num in summands.items() if prime < ceiling)[1]


if __name__ == "__main__":
    import runner

    runner.run(globals())
