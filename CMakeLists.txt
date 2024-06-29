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
set(OUTPUT_DIR ${CMAKE_SOURCE_DIR}/build/output)
set(DOC_OUTPUT_DIR ${OUTPUT_DIR}/doc)
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
    message(STATUS "Build Directory is ${CMAKE_SOURCE_DIR}/build")
else()
    message(FATAL_ERROR "Build Directory is ${CMAKE_BINARY_DIR}")
endif()

# Function to check and install Cargo packages
function(find_or_in_pkg pkg)
    find_program(${pkg} QUIET)
    if (NOT ${pkg}_FOUND)
        message(STATUS "Package: ${pkg} not Found. Install it with Cargo ...")
        execute_process(COMMAND cargo install ${pkg} RESULT_VARIABLE result)
        if (result)
            message(FATAL_ERROR "Failed to install: ${pkg}, pls update cargo or install cargo")
        else()
            message(STATUS "Successfully installed ${pkg}")
        endif()
    endif()
endfunction()

# Ensure PATH includes common directories for user-installed cargo binaries
set(ENV{PATH} "$ENV{HOME}/.cargo/bin:$ENV{PATH}")

# Find required programs
find_program(CARGO_EXECUTABLE NAMES cargo)
if (NOT CARGO_EXECUTABLE)
    message(FATAL_ERROR "Cargo not found. Please ensure it is installed and in your PATH.")
else()
    message(STATUS "Found Cargo: ${CARGO_EXECUTABLE}")
endif()

find_program(RUSTC_EXECUTABLE NAMES rustc)
if (NOT RUSTC_EXECUTABLE)
    message(FATAL_ERROR "Rustc not found. Please ensure it is installed and in your PATH.")
else()
    message(STATUS "Found Rustc: ${RUSTC_EXECUTABLE}")
endif()

find_program(CARGO_CLIPPY_EXECUTABLE NAMES cargo-clippy)
if (NOT CARGO_CLIPPY_EXECUTABLE)
    message(FATAL_ERROR "Cargo-clippy not found. Please ensure it is installed and in your PATH.")
else()
    message(STATUS "Found Cargo-clippy: ${CARGO_CLIPPY_EXECUTABLE}")
endif()

find_program(RUSTDOC_EXECUTABLE NAMES rustdoc)
if (NOT RUSTDOC_EXECUTABLE)
    message(FATAL_ERROR "Rustdoc not found. Please ensure it is installed and in your PATH.")
else()
    message(STATUS "Found Rustdoc: ${RUSTDOC_EXECUTABLE}")
endif()

find_program(RUSTFMT_EXECUTABLE NAMES rustfmt)
if (NOT RUSTFMT_EXECUTABLE)
    message(FATAL_ERROR "Rustfmt not found. Please ensure it is installed and in your PATH.")
else()
    message(STATUS "Found Rustfmt: ${RUSTFMT_EXECUTABLE}")
endif()

find_program(PKG_CONFIG_EXECUTABLE NAMES pkg-config)
if (NOT PKG_CONFIG_EXECUTABLE)
    message(FATAL_ERROR "Pkg-config not found. Please ensure it is installed and in your PATH.")
else()
    message(STATUS "Found Pkg-config: ${PKG_CONFIG_EXECUTABLE}")
endif()

find_package(OpenSSL REQUIRED)

add_custom_target(docs
    COMMAND mkdir -p ${DOC_OUTPUT_DIR}
    COMMAND ${CARGO_COMPILER} ${CARGO_COMPILER_DOC} ${CMAKE_BUILD_FLAG} --target-dir ${DOC_OUTPUT_DIR}
)