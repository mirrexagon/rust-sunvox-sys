with import <nixpkgs> {};

stdenv.mkDerivation rec {
  name = "rust-env";
  buildInputs = [ stdenv pkgconfig openssl.dev cargo rustc alsaLib SDL binutils sunvox-dll ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

  SSL_CERT_FILE = "/etc/ssl/certs/ca-certificates.crt";
}
