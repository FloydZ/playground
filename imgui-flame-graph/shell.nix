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

    python3
    glew
    glfw
    gl3w
    SDL2
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

  OCL_ICD_VENDORS="/run/opengl-driver/etc/OpenCL/vendors";
  OPENCL_LIBRARY="${ocl-icd}/lib/libOpenCL.so";
  OPENCL_HEADERS="${opencl-headers}/include/CL";

  # run cmake with: 'cmake .. -DOpenCL_LIBRARY=${OPENCL_LIBRARY} -DOpenCL_INCLUDE_DIR=${OPENCL_HEADERS}`
}
