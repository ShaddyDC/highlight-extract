{
  description = "A tool to extract highlights and notes taken on an Onyx Boox e-reader for export as Markdown";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nci = {
      url = "github:yusdacra/nix-cargo-integration";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
    parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    parts,
    nci,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      imports = [nci.flakeModule];
      systems = [
        "x86_64-linux"

        # untested
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
      ];

      perSystem = {
        config,
        pkgs,
        lib,
        ...
      }: {
        nci.projects."highlight-extract".path = ./.;
        nci.crates."highlight-extract" = {};

        packages.highlight-extract = config.nci.outputs."highlight-extract".packages.release;
        packages.highlight-extract-dev = config.nci.outputs."highlight-extract".packages.dev;
        packages.default = config.packages.highlight-extract;

        devShells.default = config.nci.outputs."highlight-extract".devShell.overrideAttrs (old: {
          shellHook = ''
            export RUST_BACKTRACE="1"
          '';

          packages = (old.packages or []) ++ (with pkgs; [
            rust-analyzer
          ]);
        });
      };
    };
}
