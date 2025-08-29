{
  description = "Development shell for Rust!";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      naersk,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        fenixLib = fenix.packages.${system};
        rustToolChain = fenixLib.stable.toolchain;
        naerskLib = pkgs.callPackage naersk { };
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = cargoToml.package.version;
        myPackage = naerskLib.buildPackage {
          pname = "tgrep";
          version = version;
          edition = "2024";
          src = ./.;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolChain ];
        };

        packages.default = myPackage;
      }
    );
}
