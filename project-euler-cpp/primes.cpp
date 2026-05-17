#include "primes.hpp"

void PrimeGenerator::advance(size_t const index) {
    if (primes.empty()) {
        primes.push_back(2);
    }
    auto candidate = primes.back() + 1;
    while (primes.size() <= index) {
        bool is_prime = true;
        for (auto prime : primes) {
            if (prime * prime > candidate) {
                break;
            }
            if (candidate % prime == 0) {
                is_prime = false;
                break;
            }
        }
        if (is_prime) {
            primes.push_back(candidate);
        }
        candidate += 1;
    }
}

int PrimeIterator::operator*() { return prime_generator.get(index); }

PrimeIterator PrimeIterator::operator++() {
    ++index;
    return *this;
}
