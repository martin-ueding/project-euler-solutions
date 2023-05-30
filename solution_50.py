from solution_7 import prime_sieve


def get_max_sequence_length(primes: list[int]) -> int:
    accumulator = 0
    for i, prime in enumerate(primes, 1):
        accumulator += prime
        if accumulator > 1_000_000:
            return i


def solution() -> int:
    ceiling = 1_000_000
    primes = prime_sieve(ceiling)
    prime_set = set(primes)
    max_prime = primes[-1]
    for length in range(get_max_sequence_length(primes), 0, -1):
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
