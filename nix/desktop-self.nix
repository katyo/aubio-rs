{ pkgs ? import <nixpkgs> {}, ... }:
with pkgs;
let stdenv = llvmPackages_11.stdenv; #clangStdenv;
in stdenv.mkDerivation {
  name = "aubio";

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";

  buildInputs = [
    pkgconfig
    #libav
    #libsndfile
    #libsamplerate
    #fftw
    #fftwFloat
    #jack2

    # build deps
    openssl
    gnupg
  ];
}
