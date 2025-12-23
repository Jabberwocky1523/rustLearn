cmake_minimum_required(VERSION 3.20)
project(test)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# add_subdirectory(ons)
include_directories(${PROJECT_SOURCE_DIR})
aux_source_directory(${PROJECT_SOURCE_DIR} SRC)

add_executable(test ${SRC})
target_link_libraries(test ${PROJECT_SOURCE_DIR}/librt.so)
