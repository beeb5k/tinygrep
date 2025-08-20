{
  description = "Development shell for Rust!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        fenixLib = fenix.packages.${system};
        rustToolChain = fenixLib.stable.toolchain;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolChain ];
        };
      }
    );
}
