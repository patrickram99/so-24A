cmake_minimum_required(VERSION 3.27)
project(E1)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)

add_executable(E1
        main.cpp
        LinkedList.cpp
        LinkedList.h
        ListNode.cpp
        ListNode.h)
