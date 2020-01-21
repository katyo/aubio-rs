{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let stdenv = clangStdenv;
in stdenv.mkDerivation {
  name = "aubio";

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";

  buildInputs = [
    pkgconfig
    aubio
    libav
    libsndfile
    libsamplerate
    fftw
    jack2
  ];
}
