with import <nixpkgs> {};
{ pkgs ? import <nixpkgs> {} }:

stdenv.mkDerivation {
  name = "ntt-opencl";
  src = ./.;


  # AMD STUFF
  #buildInputs = [
  #  rocmPackages.clang-ocl
  #  rocmPackages.rocm-runtime
  #  rocmPackages.llvm.libcxx 
  #  rocmPackages.rocminfo
  #  #BROKEN: rocmPackages.llvm.libclc
  #  # needed for libOpenCL.so
  #  ocl-icd
  #  clinfo
  #  rocm-opencl-icd
  #  rocm-opencl-runtime
  #  # needed for <CL/cl.h>
  #  opencl-headers
  #  opencl-clhpp
  #  # needed for <Gl/gl.h>
  #  libGL
  #  libGLU
  #];
  
  # Intel
  buildInputs = [
    # needed for libOpenCL.so
    clang
    ocl-icd
    clinfo
    #intel-compute-runtime
    #intel-ocl
    # needed for <CL/cl.h>
    opencl-headers
    opencl-clhpp
    # needed for <Gl/gl.h>
    libGL
    libGLU
  ];

  # NVIDIA
  #buildInputs = [
  #  # needed for libOpenCL.so
  #  clang
  #  clinfo
  #  ocl-icd
  #  cudatoolkit
  #  cudaPackages.cuda_nvcc
  #  # needed for <CL/cl.h>
  #  opencl-headers
  #  opencl-clhpp
  #  # needed for <Gl/gl.h>
  #  libGL
  #  libGLU
  #];
  #shellHook = ''
  #   export CUDA_PATH=${pkgs.cudatoolkit}
  #   # export LD_LIBRARY_PATH=${pkgs.linuxPackages.nvidia_x11}/lib:${pkgs.ncurses5}/lib
  #   export EXTRA_LDFLAGS="-L/lib -L${pkgs.linuxPackages.nvidia_x11}/lib"
  #   export EXTRA_CCFLAGS="-I/usr/include"
  #'';        


  OCL_ICD_VENDORS="/run/opengl-driver/etc/OpenCL/vendors";
  OPENCL_LIBRARY="${ocl-icd}/lib/libOpenCL.so";
  OPENCL_HEADERS="${opencl-headers}/include/CL";

  # run cmake with: 'cmake .. -DOpenCL_LIBRARY=${OPENCL_LIBRARY} -DOpenCL_INCLUDE_DIR=${OPENCL_HEADERS}`
}
