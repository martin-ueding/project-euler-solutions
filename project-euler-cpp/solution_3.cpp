#include "primes.hpp"
#include "timings.hpp"

int main() {
    auto const start = now();

    long const number = 600851475143L;
    long remainder = number;
    long solution = 1;
    for (auto prime : primes()) {
        while (remainder % prime == 0) {
            remainder /= prime;
        }
        if (remainder == 1) {
            solution = prime;
            break;
        }
    }

    report_duration(start);
    report_solution(solution);
}