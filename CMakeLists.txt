cmake_minimum_required(VERSION 3.26)
project(peng_wrapper)

set(CMAKE_CXX_STANDARD 23)

add_subdirectory(PengEngine)

add_library(
    peng_wrapper
    SHARED
    wrapper/engine.h
    wrapper/engine.cpp)

target_link_libraries(peng_wrapper PUBLIC peng)
