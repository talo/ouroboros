#include <cmath>
#include <tuple>

#include "../include/ouroboros.h"

OUROBOROS_STRUCT(Point,
                 float, x,
                 float, y,
                 float, z);

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