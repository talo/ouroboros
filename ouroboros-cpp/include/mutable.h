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

        T set(const T &t)
        {
            // Open the file descriptor
            std::fstream f(this->fd, std::ios::in | std::ios::out | std::ios::trunc);
            if (!f.is_open())
            {
                throw std::runtime_error("error opening Mutable file: " + this->fd);
            }

            // Use nlohmann/json to serialize
            nlohmann::json j = t;
            f << j.dump();

            // Flush and close the file
            f.flush();
            f.close();

            return t;
        }
    };
}