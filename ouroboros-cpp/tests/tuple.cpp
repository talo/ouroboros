#include <string>
#include <tuple>

#include "../include/ouroboros.h"

/**
 * Build this program:
 *
 * ```
 * clang++ -std=c++17 -o ./tests/tuple ./tests/tuple.cpp
 * ```
 *
 * See the expected ins/outs of the program:
 *
 * ```
 * ./tests/tuple --introspect
 * ```
 *
 * Run the program:
 *
 * ```
 * ./tests/tuple '[1.0, 2.0, 3.0]' '[4.0, 5.0, 6.0]' '"out"'
 * ```
 */
int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::tuple<std::tuple<float, float, float>, std::tuple<float, float, float>>, float>(argc, argv);

    auto [x1, y1, z1] = std::get<0>(in);
    auto [x2, y2, z2] = std::get<1>(in);

    auto dx = x2 - x1;
    auto dy = y2 - y1;
    auto dz = z2 - z1;

    out << std::sqrt(dx * dx + dy * dy + dz * dz);

    return 0;
}