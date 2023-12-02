#pragma once

#include <optional>
#include <string>
#include <tuple>
#include <vector>

#include "json.h"

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
}