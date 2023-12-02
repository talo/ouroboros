#pragma once

#include <fstream>
#include <string>
#include <system_error> // for std::error_code
#include <iostream>

#include "type_info.h"
#include "json.h"
#include "uuid.h"

namespace ouroboros
{

    template <typename T>
    class Object
    {
    private:
        std::string m_fd;

    public:
        // Constructor
        Object()
        {
            this->m_fd = ouroboros::generate_uuid();
        }

        // Constructor with provided file descriptor
        Object(const std::string &fd)
        {
            this->m_fd = fd;
        }

        // Override `<<` operator to call `assign`
        Object<T> operator<<(const T &t)
        {
            return this->assign(t);
        }

        // Override `>>` operator to call `get`
        Object<T> operator>>(T &t)
        {
            auto v = this->get();
            t = v;
            return *this;
        }

        // Assign a value to the object, which will cause it to write the value
        // to its underlying file descriptor.
        Object<T> assign(const T &t)
        {
            // Open the file descriptor
            std::fstream f(this->m_fd, std::ios::in | std::ios::out | std::ios::trunc);
            if (!f.is_open())
            {
                throw std::runtime_error("error opening object file: " + this->m_fd);
            }

            // Use nlohmann/json to serialize
            nlohmann::json j = t;
            f << j.dump();

            // Flush and close the file
            f.flush();
            f.close();

            return *this;
        }

        T get()
        {
            // Open the file descriptor
            std::fstream f(this->m_fd, std::ios::in);
            if (!f.is_open())
            {
                throw std::runtime_error("error opening object file: " + this->m_fd);
            }

            // Use nlohmann/json to deserialize
            nlohmann::json j;
            f >> j;
            T t = j.get<T>();

            // Close the file
            f.close();

            return t;
        }

        friend void to_json(nlohmann::json &j, const Object<T> &obj)
        {
            j = obj.m_fd;
        }

        friend void from_json(const nlohmann::json &j, Object<T> &obj)
        {
            std::string fd = j;
            obj = Object<T>(fd);
        }
    };

    template <typename T>
    struct TypeInfo<Object<T>>
    {
        static nlohmann::json type_info()
        {
            return nlohmann::json{
                {"k", "record"}, {"value", "string"}};
        }
    };
}
