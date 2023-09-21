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
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ self, nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
        
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          crane = rec {
            lib = self.inputs.crane.lib.${system};
            nightly = lib.overrideToolchain self'.packages.rust-nightly;
          };

          export_base_url = "https://cdn.malie.io/file/malie-io/tcgl/export";

          # index = pkgs.fetchurl {
          #   url = "${export_base_url}/index.json";
          #   hash = "";
          # };
        in {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = with inputs; [
              rust-overlay.overlays.default
            ];
          };
        
          packages = {
            rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
            default = crane.nightly.buildPackage {
              src = ./.;
              cargoBuildCommand = "cargo build --release";
            };

            fetch-sources = pkgs.writeShellApplication {
              name = "fetch-sources";
              runtimeInputs = [ pkgs.wget ];
              text = ''
                cd sources
                find . -type f -delete

                curl https://cdn.malie.io/file/malie-io/tcgl/export/index.json \
                | jq -r '. | to_entries | map(.value | to_entries | map(.value)) | flatten | .[]' \
                | while IFS= read -r line
                  do
                    wget "${export_base_url}/$line";
                  done
              '';
            };
          };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-nightly ]
                ++ (with pkgs; [
                  jq
                  moreutils
                  rnix-lsp
                ]);
            };
          };
        };
    };
}
