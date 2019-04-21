cmake_minimum_required(VERSION 3.0)
project(process_monitor)

# Preprocessor definitions
add_definitions(-std=c++11)
add_definitions(-DPROCESS_MONITOR_PROJECT_ROOT="${CMAKE_SOURCE_DIR}")
add_definitions(-DPROCESS_MONITOR_VERSION="0.0.1")

add_subdirectory(src)
add_subdirectory(test)
