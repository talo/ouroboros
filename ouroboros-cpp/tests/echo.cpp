#include <string>

#include "../include/ouroboros.h"

int main(const int argc, const char **argv)
{
    auto io = ouroboros::init<std::string, std::string>(argc, argv);

    return 0;
}