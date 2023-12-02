#pragma once

#include <optional>
#include <string>
#include <tuple>
#include <vector>

#include "json.h"
#include "macro.h"
#include "mutable.h"
#include "object.h"
#include "type_info.h"
#include "uuid.h"

namespace ouroboros
{
    template <typename I, typename O>
    std::tuple<I, Mutable<O>> init(const int argc, const char **argv)
    {
        // Check if there are enough arguments for the module to be valid
        if (argc < 2)
        {
            std::cerr << "error: too few arguments" << std::endl;
            exit(1);
        }

        // Check if first arg is `--introspect`
        if (std::string(argv[1]) == "--introspect")
        {
            nlohmann::json m = {
                {"name", std::string(argv[0])},
                {"ins", TypeInfo<I>::type_info()},
                {"outs", TypeInfo<O>::type_info()},
            };
            std::cout << m << std::endl;
            exit(0);
        }

        // Check if there are enough arguments for the module to be valid
        if (argc < 3)
        {
            std::cerr << "error: too few arguments" << std::endl;
            exit(1);
        }

        auto in_type_info = TypeInfo<I>::type_info();

        // If multiple input arguments are passed, then it is required that the
        // input type is a tuple.
        if (argc > 3)
        {
            if (!in_type_info.at("k").is_null() && in_type_info.at("k") == "tuple")
            {
                // Combine all of the arguments into one JSON array.
                std::string args = "[";
                for (int i = 1; i < argc - 1; i++) // -1 because the last argument is the output
                {
                    if (i > 1)
                    {
                        args += ",";
                    }
                    args += std::string(argv[i]);
                }
                args += "]";

                // Parse it as a tuple.
                return std::make_tuple<I, Mutable<O>>(nlohmann::json::parse(args), nlohmann::json::parse(argv[argc - 1]));
            }

            // If the input type is not a tuple, then throw an error.
            std::cerr << "error: too many arguments" << std::endl;
            exit(1);
        }

        return std::make_tuple<I, Mutable<O>>(nlohmann::json::parse(argv[1]), nlohmann::json::parse(argv[2]));
    }
}