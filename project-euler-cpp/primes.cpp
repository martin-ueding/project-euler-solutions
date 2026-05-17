#include "primes.hpp"

#include <algorithm>
#include <ranges>
#include <vector>

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

std::generator<long long> primes() {
    std::vector<long long> known;
    for (long long candidate = 2;; ++candidate) {
        bool is_prime =
            std::ranges::none_of(known | std::views::take_while([&](auto p) {
                                     return p * p <= candidate;
                                 }),
                                 [&](auto p) { return candidate % p == 0; });
        if (is_prime) {
            known.push_back(candidate);
            co_yield candidate;
        }
    }
}