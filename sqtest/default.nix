{
  mkShell,
  sequoia-sq,
  openpgp-card-tools,
}:
mkShell {
  packages = [
    sequoia-sq
    openpgp-card-tools
  ];
}
