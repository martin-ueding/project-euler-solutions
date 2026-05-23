#include "registry.hpp"

std::map<int, std::map<std::string, std::function<int64_t(void)>>> solutions;