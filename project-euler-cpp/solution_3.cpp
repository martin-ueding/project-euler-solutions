#include "primes.hpp"
#include "registry.hpp"

int solution_3() {
    long const number = 600851475143L;
    long remainder = number;
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