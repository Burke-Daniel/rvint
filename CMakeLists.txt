cmake_minimum_required(VERSION 3.10)

project(rvint)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(src_files
   rvint.cpp 
)

add_executable(rvint ${src_files})
