import collections
import itertools

from tqdm import tqdm
from solution_58 import is_prime_accelerated
from solution_7 import prime_sieve

ceiling = 1000


def has_property(first: int, second: int) -> bool:
    number_1 = int(str(first) + str(second))
    number_2 = int(str(second) + str(first))
    return is_prime_accelerated(number_1) and is_prime_accelerated(number_2)


def _solution_full_grid() -> int:
    primes = prime_sieve(ceiling)
    prime_set = set(primes)
    for p1 in primes:
        for p2 in primes:
            if p2 == p1:
                break
            if has_property(p1, p2):
                for p3 in primes:
                    if p3 == p2:
                        break
                    if has_property(p1, p3) and has_property(p2, p3):
                        for p4 in primes:
                            if p4 == p3:
                                break
                            if (
                                has_property(p1, p4)
                                and has_property(p2, p4)
                                and has_property(p3, p4)
                            ):
                                return sum([p1, p2, p3, p4])


def create_partner_graph(all_partners: dict[int, list[int]]) -> None:
    with open("p060_graph.dot", "w") as f:
        f.write("digraph {\noverlap=false\nsplines=true\n\n")
        for number, partners in all_partners.items():
            for partner in partners:
                f.write(f"  {number} -> {partner}\n")
        f.write("}\n")


def find_tuple_of(size: int) -> int:
    partners = collections.defaultdict(list)
    primes = prime_sieve(10000)
    for first_prime in tqdm(primes):
        for second_prime in primes:
            if second_prime == first_prime:
                break
            if has_property(first_prime, second_prime):
                partners[first_prime].append(second_prime)
        if len(partners[first_prime]) >= size - 1:
            for subset in itertools.combinations(partners[first_prime], size - 1):
                for i, number in enumerate(subset):
                    for other_number in subset[:i]:
                        if other_number not in partners[number]:
                            break
                    else:
                        continue
                    break
                else:
                    return first_prime + sum(subset)


def solution() -> None:
    return find_tuple_of(5)


if __name__ == "__main__":
    import runner

    runner.run(globals())
