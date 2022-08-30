with import <nixpkgs> {};
stdenv.mkDerivation {
  name = "myenv";
  buildInputs = [ alsa-lib ];
  nativeBuildInputs = [ pkg-config ];
}
