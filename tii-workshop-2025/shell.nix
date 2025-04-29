with import <nixpkgs> {};
let 
  bInputs = [ 
    # just for cargo 
    clang 
    llvm 
    lldb
    cargo
    rustc
    rustup

    # needed for p62 
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
