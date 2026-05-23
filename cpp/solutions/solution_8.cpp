#include <algorithm>
#include <cstring>
#include <iostream>
#include <numeric>
#include <ranges>
#include <vector>

#include "registry.hpp"

static char const* const DIGIT_STRING =
    "73167176531330624919225119674426574742355349194934969835203127745063262395"
    "78318016984801869478851843858615607891129494954595017379583319528532088055"
    "11125406987471585238630507156932909632952274430435576689664895044524452316"
    "17318564030987111217223831136222989342338030813533627661428280644448664523"
    "87493035890729629049156044077239071381051585930796086670172427121883998797"
    "90879227492190169972088809377665727333001053367881220235421809751254540594"
    "75224352584907711670556013604839586446706324415722155397536978179778461740"
    "64955149290862569321978468622482839722413756570560574902614079729686524145"
    "35100474821663704844031998900088952434506585412275886668811642717147992444"
    "29282308634656748139191231628245861786645835912456652947654568284891288314"
    "26076900422421902267105562632111110937054421750694165896040807198403850962"
    "45544436298123098787992724428490918884580156166097919133875499200524063689"
    "91256071760605886116467109405077541002256983155200055935729725716362695618"
    "82670428252483600823257530420752963450";

static constexpr int NUM_DIGITS = 13;

std::vector<int> get_digits() {
    std::vector<int> result;
    auto const len = std::strlen(DIGIT_STRING);
    result.reserve(len);
    for (auto i = 0; i < len; ++i) {
        result.push_back(DIGIT_STRING[i] - '0');
    }
    return result;
}

int64_t solution_8_procedural() {
    auto const digits = get_digits();
    int64_t largest = 0;
    for (int start = 0; start < digits.size() - NUM_DIGITS; ++start) {
        int64_t product = 1;
        for (int i = start; i < start + NUM_DIGITS; ++i) {
            product *= digits[i];
        }
        largest = std::max(largest, product);
    }
    return largest;
}

int64_t solution_8_ranges() {
    auto const digits = get_digits();
    auto const products = digits | std::views::slide(NUM_DIGITS) |
                          std::views::transform([](auto window) {
                              return std::ranges::fold_left(
                                  window, 1LL, std::multiplies<int64_t>());
                          });

    return std::ranges::fold_left(
        products, 0LL, [](int64_t a, int64_t b) { return std::max(a, b); });
}

static Registration registration(8, solution_8_procedural);