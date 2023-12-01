#pragma once

#include <optional>
#include <string>
#include <tuple>
#include <vector>

#include "mutable.h"

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

template <typename T>
struct TypeInfo<std::vector<T>>
{
    static nlohmann::json type_info()
    {
        return nlohmann::json{{"array", TypeInfo<T>::type_info()}};
    }
};

namespace ouroboros
{
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
                {"description", "Echoes the input to the output"},
            };
            std::cout << m << std::endl;
            exit(0);
        }

        for (int i = 1; i < argc; i++)
        {
            nlohmann::json arg = nlohmann::json::parse(argv[i]);
            std::cout << arg << std::endl;
        }

        return std::make_tuple<I, Mutable<O>>(I(), Mutable<O>());
    }
}

#define OUROBOROS_EXPAND(x) x

#define OUROBOROS_GET_MACRO1(_,   \
                             _1,  \
                             _2,  \
                             _3,  \
                             _4,  \
                             _5,  \
                             _6,  \
                             _7,  \
                             _8,  \
                             _9,  \
                             _10, \
                             _11, \
                             _12, \
                             _13, \
                             _14, \
                             _15, \
                             _16, \
                             _17, \
                             _18, \
                             _19, \
                             _20, \
                             _21, \
                             _22, \
                             _23, \
                             _24, \
                             _25, \
                             _26, \
                             _27, \
                             _28, \
                             _29, \
                             _30, \
                             _31, \
                             _32, \
                             NAME, ...) NAME

#define OUROBOROS_GET_MACRO2(_,   \
                             _1,  \
                             _2,  \
                             _3,  \
                             _4,  \
                             _5,  \
                             _6,  \
                             _7,  \
                             _8,  \
                             _9,  \
                             _10, \
                             _11, \
                             _12, \
                             _13, \
                             _14, \
                             _15, \
                             _16, \
                             _17, \
                             _18, \
                             _19, \
                             _20, \
                             _21, \
                             _22, \
                             _23, \
                             _24, \
                             _25, \
                             _26, \
                             _27, \
                             _28, \
                             _29, \
                             _30, \
                             _31, \
                             _32, \
                             _33, \
                             _34, \
                             _35, \
                             _36, \
                             _37, \
                             _38, \
                             _39, \
                             _40, \
                             _41, \
                             _42, \
                             _43, \
                             _44, \
                             _45, \
                             _46, \
                             _47, \
                             _48, \
                             _49, \
                             _50, \
                             _51, \
                             _52, \
                             _53, \
                             _54, \
                             _55, \
                             _56, \
                             _57, \
                             _58, \
                             _59, \
                             _60, \
                             _61, \
                             _62, \
                             _63, \
                             _64, \
                             NAME, ...) NAME

#define OUROBOROS_PASTE1(...) OUROBOROS_EXPAND(OUROBOROS_GET_MACRO1(__VA_ARGS__,         \
                                                                    OUROBOROS_PASTE1_32, \
                                                                    OUROBOROS_PASTE1_31, \
                                                                    OUROBOROS_PASTE1_30, \
                                                                    OUROBOROS_PASTE1_29, \
                                                                    OUROBOROS_PASTE1_28, \
                                                                    OUROBOROS_PASTE1_27, \
                                                                    OUROBOROS_PASTE1_26, \
                                                                    OUROBOROS_PASTE1_25, \
                                                                    OUROBOROS_PASTE1_24, \
                                                                    OUROBOROS_PASTE1_23, \
                                                                    OUROBOROS_PASTE1_22, \
                                                                    OUROBOROS_PASTE1_21, \
                                                                    OUROBOROS_PASTE1_20, \
                                                                    OUROBOROS_PASTE1_19, \
                                                                    OUROBOROS_PASTE1_18, \
                                                                    OUROBOROS_PASTE1_17, \
                                                                    OUROBOROS_PASTE1_16, \
                                                                    OUROBOROS_PASTE1_15, \
                                                                    OUROBOROS_PASTE1_14, \
                                                                    OUROBOROS_PASTE1_13, \
                                                                    OUROBOROS_PASTE1_12, \
                                                                    OUROBOROS_PASTE1_11, \
                                                                    OUROBOROS_PASTE1_10, \
                                                                    OUROBOROS_PASTE1_9,  \
                                                                    OUROBOROS_PASTE1_8,  \
                                                                    OUROBOROS_PASTE1_7,  \
                                                                    OUROBOROS_PASTE1_6,  \
                                                                    OUROBOROS_PASTE1_5,  \
                                                                    OUROBOROS_PASTE1_4,  \
                                                                    OUROBOROS_PASTE1_3,  \
                                                                    OUROBOROS_PASTE1_2,  \
                                                                    OUROBOROS_PASTE1_1,  \
                                                                    _)(__VA_ARGS__))

