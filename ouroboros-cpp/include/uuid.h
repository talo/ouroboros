#pragma once

#include <random>
#include <sstream>
#include <iomanip>

namespace ouroboros
{

    std::string generate_uuid()
    {
        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_int_distribution<> dis(0, 15);
        std::uniform_int_distribution<> dis2(8, 11);

        std::stringstream ss;
        ss << std::hex;
        for (int i = 0; i < 8; i++)
        {
            ss << dis(gen);
        }
        ss << "-";
        for (int i = 0; i < 4; i++)
        {
            ss << dis(gen);
        }
        ss << "-4"; // The 4 signifies the UUID version
        for (int i = 0; i < 3; i++)
        {
            ss << dis(gen);
        }
        ss << "-";
        ss << dis2(gen); // The variant
        for (int i = 0; i < 3; i++)
        {
            ss << dis(gen);
        }
        ss << "-";
        for (int i = 0; i < 12; i++)
        {
            ss << dis(gen);
        }
        return ss.str();
    }

}