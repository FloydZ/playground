with import <nixpkgs> {};
let 
  unstable_pkgs = import <nixos-unstable> {};
  bInputs = [ 
    # C++ stuff 
    cmake 
    gnumake 
    mpfr
    clang 
    llvm 
    gnum4

    # just for cargo 
    unstable_pkgs.cargo
    unstable_pkgs.rustc
    unstable_pkgs.rustup
    rust-bindgen
    rust-cbindgen

    # needed for debugging
    gdb
    lldb
  ] ++ (lib.optionals pkgs.stdenv.isLinux ([
  ]));
in
{ pkgs ? import <nixpkgs> {} }:

stdenv.mkDerivation {
  name = "";
  src = ./.;

  buildInputs = bInputs; #with pkgs; [ ];
  nativeBuildInputs = with pkgs; [addOpenGLRunpath];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
