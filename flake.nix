{
  description = "Dev flake for liblo-sys FFI crate";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = inputs.flake-utils.lib.defaultSystems;

      perSystem =
        {
          self',
          pkgs,
          system,
          lib,
          ...
        }:
        let
          fnx = inputs.fenix.packages.${system};

          mkRustDeriv =
            fnx-version: extra-components:
            let
              std-components = [
                fnx-version.cargo
                fnx-version.clippy
                fnx-version.rust-src
                fnx-version.rustc
                fnx-version.rust-analyzer

                # it's generally recommended to use nightly rustfmt
                fnx.complete.rustfmt
              ];
              all-components = std-components ++ extra-components;
            in
            fnx.combine all-components;

          stableRust = mkRustDeriv fnx.stable [ ];
          nightlyRust = mkRustDeriv fnx.complete [ ];
          wasmRust = mkRustDeriv fnx.stable [ fnx.targets.wasm32-unknown-unknown.stable.rust-std ];

          generalPkgs = with pkgs; [
            pkg-config
            alsa-lib
            openssl
            rustPlatform.bindgenHook

            # used for non-bundled build
            liblo
            
            # used for bundled build
            autoconf
            automake
            autogen
            libtool
          ];
          nightlyPkgs = [ ];
          wasmPkgs = [ ];

          mkName = name: name + "-dev-shell";
          mkPersonalShell =
            { shellName, shellPackages }:
            pkgs.mkShell rec {
              name = mkName shellName;
              packages = generalPkgs ++ shellPackages;
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
            };
        in
        {

          formatter = pkgs.nixfmt-rfc-style;
          devShells.default = self'.devShells.rust-stable;
          devShells = {
            rust-stable = mkPersonalShell {
              shellName = "rust-stable";
              shellPackages = [ stableRust ];
            };
            rust-nightly = mkPersonalShell {
              shellName = "rust-nightly";
              shellPackages = nightlyPkgs ++ [ nightlyRust ];
            };
            rust-wasm = mkPersonalShell {
              shellName = "rust-wasm";
              shellPackages = wasmPkgs ++ [ wasmRust ];
            };
          };
        };
    };
}
