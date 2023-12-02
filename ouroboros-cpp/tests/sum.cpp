#include <string>

#include "../include/ouroboros.h"

/**
 * Build this program:
 *
 * ```
 * clang++ -std=c++17 -o ./tests/sum ./tests/sum.cpp
 * ```
 *
 * See the expected ins/outs of the program:
 *
 * ```
 * ./tests/sum --introspect
 * ```
 *
 * Run the program:
 *
 * ```
 * ./tests/sum '[1,2,3,4]' '"out"'
 * ```
 */
int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::vector<int32_t>, int32_t>(argc, argv);

    out << std::accumulate(in.begin(), in.end(), 0);

    return 0;
}