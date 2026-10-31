{
  description = "UBC Hackerfab Firmware Flake";

  inputs = {
    crane.url = "github:ipetkov/crane";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.zst";
    # For embedded targets
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.treefmt-nix.flakeModule ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      perSystem =
        { pkgs, system, ... }:
        let
          # Pin tooling with rust-toolchain.toml
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          # Include overlay
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          # Development packages go here
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.picotool
              pkgs.probe-rs-tools
              toolchain
            ];
          };

          # This is so nix fmt and cargo fmt stay in sync
          treefmt = {
            programs.nixfmt.enable = true;
            programs.rustfmt = {
              enable = true;
              package = toolchain;
            };
          };

          # Any release packages would go here
          packages.default = pkgs.hello;
        };
    };
}
