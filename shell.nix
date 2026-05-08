{ pkgs ? import <nixpkgs> {
    overlays = [ (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")) ];
  }
}:

let
  rustToolchain = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" "rust-analyzer" ];
    targets = [
      "aarch64-linux-android"
      "armv7-linux-androideabi"
      "x86_64-linux-android"
      "i686-linux-android"
    ];
  };
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustToolchain
    cmake
    pkg-config
    openssl
    dbus
    maven
  ];

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
    openssl
    dbus
    libgcc
  ]);
}
