{
  description = "Rust flake template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
    malie = {
      url = "https://cdn.malie.io/file/malie-io/tcgl/export/index.json";
      flake = false;
    };
  };
  outputs = inputs@{ nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          dbg =
            value:
            builtins.trace (
              if value ? type && value.type == "derivation" then
                "derivation: ${value}"
              else
                pkgs.lib.generators.toPretty { } value
            ) value;

          crane = {
            lib = (inputs.crane.mkLib pkgs).overrideToolchain (_: self'.packages.rust-nightly);
          };

          packageVersion = builtins.elemAt (pkgs.lib.strings.splitString "+" (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version) 1;

          index = builtins.fromJSON (builtins.readFile inputs.malie.outPath);

          sources = pkgs.linkFarm "malie" (pkgs.lib.flatten (
            pkgs.lib.mapAttrsToList (
              lang: pkgs.lib.mapAttrsToList (
                setName: setInfo:
                let
                  split = pkgs.lib.strings.splitString "/" setInfo.path;
                  version = builtins.elemAt split 0;
                  fileName = builtins.elemAt split 1;
                in
                assert version == packageVersion;
                {
                  name = fileName;
                  path = derivation {
                    name = "${fileName}-${version}";
                    builder = "${pkgs.bash}/bin/bash";
                    PATH = "${pkgs.wget}/bin";
                    args = [ (builtins.toFile "builder.sh" ''
                      wget https://cdn.malie.io/file/malie-io/tcgl/export/${setInfo.path} -O $out
                    '') ];
                    inherit system;
                    outputHash = setInfo.hash;
                    outputHashAlgo = "md5";
                  };
                }
              )
            ) index
          ));
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [
              rust-overlay.overlays.default
            ];
          };

          packages = {
            inherit sources;
            rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            default = crane.lib.buildPackage {
              src = ./.;
              doCheck = false;
              cargoBuildCommand = "cargo build --release";
            };
          };
          checks = {
            default = crane.lib.cargoTest {
              strictDeps = true;
              src = ./.;
              cargoArtifacts = crane.lib.buildDepsOnly {
                strictDeps = true;
                src = ./.;
              };
              SOURCES_DIR="${sources}";
            };
          };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-nightly ]
                ++ (with pkgs; [
                jq
                moreutils
                nixd
                cargo-edit
              ]);
            };
          };
        };
    };
}
