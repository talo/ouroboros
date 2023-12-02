#include <string>

#include "../include/ouroboros.h"

/**
 * Build this program:
 *
 * ```
 * clang++ -std=c++17 -o ./tests/echo ./tests/echo.cpp
 * ```
 *
 * See the expected ins/outs of the program:
 *
 * ```
 * ./tests/echo --introspect
 * ```
 *
 * Run the program:
 *
 * ```
 * ./tests/echo '"hello, world!"' '"out"'
 * ```
 */
int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::string, std::string>(argc, argv);

    out << in;

    return 0;
}