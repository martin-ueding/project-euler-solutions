#include "primes.hpp"
#include "registry.hpp"

int64_t solution_3() {
    int64_t const number = 600851475143L;
    int64_t remainder = number;
    for (auto prime : PrimeGenerator()) {
        while (remainder % prime == 0) {
            remainder /= prime;
        }
        if (remainder == 1) {
            return prime;
        }
    }
    throw std::runtime_error("Unreachable.");
}

static Registration registration(3, solution_3);