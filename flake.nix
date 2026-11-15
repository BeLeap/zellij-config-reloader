{
  description = "Zellij config reloader WASM plugin";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = {nixpkgs, ...}: let
    systems = [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
  in {
    packages = nixpkgs.lib.genAttrs systems (system: {
      default = import ./package.nix {
        pkgs = import nixpkgs {inherit system;};
      };
    });
  };
}
