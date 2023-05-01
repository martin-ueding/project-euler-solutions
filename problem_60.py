import collections
from problem_3 import prime_generator
from problem_58 import is_prime_accelerated


def has_property(first: int, second: int) -> bool:
    return is_prime_accelerated(int(str(first) + str(second))) and is_prime_accelerated(
        int(str(second) + str(first))
    )


def find_tuple_of(size: int) -> int:
    partners = collections.defaultdict(set)
    for first_prime in prime_generator():
        for second_prime in prime_generator():
            if first_prime == second_prime:
                break
            if has_property(first_prime, second_prime):
                partners[second_prime].add(first_prime)
                if len(partners[second_prime]) == size - 1:
                    print(second_prime, partners[second_prime])
                    return sum(partners[second_prime])


def solution() -> None:
    return find_tuple_of(5)


if __name__ == "__main__":
    import runner

    runner.run(globals())
