#include <string>

#include "../include/ouroboros.h"

OUROBOROS_TYPE_DEF_ENUM(
    Baz,
    A,
    B,
    C);

OUROBOROS_TYPE_DEF_STRUCT(
    Bar,
    bool, x,
    uint32_t, y,
    float, z,
    double, u,
    std::string, v,
    Baz, baz);

OUROBOROS_TYPE_DEF_STRUCT(
    Foo,
    bool, x,
    uint32_t, y,
    float, z,
    double, u,
    std::string, v,
    Bar, bar);

int main(const int argc, const char **argv)
{
    auto io = ouroboros::init<Foo, std::string>(argc, argv);

    return 0;
}