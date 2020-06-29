with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "toy-project-env";

  nativeBuildInputs = [
    rustc cargo
  ];

  RUST_BACKTRACE = 1;

  LD_LIBRARY_PATH = with xlibs; "${mesa}/lib:${libX11}/lib:${libXcursor}/lib:${libXxf86vm}/lib:${libXi}/lib:${libXrandr}/lib:${libGL}/lib";
}
