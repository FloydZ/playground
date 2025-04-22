with import <nixpkgs> {};
{ pkgs ? import <nixpkgs> {} }:

stdenv.mkDerivation {
  name = "bash_get_load_cpus";
  src = ./.;

  buildInputs = [ 
    clang
    bc 
    sysstat
  ];
}
