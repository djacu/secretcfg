{
  lib,
  rustPlatform,
  sequoia-sq,
}:

rustPlatform.buildRustPackage {

  pname = "secretcfg";
  version = "0.1.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ sequoia-sq ];

}
