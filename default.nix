{ pkgs ? import <nixpkgs> {} }:
with pkgs;

let
  cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
in
rustPlatform.buildRustPackage {
  cargoLock.lockFile = ./Cargo.lock;

  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  meta = {
    description = pkg.description;
    homepage = pkg.homepage;
    license = lib.getLicenseFromSpdxId cargoToml.package.license;
    mainProgram = pkg.name;
    platforms = lib.platforms.darwin;
    maintainers = [{ name = "Aditya Singh"; email = "antichrome1@proton.me"; }];
  };

  src = lib.cleanSource ./.;
}
