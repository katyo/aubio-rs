{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let stdenv = clangStdenv;
in stdenv.mkDerivation {
  name = "aubio";

  buildInputs = [
    pkgconfig
    libav
    libsndfile
    libsamplerate
    fftw
    jack2
  ];
}
