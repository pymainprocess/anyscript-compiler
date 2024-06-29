# Hier setze ich den Mindest Toolchain auf 3.10 von Cmake
cmake_minimum_required(VERSION 3.10)

# Hier definiere ich den Projekt Namen
project(anyscript-compiler)

# Hier aktiviere ich die ausführliche Ausgabe
set(CMAKE_VERBOSE_MAKEFILE ON)

# Hier füge ich die CargoSetup.cmake ein
include(${CMAKE_SOURCE_DIR}/CargoSetup.cmake)

# Jetzt werden wichtige Umgebungsvariablen definiert
set(DEBUG_BUILD OFF)
set(OUTPUT_DIR ${CMAKE_CURRENT_SOURCE_DIR}/output)
set(NINJA_SUPPORT OFF)

# Nun werden wir die Variablen verwenden, um unseren Build zu steuern
if (NOT DEBUG_BUILD)
    set(CMAKE_BUILD_TYPE Release)
    set(CMAKE_BUILD_FLAG ${CARGO_COMPILER_RELEASE})
else()
    set(CMAKE_BUILD_TYPE Debug)
    set(CMAKE_BUILD_FLAG ${CARGO_COMPILER_DEBUG})
endif()

# Hier wird kontrolliert, ob man ein Build-Verzeichnis erstellt hat und man sich daher in ${CMAKE_SOURCE_DIR}/build befindet
if (EXISTS "${CMAKE_SOURCE_DIR}/build" AND "${CMAKE_BINARY_DIR}" STREQUAL "${CMAKE_SOURCE_DIR}/build")
    message(STATUS "Build-Verzeichnis ist ${CMAKE_SOURCE_DIR}/build")
else()
    message(FATAL_ERROR "Build-Verzeichnis ist ${CMAKE_BINARY_DIR}")
endif()

# Funktion zum Überprüfen und Installieren von Cargo-Paketen
function(find_or_install_package pkg)
    find_package(${pkg} QUIET)
    if (NOT ${pkg}_FOUND)
        message(STATUS "Package: ${pkg} not found. Installing with Cargo...")
        execute_process(COMMAND cargo install ${pkg} RESULT_VARIABLE result)
        if (result)
            message(FATAL_ERROR "Failed to install: ${pkg}. Please update Cargo or install Cargo.")
        else()
            message(STATUS "Successfully installed ${pkg}")
        endif()
    endif()
endfunction()