import itertools
from typing import Optional


def pandigital_product(number: int) -> Optional[int]:
    digits = set()
    results = []
    for n in itertools.count(1):
        product = number * n
        product_digits = set(str(product))
        if len(product_digits) != len(str(product)):
            return None
        if "0" in product_digits:
            return None
        if digits & product_digits:
            return None
        else:
            digits |= product_digits
            results.append(product)
            if len(digits) == 9:
                return int("".join(map(str, results)))


def solution() -> int:
    pandigital_products = []
    for start in range(1, 100_000):
        if product := pandigital_product(start):
            pandigital_products.append(product)
    return max(pandigital_products)


if __name__ == "__main__":
    import runner

    runner.run(globals())
