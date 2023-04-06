def solution_grid_search() -> int:
    largest = 0
    for i in range(100, 1000):
        for j in range(i, 1000):
            product = i * j
            if product > largest and str(product) == str(product)[::-1]:
                largest = product
    return largest


if __name__ == "__main__":
    import runner

    runner.run(globals())
