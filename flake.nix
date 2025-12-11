{
  description = "Development shell for Rust!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        naerskLib = pkgs.callPackage naersk { };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = cargoToml.package.version;
        myPackage = naerskLib.buildPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;
          edition = cargoToml.package.edition;
          src = ./.;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [ 
            cargo
            rustfmt
            rustc
            rust-analyzer
            clippy
            lld
          ];
        };

        packages.default = myPackage;
      }
    );
}
