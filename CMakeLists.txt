# Hier setze ich den Mindest Toolchain auf 3.10 von Cmake
cmake_minimum_required(VERSION 3.10)
# Hier definiere ich den Projekt Namen
project(anyscript-compiler)
# Hier aktiviere ich die ausfhrliche Ausgabe
set(CMAKE_VERBOSE_MAKEFILE ON)
# Hier fge ich die CargoSetup.cmake ein
include(${CMAKE_SOURCE_DIR}/CargoSetup.cmake)