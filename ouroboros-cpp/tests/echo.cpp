#include <string>

#include "../include/ouroboros.h"

int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::string, std::string>(argc, argv);

    out << in;

    return 0;
}