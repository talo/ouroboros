#include "../include/uuid.h"

#include <iostream>

int main()
{
    std::cout << ouroboros::generate_uuid() << std::endl;
    return 0;
}