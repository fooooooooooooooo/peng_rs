cmake_minimum_required(VERSION 3.26)
project(peng_wrapper)

set(CMAKE_CXX_STANDARD 23)

add_subdirectory(PengEngine)

add_library(
    peng_wrapper
    SHARED
    wrapper/wrapper.h
    wrapper/w_engine.h
    wrapper/w_engine.cpp
    wrapper/w_vector2.cpp
    wrapper/w_vector2.h
    wrapper/w_vector2.cpp
    wrapper/w_logging.cpp
    wrapper/w_logging.h
    wrapper/w_window_subsystem.cpp
    wrapper/w_window_subsystem.h)

target_link_libraries(peng_wrapper PUBLIC peng)
