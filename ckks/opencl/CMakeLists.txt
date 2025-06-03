cmake_minimum_required(VERSION 3.20)

project(test_opencl LANGUAGES CXX)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_VERBOSE_MAKEFILE ON)
# set(CMAKE_CXX_STANDARD 20)

find_package(OpenCL REQUIRED)

if(NOT CMAKE_BUILD_TYPE)
	set(CMAKE_BUILD_TYPE Debug CACHE STRING "build type" FORCE)
endif()

# needed for nvim
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE INTERNAL "")
if(CMAKE_EXPORT_COMPILE_COMMANDS)
  set(CMAKE_CXX_STANDARD_INCLUDE_DIRECTORIES 
      ${CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES})
endif()


# Set additionally our own debug/optimization compiler/linkage flags. Maybe not
# he nicest way, but by far the easiest.
set(DEBUG_FLAGS "-g -O0 -Wall -Wextra -Wpedantic -fpermissive -DDEBUG -lOpenCL")
set(OPTIMIZE_FLAGS "-O3 -flto -DNDEBUG -fno-exceptions")

set(CMAKE_C_FLAGS_DEBUG "${CMAKE_C_FLAGS_DEBUG} ${DEBUG_FLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} ${DEBUG_FLAGS}")
set(CMAKE_C_FLAGS_RELEASE "${CMAKE_C_FLAGS_RELEASE} ${OPTIMIZE_FLAGS}")
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} ${OPTIMIZE_FLAGS}")

add_executable(test test.cpp)
target_link_libraries(test OpenCL::OpenCL)

add_executable(ntt ntt.cpp init.cpp)
target_link_libraries(ntt OpenCL::OpenCL)
