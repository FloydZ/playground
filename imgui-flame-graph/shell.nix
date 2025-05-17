with import <nixpkgs> {};
{ pkgs ? import <nixpkgs> {} }:
let 
  general_inputs = [
    gnumake 
    cmake
    # needed for libOpenCL.so
    clang
    ocl-icd
    clinfo
    # needed for <CL/cl.h>
    opencl-headers
    opencl-clhpp
    # needed for <Gl/gl.h>
    libGL
    libGLU
  
    # c++
    pkg-config
    python3
    glew
    glfw
    gl3w
    SDL2

    # rust
    cargo 
    rustc
    rustup
    rust-analyzer
    llvm
    lldb
    clang
    xorg.libX11 
    xorg.xwd 
    xorg.xdm
    xorg.libXcursor
    xorg.libXi
    libxkbcommon
  ];
in
stdenv.mkDerivation {
  name = "flamegraph_example";
  src = ./.;

  # AMD STUFF
  #buildInputs = general_inputs ++ [
  #  rocmPackages.clang-ocl
  #  rocmPackages.rocm-runtime
  #  rocmPackages.llvm.libcxx 
  #  rocmPackages.rocminfo
  #  #BROKEN: rocmPackages.llvm.libclc
  #  rocm-opencl-icd
  #  rocm-opencl-runtime
  #];
  
  # Intel
  #buildInputs = general_inputs ++ [
  #  intel-compute-runtime
  #  intel-ocl
  #];

  # NVIDIA
  buildInputs = general_inputs ++ [
    cudatoolkit
    cudaPackages.cuda_nvcc
  ];

  shellHook = ''
     # export CUDA_PATH=${pkgs.cudatoolkit}
     # export LD_LIBRARY_PATH=${pkgs.linuxPackages.nvidia_x11}/lib:${pkgs.ncurses5}/lib
     # export EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib"
     # export EXTRA_CCFLAGS="-I/usr/include"

     mkdir -p build 
     cd build 
     cmake .. -DIMGUI_WITH_BACKEND=ON -DIMGUI_BACKEND_PLATFORM=SDL 
     make
     cd ..
  '';
    LD_LIBRARY_PATH = "${lib.makeLibraryPath general_inputs}";
}