#define OUROBOROS_PASTE2(...) OUROBOROS_EXPAND(OUROBOROS_GET_MACRO2(__VA_ARGS__,         \
                                                                    OUROBOROS_PASTE2_32, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_31, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_30, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_29, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_28, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_27, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_26, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_25, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_24, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_23, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_22, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_21, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_20, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_19, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_18, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_17, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_16, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_15, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_14, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_13, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_12, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_11, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_10, \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_9,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_8,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_7,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_6,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_5,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_4,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_3,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_2,  \
                                                                    _,                   \
                                                                    OUROBOROS_PASTE2_1,  \
                                                                    _)(__VA_ARGS__))

#define OUROBOROS_FIELD(X1, Y1) X1 Y1;

#define OUROBOROS_VARIANT(X1) X1,

#define OUROBOROS_TYPE_INFO(X1, Y1) {#Y1, TypeInfo<X1>::type_info()},

#define OUROBOROS_TYPE_NAME(Y1) #Y1,

#define OUROBOROS_PASTE1_1(FUNC, X1) FUNC(X1)
#define OUROBOROS_PASTE1_2(FUNC, X1, X2) OUROBOROS_PASTE1_1(FUNC, X1) OUROBOROS_PASTE1_1(FUNC, X2)
#define OUROBOROS_PASTE1_3(FUNC, X1, X2, X3) OUROBOROS_PASTE1_2(FUNC, X1, X2) OUROBOROS_PASTE1_1(FUNC, X3)
#define OUROBOROS_PASTE1_4(FUNC, X1, X2, X3, X4) OUROBOROS_PASTE1_3(FUNC, X1, X2, X3) OUROBOROS_PASTE1_1(FUNC, X4)
#define OUROBOROS_PASTE1_5(FUNC, X1, X2, X3, X4, X5) OUROBOROS_PASTE1_4(FUNC, X1, X2, X3, X4) OUROBOROS_PASTE1_1(FUNC, X5)
#define OUROBOROS_PASTE1_6(FUNC, X1, X2, X3, X4, X5, X6) OUROBOROS_PASTE1_5(FUNC, X1, X2, X3, X4, X5) OUROBOROS_PASTE1_1(FUNC, X6)
#define OUROBOROS_PASTE1_7(FUNC, X1, X2, X3, X4, X5, X6, X7) OUROBOROS_PASTE1_6(FUNC, X1, X2, X3, X4, X5, X6) OUROBOROS_PASTE1_1(FUNC, X7)
#define OUROBOROS_PASTE1_8(FUNC, X1, X2, X3, X4, X5, X6, X7, X8) OUROBOROS_PASTE1_7(FUNC, X1, X2, X3, X4, X5, X6, X7) OUROBOROS_PASTE1_1(FUNC, X8)
#define OUROBOROS_PASTE1_9(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9) OUROBOROS_PASTE1_8(FUNC, X1, X2, X3, X4, X5, X6, X7, X8) OUROBOROS_PASTE1_1(FUNC, X9)
#define OUROBOROS_PASTE1_10(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10) OUROBOROS_PASTE1_9(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9) OUROBOROS_PASTE1_1(FUNC, X10)
#define OUROBOROS_PASTE1_11(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11) OUROBOROS_PASTE1_10(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10) OUROBOROS_PASTE1_1(FUNC, X11)
#define OUROBOROS_PASTE1_12(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12) OUROBOROS_PASTE1_11(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11) OUROBOROS_PASTE1_1(FUNC, X12)
#define OUROBOROS_PASTE1_13(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13) OUROBOROS_PASTE1_12(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12) OUROBOROS_PASTE1_1(FUNC, X13)
#define OUROBOROS_PASTE1_14(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14) OUROBOROS_PASTE1_13(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13) OUROBOROS_PASTE1_1(FUNC, X14)
#define OUROBOROS_PASTE1_15(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15) OUROBOROS_PASTE1_14(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14) OUROBOROS_PASTE1_1(FUNC, X15)
#define OUROBOROS_PASTE1_16(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16) OUROBOROS_PASTE1_15(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15) OUROBOROS_PASTE1_1(FUNC, X16)
#define OUROBOROS_PASTE1_17(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17) OUROBOROS_PASTE1_16(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16) OUROBOROS_PASTE1_1(FUNC, X17)
#define OUROBOROS_PASTE1_18(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18) OUROBOROS_PASTE1_17(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17) OUROBOROS_PASTE1_1(FUNC, X18)
#define OUROBOROS_PASTE1_19(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19) OUROBOROS_PASTE1_18(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18) OUROBOROS_PASTE1_1(FUNC, X19)
#define OUROBOROS_PASTE1_20(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20) OUROBOROS_PASTE1_19(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19) OUROBOROS_PASTE1_1(FUNC, X20)
#define OUROBOROS_PASTE1_21(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21) OUROBOROS_PASTE1_20(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20) OUROBOROS_PASTE1_1(FUNC, X21)
#define OUROBOROS_PASTE1_22(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22) OUROBOROS_PASTE1_21(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21) OUROBOROS_PASTE1_1(FUNC, X22)
#define OUROBOROS_PASTE1_23(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23) OUROBOROS_PASTE1_22(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22) OUROBOROS_PASTE1_1(FUNC, X23)
#define OUROBOROS_PASTE1_24(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24) OUROBOROS_PASTE1_23(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23) OUROBOROS_PASTE1_1(FUNC, X24)
#define OUROBOROS_PASTE1_25(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25) OUROBOROS_PASTE1_24(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24) OUROBOROS_PASTE1_1(FUNC, X25)
#define OUROBOROS_PASTE1_26(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26) OUROBOROS_PASTE1_25(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25) OUROBOROS_PASTE1_1(FUNC, X26)
#define OUROBOROS_PASTE1_27(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27) OUROBOROS_PASTE1_26(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26) OUROBOROS_PASTE1_1(FUNC, X27)
#define OUROBOROS_PASTE1_28(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28) OUROBOROS_PASTE1_27(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27) OUROBOROS_PASTE1_1(FUNC, X28)
#define OUROBOROS_PASTE1_29(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29) OUROBOROS_PASTE1_28(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28) OUROBOROS_PASTE1_1(FUNC, X29)
#define OUROBOROS_PASTE1_30(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30) OUROBOROS_PASTE1_29(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29) OUROBOROS_PASTE1_1(FUNC, X30)
#define OUROBOROS_PASTE1_31(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31) OUROBOROS_PASTE1_30(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30) OUROBOROS_PASTE1_1(FUNC, X31)
#define OUROBOROS_PASTE1_32(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31, X32) OUROBOROS_PASTE1_31(FUNC, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31) OUROBOROS_PASTE1_1(FUNC, X32)

