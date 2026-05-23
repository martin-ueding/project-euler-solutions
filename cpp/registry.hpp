#pragma once

#include <cstdint>
#include <functional>
#include <map>
#include <string>

extern std::map<int, std::map<std::string, std::function<int64_t(void)>>>
    solutions;

class Registration {
   public:
    Registration(int const id, std::string const& name,
                 std::function<int64_t(void)> solution) {
        solutions[id][name] = solution;
    }
};