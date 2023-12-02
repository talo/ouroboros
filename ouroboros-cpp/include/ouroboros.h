#pragma once

#include <optional>
#include <string>
#include <tuple>
#include <vector>

#include "json.h"
#include "macro.h"
#include "mutable.h"
#include "object.h"
#include "uuid.h"

namespace ouroboros
{

    template <typename T>
    struct TypeInfo
    {
        static nlohmann::json type_info();
    };

    template <>
    struct TypeInfo<bool>
    {
        static nlohmann::json type_info()
        {
            return "bool";
        }
    };

    template <>
    struct TypeInfo<uint8_t>
    {
        static nlohmann::json type_info()
        {
            return "u8";
        }
    };

    template <>
    struct TypeInfo<uint16_t>
    {
        static nlohmann::json type_info()
        {
            return "u16";
        }
    };

    template <>
    struct TypeInfo<uint32_t>
    {
        static nlohmann::json type_info()
        {
            return "u32";
        }
    };

    template <>
    struct TypeInfo<uint64_t>
    {
        static nlohmann::json type_info()
        {
            return "u64";
        }
    };

    template <>
    struct TypeInfo<int8_t>
    {
        static nlohmann::json type_info()
        {
            return "i8";
        }
    };

    template <>
    struct TypeInfo<int16_t>
    {
        static nlohmann::json type_info()
        {
            return "i16";
        }
    };

    template <>
    struct TypeInfo<int32_t>
    {
        static nlohmann::json type_info()
        {
            return "i32";
        }
    };

    template <>
    struct TypeInfo<int64_t>
    {
        static nlohmann::json type_info()
        {
            return "i64";
        }
    };

    template <>
    struct TypeInfo<float>
    {
        static nlohmann::json type_info()
        {
            return "f32";
        }
    };

    template <>
    struct TypeInfo<double>
    {
        static nlohmann::json type_info()
        {
            return "f64";
        }
    };

    template <>
    struct TypeInfo<std::string>
    {
        static nlohmann::json type_info()
        {
            return "string";
        }
    };

    template <typename T1>
    struct TypeInfo<std::tuple<T1>>
    {
        static nlohmann::json type_info()
        {
            return nlohmann::json{{"k", "tuple"}, {"t", {TypeInfo<T1>::type_info()}}};
        }
    };

    template <typename T1, typename T2>
    struct TypeInfo<std::tuple<T1, T2>>
    {
        static nlohmann::json type_info()
        {
            return nlohmann::json{{"k", "tuple"}, {"t", {TypeInfo<T1>::type_info(), TypeInfo<T2>::type_info()}}};
        }
    };

    template <typename T1, typename T2, typename T3>
    struct TypeInfo<std::tuple<T1, T2, T3>>
    {
        static nlohmann::json type_info()
        {
            return nlohmann::json{{"k", "tuple"}, {"t", {TypeInfo<T1>::type_info(), TypeInfo<T2>::type_info(), TypeInfo<T3>::type_info()}}};
        }
    };

    template <typename T1, typename T2, typename T3, typename T4>
    struct TypeInfo<std::tuple<T1, T2, T3, T4>>
    {
        static nlohmann::json type_info()
        {
            return nlohmann::json{{"k", "tuple"}, {"t", {TypeInfo<T1>::type_info(), TypeInfo<T2>::type_info(), TypeInfo<T3>::type_info(), TypeInfo<T4>::type_info()}}};
        }
    };

    template <typename T>
    struct TypeInfo<std::vector<T>>
    {
        static nlohmann::json type_info()
        {
            return nlohmann::json{{"k", "array"}, {"t", TypeInfo<T>::type_info()}};
        }
    };
    struct Manifest
    {
        std::string name;
        nlohmann::json ins;
        nlohmann::json outs;
        // // ResourceBounds resource_bounds;
        std::optional<std::string> description;
        std::optional<std::string> usage;
        std::optional<std::vector<std::tuple<std::string, std::string>>> ins_usage;
        std::optional<std::vector<std::tuple<std::string, std::string>>> outs_usage;
        // // Supported tengu-runtime version
        // std::optional<VersionReq> runtime_version;
        // std::optional<ManifestVersion> manifest_version;
    };

    template <typename I, typename O>
    std::tuple<I, Mutable<O>> init(const int argc, const char **argv)
    {
        // Check if there are enough arguments for the module to be valid
        if (argc < 3)
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
                {"description", "Echoes the input to the output"},
            };
            std::cout << m << std::endl;
            exit(0);
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