#define OUROBOROS_PASTE2_1(FUNC, X1, Y1) FUNC(X1, Y1)
#define OUROBOROS_PASTE2_2(FUNC, X1, Y1, X2, Y2) OUROBOROS_PASTE2_1(FUNC, X1, Y1) OUROBOROS_PASTE2_1(FUNC, X2, Y2)
#define OUROBOROS_PASTE2_3(FUNC, X1, Y1, X2, Y2, X3, Y3) OUROBOROS_PASTE2_2(FUNC, X1, Y1, X2, Y2) OUROBOROS_PASTE2_1(FUNC, X3, Y3)
#define OUROBOROS_PASTE2_4(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4) OUROBOROS_PASTE2_3(FUNC, X1, Y1, X2, Y2, X3, Y3) OUROBOROS_PASTE2_1(FUNC, X4, Y4)
#define OUROBOROS_PASTE2_5(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5) OUROBOROS_PASTE2_4(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4) OUROBOROS_PASTE2_1(FUNC, X5, Y5)
#define OUROBOROS_PASTE2_6(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6) OUROBOROS_PASTE2_5(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5) OUROBOROS_PASTE2_1(FUNC, X6, Y6)
#define OUROBOROS_PASTE2_7(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7) OUROBOROS_PASTE2_6(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6) OUROBOROS_PASTE2_1(FUNC, X7, Y7)
#define OUROBOROS_PASTE2_8(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8) OUROBOROS_PASTE2_7(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7) OUROBOROS_PASTE2_1(FUNC, X8, Y8)
#define OUROBOROS_PASTE2_9(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9) OUROBOROS_PASTE2_8(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8) OUROBOROS_PASTE2_1(FUNC, X9, Y9)
#define OUROBOROS_PASTE2_10(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10) OUROBOROS_PASTE2_9(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9) OUROBOROS_PASTE2_1(FUNC, X10, Y10)
#define OUROBOROS_PASTE2_11(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11) OUROBOROS_PASTE2_10(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10) OUROBOROS_PASTE2_1(FUNC, X11, Y11)
#define OUROBOROS_PASTE2_12(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12) OUROBOROS_PASTE2_11(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11) OUROBOROS_PASTE2_1(FUNC, X12, Y12)
#define OUROBOROS_PASTE2_13(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13) OUROBOROS_PASTE2_12(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12) OUROBOROS_PASTE2_1(FUNC, X13, Y13)
#define OUROBOROS_PASTE2_14(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14) OUROBOROS_PASTE2_13(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13) OUROBOROS_PASTE2_1(FUNC, X14, Y14)
#define OUROBOROS_PASTE2_15(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15) OUROBOROS_PASTE2_14(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14) OUROBOROS_PASTE2_1(FUNC, X15, Y15)
#define OUROBOROS_PASTE2_16(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16) OUROBOROS_PASTE2_15(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15) OUROBOROS_PASTE2_1(FUNC, X16, Y16)
#define OUROBOROS_PASTE2_17(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17) OUROBOROS_PASTE2_16(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16) OUROBOROS_PASTE2_1(FUNC, X17, Y17)
#define OUROBOROS_PASTE2_18(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18) OUROBOROS_PASTE2_17(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17) OUROBOROS_PASTE2_1(FUNC, X18, Y18)
#define OUROBOROS_PASTE2_19(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19) OUROBOROS_PASTE2_18(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18) OUROBOROS_PASTE2_1(FUNC, X19, Y19)
#define OUROBOROS_PASTE2_20(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20) OUROBOROS_PASTE2_19(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19) OUROBOROS_PASTE2_1(FUNC, X20, Y20)
#define OUROBOROS_PASTE2_21(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21) OUROBOROS_PASTE2_20(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20) OUROBOROS_PASTE2_1(FUNC, X21, Y21)
#define OUROBOROS_PASTE2_22(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22) OUROBOROS_PASTE2_21(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21) OUROBOROS_PASTE2_1(FUNC, X22, Y22)
#define OUROBOROS_PASTE2_23(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23) OUROBOROS_PASTE2_22(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22) OUROBOROS_PASTE2_1(FUNC, X23, Y23)
#define OUROBOROS_PASTE2_24(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24) OUROBOROS_PASTE2_23(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23) OUROBOROS_PASTE2_1(FUNC, X24, Y24)
#define OUROBOROS_PASTE2_25(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25) OUROBOROS_PASTE2_24(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24) OUROBOROS_PASTE2_1(FUNC, X25, Y25)
#define OUROBOROS_PASTE2_26(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26) OUROBOROS_PASTE2_25(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25) OUROBOROS_PASTE2_1(FUNC, X26, Y26)
#define OUROBOROS_PASTE2_27(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27) OUROBOROS_PASTE2_26(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26) OUROBOROS_PASTE2_1(FUNC, X27, Y27)
#define OUROBOROS_PASTE2_28(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28) OUROBOROS_PASTE2_27(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27) OUROBOROS_PASTE2_1(FUNC, X28, Y28)
#define OUROBOROS_PASTE2_29(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29) OUROBOROS_PASTE2_28(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28) OUROBOROS_PASTE2_1(FUNC, X29, Y29)
#define OUROBOROS_PASTE2_30(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29, X30, Y30) OUROBOROS_PASTE2_29(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29) OUROBOROS_PASTE2_1(FUNC, X30, Y30)
#define OUROBOROS_PASTE2_31(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29, X30, Y30, X31, Y31) OUROBOROS_PASTE2_30(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29, X30, Y30) OUROBOROS_PASTE2_1(FUNC, X31, Y31)
#define OUROBOROS_PASTE2_32(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29, X30, Y30, X31, Y31, X32, Y32) OUROBOROS_PASTE2_31(FUNC, X1, Y1, X2, Y2, X3, Y3, X4, Y4, X5, Y5, X6, Y6, X7, Y7, X8, Y8, X9, Y9, X10, Y10, X11, Y11, X12, Y12, X13, Y13, X14, Y14, X15, Y15, X16, Y16, X17, Y17, X18, Y18, X19, Y19, X20, Y20, X21, Y21, X22, Y22, X23, Y23, X24, Y24, X25, Y25, X26, Y26, X27, Y27, X28, Y28, X29, Y29, X30, Y30, X31, Y31) OUROBOROS_PASTE2_1(FUNC, X32, Y32)

