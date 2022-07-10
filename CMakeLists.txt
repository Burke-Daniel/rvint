cmake_minimum_required(VERSION 3.10)

project(rvint)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(src_files
   rvint.cpp 
   src/parser.cpp
   src/tokenizer.cpp
   src/interpreter.cpp
)

add_executable(rvint ${src_files})

target_include_directories(rvint PUBLIC
    inc/
)

