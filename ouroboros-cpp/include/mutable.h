#pragma once

#include <fstream>
#include <string>
#include <system_error> // for std::error_code
#include <iostream>

#include "json.h"
#include "uuid.h"

namespace ouroboros
{

    template <typename T>
    class Mutable
    {
    private:
        std::string m_fd;

    public:
        // Constructor
        Mutable()
        {
            this->m_fd = ouroboros::generate_uuid();
        }

        // Constructor with provided file descriptor
        Mutable(const std::string &fd)
        {
            this->m_fd = fd;
        }

        // Override `<<` operator to call `assign`
        T operator<<(const T &t)
        {
            return this->assign(t);
        }

        // Assign a value to the mutable, which will cause it to write the value
        // to its underlying file descriptor.
        T assign(const T &t)
        {
            // Open the file descriptor
            std::fstream f(this->m_fd, std::ios::in | std::ios::out | std::ios::trunc);
            if (!f.is_open())
            {
                throw std::runtime_error("error opening Mutable file: " + this->m_fd);
            }

            // Use nlohmann/json to serialize
            nlohmann::json j = t;
            f << j.dump();

            // Flush and close the file
            f.flush();
            f.close();

            return t;
        }

        friend void from_json(const nlohmann::json &j, Mutable<T> &mut)
        {
            std::string fd = j;
            mut = Mutable<T>(fd);
        }
    };
}