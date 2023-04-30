from problem_38 import pandigital_product


def test_is_pandigital_product() -> None:
    assert pandigital_product(192) == 192_384_576
