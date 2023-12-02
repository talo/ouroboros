#include <string>

#include "../include/ouroboros.h"

int main(const int argc, const char **argv)
{
    auto [in, out] = ouroboros::init<std::vector<int32_t>, int32_t>(argc, argv);

    out << std::accumulate(in.begin(), in.end(), 0);

    return 0;
}