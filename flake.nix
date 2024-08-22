{
  description =
    "A flake providing a reproducible environment for localic-utils";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in rec {
        packages.cosmwasm-check = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cosmwasm-checck";
          version = "v2.1.3";

          src = pkgs.fetchFromGitHub {
            owner = "CosmWasm";
            repo = "cosmwasm";
            rev = version;
            hash = "sha256-WXhz47cNeSRlUGfiXZkGOvu6WjK26MPJB716DiFqYPY=";
          };

          cargoHash = "sha256-2zTShh1tN6jC/PoVitcYwxAIKwel6uwJKRudC7LoBYQ=";

          meta = {
            description =
              "Utility for validating properties of cosmwasm artifacts";
            homepage = "https://github.com/CosmWasm/cosmwasm";
          };

          checkFlags = [
            "--skip=results::events::tests::attribute_new_reserved_key_panicks"
            "--skip=results::events::tests::attribute_new_reserved_key_panicks2"
          ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs.buildPackages;
            [ rust-bin.stable.latest.default ];

          buildInputs = with pkgs; [
            openssl
            libiconv
            pkg-config
            packages.cosmwasm-check
          ];
        };
      });
}
