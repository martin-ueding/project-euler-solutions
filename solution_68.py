from typing import Iterator


def triangle_valid(numbers: list[int]) -> bool:
    if len(numbers) > 5:
        first = numbers[0] + numbers[1] + numbers[2]
        second = numbers[3] + numbers[2] + numbers[4]
        third = numbers[5] + numbers[4] + numbers[1]
        return first == second == third
    elif len(numbers) > 4:
        first = numbers[0] + numbers[1] + numbers[2]
        second = numbers[3] + numbers[2] + numbers[4]
        return first == second
    else:
        return True


def shift_lines(lines: list[tuple]) -> list[tuple]:
    starts = [line[0] for line in lines]
    min_start = min(starts)
    while lines[0][0] != min_start:
        lines = lines[1:] + lines[:1]
    return lines


def triangle_solution_string(coefficients: list[int]) -> int:
    lines = [
        (coefficients[0], coefficients[1], coefficients[2]),
        (coefficients[3], coefficients[2], coefficients[4]),
        (coefficients[5], coefficients[4], coefficients[1]),
    ]
    lines = shift_lines(lines)
    return int("".join(map(str, (number for line in lines for number in line))))


def triangle_solutions(coeffcients: list[int]) -> Iterator[list[int]]:
    if triangle_valid(coeffcients):
        if len(coeffcients) == 6:
            yield triangle_solution_string(coeffcients)
    else:
        return
    for number in range(1, 7):
        if number in coeffcients:
            continue
        coeffcients.append(number)
        yield from triangle_solutions(coeffcients)
        coeffcients.pop()


def pentagon_valid(numbers: list[int]) -> bool:
    if len(numbers) > 9:
        first = numbers[0] + numbers[1] + numbers[2]
        second = numbers[3] + numbers[2] + numbers[4]
        third = numbers[5] + numbers[4] + numbers[6]
        fourth = numbers[7] + numbers[6] + numbers[8]
        fifth = numbers[9] + numbers[8] + numbers[1]
        return first == second == third == fourth == fifth
    elif len(numbers) > 8:
        first = numbers[0] + numbers[1] + numbers[2]
        second = numbers[3] + numbers[2] + numbers[4]
        third = numbers[5] + numbers[4] + numbers[6]
        fourth = numbers[7] + numbers[6] + numbers[8]
        return first == second == third == fourth
    elif len(numbers) > 6:
        first = numbers[0] + numbers[1] + numbers[2]
        second = numbers[3] + numbers[2] + numbers[4]
        third = numbers[5] + numbers[4] + numbers[6]
        return first == second == third
    elif len(numbers) > 4:
        first = numbers[0] + numbers[1] + numbers[2]
        second = numbers[3] + numbers[2] + numbers[4]
        return first == second
    else:
        return True


def pentagon_solution_string(coefficients: list[int]) -> int:
    lines = [
        (coefficients[0], coefficients[1], coefficients[2]),
        (coefficients[3], coefficients[2], coefficients[4]),
        (coefficients[5], coefficients[4], coefficients[6]),
        (coefficients[7], coefficients[6], coefficients[8]),
        (coefficients[9], coefficients[8], coefficients[1]),
    ]
    lines = shift_lines(lines)
    return int("".join(map(str, (number for line in lines for number in line))))


def pentagon_solutions(coeffcients: list[int]) -> Iterator[list[int]]:
    if pentagon_valid(coeffcients):
        if len(coeffcients) == 10:
            yield pentagon_solution_string(coeffcients)
    else:
        return
    for number in range(1, 10):
        if number in coeffcients:
            continue
        coeffcients.append(number)
        yield from pentagon_solutions(coeffcients)
        coeffcients.pop()


def solution() -> int:
    return max(pentagon_solutions([10]))


if __name__ == "__main__":
    import runner

    runner.run(globals())
