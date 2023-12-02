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
 * clang++ -std=c++17 -o ./tests/object ./tests/object.cpp
 * ```
 *
 * See the expected ins/outs of the program:
 *
 * ```
 * ./tests/object --introspect
 * ```
 *
 * Run the program:
 *
 * ```
 * ./tests/object '"p1"' '"p2"' '"out"'
 * ```
 *
 * where `"p1"` and `"p2"` are JSON files that contain `{"x": 1.0, "y": 2.0,
 * "z": 3.0}` and `{"x": 4.0, "y": 5.0, "z": 6.0}` respectively.
 */
int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::tuple<ouroboros::Object<Point>, ouroboros::Object<Point>>, ouroboros::Object<float>>(argc, argv);

    auto [o1, o2] = in;

    auto p1 = Point{};
    auto p2 = Point{};

    o1 >> p1;
    o2 >> p2;

    auto dx = p2.x - p1.x;
    auto dy = p2.y - p1.y;
    auto dz = p2.z - p1.z;

    out << ouroboros::Object<float>{} << std::sqrt(dx * dx + dy * dy + dz * dz);

    return 0;
}