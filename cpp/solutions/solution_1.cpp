#include "registry.hpp"

int64_t solution_1() {
    int64_t sum = 0;
    for (int i = 0; i < 1000; ++i) {
        if (i % 3 == 0 || i % 5 == 0) {
            sum += i;
        }
    }
    return sum;
}

static Registration registration(1, solution_1);