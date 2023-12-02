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

        T set(const T &t)
        {
            // Open the file descriptor
            std::fstream f(this->fd, std::ios::in | std::ios::out | std::ios::trunc);
            if (!f.is_open())
            {
                throw std::runtime_error("error opening object file: " + this->fd);
            }

            // Use nlohmann/json to serialize
            nlohmann::json j = t;
            f << j.dump();

            // Flush and close the file
            f.flush();
            f.close();

            return t;
        }

        T get()
        {
            // Open the file descriptor
            std::fstream f(this->fd, std::ios::in);
            if (!f.is_open())
            {
                throw std::runtime_error("error opening object file: " + this->fd);
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
            j = nlohmann::json{{"filename", obj.getFilename()}};
        }

        friend void from_json(const nlohmann::json &j, Object<T> &obj)
        {
            std::string filename = j.at("filename").get<std::string>();
            obj = Object<T>(filename);
        }
    };
}
