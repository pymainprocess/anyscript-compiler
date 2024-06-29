# Hier setze ich den Mindest Toolchain auf 3.10 von Cmake
cmake_minimum_required(VERSION 3.10)
# Hier definiere ich den Projekt Namen
project(anyscript-compiler)
# Hier aktiviere ich die ausfhrliche Ausgabe
set(CMAKE_VERBOSE_MAKEFILE ON)
# Hier fge ich die CargoSetup.cmake ein
include(${CMAKE_SOURCE_DIR}/CargoSetup.cmake)
# Jetzt werden wichtige Umgebungs Variablen Definiert
set(DEBUG_BUILD OFF)
set(OUTPUT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/output)
set(NINJA_SUPPORT OFF)
# Nun werden wir die Variablen verwenden, um unseren Build Zu steuern
if (NOT DEBUG_BUILD)
    set(CMAKE_BUILD_TYPE Release)
    set(CMAKE_BUILD_FLAG ${CARGO_COMPILER_RELEASE})
else()
    set(CMAKE_BUILD_TYPE Debug)
    set(CMAKE_BUILD_FLAG ${CARGO_COMPILER_DEBUG})
endif()
# Hier wird Kontrolliert, ob man einen Build Verzeichnis erstellt hat und man sich daher in ${CMAKE_SOURCE_DIR}/build befindet
if (EXISTS "${CMAKE_SOURCE_DIR}/build" AND "${CMAKE_BINARY_DIR}" STREQUAL "${CMAKE_SOURCE_DIR}/build")
    message(STATUS "Build Directory is ${CMAKE_SOURCE_DIR}/build")
else()
    message(FATAL_ERROR "Build Directory is ${CMAKE_BINARY_DIR}")
endif()