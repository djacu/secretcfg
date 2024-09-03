{ pkgs, lib }:
let
  inherit (pkgs) newScope;
  inherit (lib) makeScope;
in
makeScope newScope (
  self:
  let
    packages = lib.filterAttrs (_: value: value == "directory") (builtins.readDir ./.);
  in
  builtins.mapAttrs (name: _: self.callPackage (./. + "/${name}") { }) packages
)
