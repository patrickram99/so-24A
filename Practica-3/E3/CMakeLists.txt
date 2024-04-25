cmake_minimum_required(VERSION 3.27)
project(E3)

set(CMAKE_C_STANDARD 99)

include_directories(.)

add_executable(E3
        mensaje.c
        salida_alt.c
        salida_alt.h)

