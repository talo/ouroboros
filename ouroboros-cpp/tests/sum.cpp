#include <string>

#include "../include/ouroboros.h"

int main(const int argc, const char **argv)
{
    auto io = ouroboros::init<std::vector<int32_t>, int32_t>(argc, argv);

    return 0;
}