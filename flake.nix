{
  description = "Secret Configuration Management Tool";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        inherit (pkgs) lib;
      in
      {
        packages = builtins.removeAttrs (pkgs.callPackage ./packages { }) [
          "callPackage"
          "newScope"
          "override"
          "overrideDerivation"
          "overrideScope"
          "overrideScope'"
          "packages"
        ];
      }
    );
}
