# Install script for directory: /home/duda/Downloads/programming/playground/imgui-flame-graph

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/dist")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set path to fallback-tool for dependency-resolution.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/nix/store/mzfk1sdv6bxvdh9slvddpj734ck99idi-gcc-wrapper-13.3.0/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib64" TYPE STATIC_LIBRARY FILES "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/libimgui.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/imgui" TYPE FILE FILES
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/imgui.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/imconfig.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/imgui_internal.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/imstb_rectpack.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/imstb_textedit.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/imstb_truetype.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/backends/imgui_impl_sdl2.h"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/backends/imgui_impl_opengl3.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/misc/fonts" TYPE FILE FILES
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/misc/fonts/Cousine-Regular.ttf"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/misc/fonts/DroidSans.ttf"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/misc/fonts/Karla-Regular.ttf"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/misc/fonts/ProggyClean.ttf"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/misc/fonts/ProggyTiny.ttf"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/imgui/misc/fonts/Roboto-Medium.ttf"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui/ImGuiTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui/ImGuiTargets.cmake"
         "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/CMakeFiles/Export/1c9f1db1a0c9398944d65050189647a4/ImGuiTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui/ImGuiTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui/ImGuiTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui" TYPE FILE FILES "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/CMakeFiles/Export/1c9f1db1a0c9398944d65050189647a4/ImGuiTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui" TYPE FILE FILES "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/CMakeFiles/Export/1c9f1db1a0c9398944d65050189647a4/ImGuiTargets-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/imgui" TYPE FILE FILES
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/generated/ImGuiConfigVersion.cmake"
    "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/generated/ImGuiConfig.cmake"
    )
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for the subdirectory.
  include("/home/duda/Downloads/programming/playground/imgui-flame-graph/build/example/cmake_install.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT)
  if(CMAKE_INSTALL_COMPONENT MATCHES "^[a-zA-Z0-9_.+-]+$")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
  else()
    string(MD5 CMAKE_INST_COMP_HASH "${CMAKE_INSTALL_COMPONENT}")
    set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INST_COMP_HASH}.txt")
    unset(CMAKE_INST_COMP_HASH)
  endif()
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
  file(WRITE "/home/duda/Downloads/programming/playground/imgui-flame-graph/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
