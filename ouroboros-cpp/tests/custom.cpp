#include <cmath>
#include <tuple>

#include "../include/ouroboros.h"

OUROBOROS_STRUCT(Point,
                 float, x,
                 float, y,
                 float, z);

/**
 * Build this program:
 *
 * ```
 * clang++ -std=c++17 -o ./tests/custom ./tests/custom.cpp
 * ```
 *
 * See the expected ins/outs of the program:
 *
 * ```
 * ./tests/custom --introspect
 * ```
 *
 * Run the program:
 *
 * ```
 * ./tests/custom '{"x": 1.0, "y": 2.0, "z": 3.0}' '{"x": 4.0, "y": 5.0, "z": 6.0}' '"out"'
 * ```
 */
int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::tuple<Point, Point>, float>(argc, argv);

    auto [p1, p2] = in;

    auto dx = p2.x - p1.x;
    auto dy = p2.y - p1.y;
    auto dz = p2.z - p1.z;

    out << std::sqrt(dx * dx + dy * dy + dz * dz);

    return 0;
}