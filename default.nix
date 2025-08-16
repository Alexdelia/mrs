{
  pkgs ? import <nixpkgs> {},
  rustToolchain,
}: let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };
in
  rustPlatform.buildRustPackage
  {
    pname = "mrs";
    version = "0.1.0";

    src = pkgs.lib.cleanSource ./.;

    # cargoSha256 = pkgs.lib.fakeSha256;
    cargoLock = {
      lockFile = ./Cargo.lock;
    };
  }