#define OUROBOROS_TYPE_DEF_STRUCT(Type, ...)                                                                                         \
    struct Type                                                                                                                      \
    {                                                                                                                                \
        OUROBOROS_EXPAND(OUROBOROS_PASTE2(OUROBOROS_FIELD, __VA_ARGS__))                                                             \
    };                                                                                                                               \
    template <>                                                                                                                      \
    struct TypeInfo<Type>                                                                                                            \
    {                                                                                                                                \
        static nlohmann::json type_info()                                                                                            \
        {                                                                                                                            \
            return nlohmann::json{{"struct", nlohmann::json{OUROBOROS_EXPAND(OUROBOROS_PASTE2(OUROBOROS_TYPE_INFO, __VA_ARGS__))}}}; \
        }                                                                                                                            \
    };

#define OUROBOROS_TYPE_DEF_ENUM(Type, ...)                                                                                         \
    enum Type                                                                                                                      \
    {                                                                                                                              \
        OUROBOROS_EXPAND(OUROBOROS_PASTE1(OUROBOROS_VARIANT, __VA_ARGS__))                                                         \
    };                                                                                                                             \
    template <>                                                                                                                    \
    struct TypeInfo<Type>                                                                                                          \
    {                                                                                                                              \
        static nlohmann::json type_info()                                                                                          \
        {                                                                                                                          \
            return nlohmann::json{{"enum", nlohmann::json{OUROBOROS_EXPAND(OUROBOROS_PASTE1(OUROBOROS_TYPE_NAME, __VA_ARGS__))}}}; \
        }                                                                                                                          \
    };