#include "primes.hpp"
#include "timings.hpp"

int solution() {
    long const number = 600851475143L;
    long remainder = number;
    for (auto prime : primes()) {
        while (remainder % prime == 0) {
            remainder /= prime;
        }
        if (remainder == 1) {
            return prime;
        }
    }
}

int main() { run_solution(solution); }