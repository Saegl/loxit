cmake_minimum_required(VERSION 3.25)

project(LoxProgrammingLang C)

add_executable(
    loxit
    csrc/main.c
)

target_include_directories(loxit PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/csrc)
