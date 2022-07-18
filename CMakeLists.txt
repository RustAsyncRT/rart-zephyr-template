####################################################################
# This project has three parts:
#   1. The Zephyr app target, which is the minimum additions of
#      any firmware build with Zephyr SO;
#   2. The Rust library made by the user;
#   3. The backend of RART built for Zephyr SO;
# The part 2 and 3 is merged into a library, with the project name,
# and further linked with the Zephyr app target.
####################################################################
cmake_minimum_required(VERSION 3.13.1)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(rart_zephyr_template)

target_sources(app PRIVATE src/main.c)

# The name of rs-lib project (package/name at Cargo.toml)
set(RS_LIB_NAME rart_zephyr_template_rs)
# The path of rart-c library
set(RART_C_PATH ${CMAKE_SOURCE_DIR}/3rdparty/rart-c/zephyr)
# Including rart-c CMakeLists.txt
add_subdirectory(${RART_C_PATH})
# Add generated include directory
include_directories(app PUBLIC ${CMAKE_SOURCE_DIR}/3rdparty/rart-c/generated)
# Creating a library, with the project name, to include rs-lib and rart-c
add_library(${PROJECT_NAME} STATIC IMPORTED GLOBAL)
# Adding rs-lib
set_target_properties(${PROJECT_NAME} PROPERTIES IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/rs_lib/target/thumbv7em-none-eabi/release/${RS_LIB_NAME}.a)
# Adding rart-c to library
set_target_properties(${PROJECT_NAME} PROPERTIES INTERFACE_INCLUDE_DIRECTORIES ${RART_C_PATH})
# Linking library to Zephyr app target
target_link_libraries(app PUBLIC ${PROJECT_NAME})
