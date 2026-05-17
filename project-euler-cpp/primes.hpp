#pragma once

#include <vector>

class PrimeGenerator;

class PrimeIterator {
   public:
    PrimeIterator(PrimeGenerator& prime_generator, size_t const index)
        : prime_generator(prime_generator), index(index) {}

    int operator*();

    PrimeIterator operator++();

    bool operator!=(PrimeIterator const& other) { return index != other.index; }

   private:
    PrimeGenerator& prime_generator;
    size_t index = 0;
};

class PrimeGenerator {
   public:
    PrimeGenerator() {}

    PrimeIterator begin() { return PrimeIterator(*this, 0); }
    PrimeIterator end() { return PrimeIterator(*this, -1); }

    int get(size_t index) {
        advance(index);
        return primes[index];
    }

   private:
    void advance(size_t const index);

    std::vector<int> primes;
};