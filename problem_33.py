def greatest_common_denominator(a: int, b: int) -> int:
    while b != 0:
        b, a = a % b, b
    return a


def solution() -> int:
    result_numerator = 1
    result_denominator = 1
    for numerator in range(10, 100):
        for denominator in range(numerator + 1, 100):
            common_digits = set(str(numerator)) & set(str(denominator))
            if "0" in common_digits:
                common_digits.remove("0")
            if len(common_digits) == 1:
                common_digit = list(common_digits)[0]
                new_numerator = str(numerator).replace(str(common_digit), "")
                new_denominator = str(denominator).replace(str(common_digit), "")
                if not new_numerator or not new_denominator:
                    continue
                if numerator * int(new_denominator) == denominator * int(new_numerator):
                    result_numerator *= numerator
                    result_denominator *= denominator
    gcd = greatest_common_denominator(result_numerator, result_denominator)
    return result_denominator // gcd


if __name__ == "__main__":
    import runner

    runner.run(globals())
