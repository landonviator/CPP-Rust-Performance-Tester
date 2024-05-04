cmake_minimum_required(VERSION 3.24)
project(Tester)

set(CMAKE_CXX_STANDARD 17)

add_executable(Tester main.cpp)

# Link the Rust library
find_library(RUST_LIBRARY NAMES rust librust PATHS ${CMAKE_SOURCE_DIR}/rust/target/release)
if(NOT RUST_LIBRARY)
    message(FATAL_ERROR "Rust library not found")
endif()

target_link_libraries(Tester ${RUST_LIBRARY})
