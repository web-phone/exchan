{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    rust-overlay.url = "github:oxalica/rust-overlay";
    surrealdb-overlay.url = "github:haruki7049/surrealdb-overlay";
    surrealist-overlay.url = "github:haruki7049/surrealist-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, treefmt-nix, rust-overlay, surrealdb-overlay, surrealist-overlay, flake-utils, crane }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) (import surrealdb-overlay) (import surrealist-overlay) ];
        pkgs = import nixpkgs {
          inherit overlays system;
          config.allowUnfree = true;
        };
        treefmtEval = treefmt-nix.lib.evalModule pkgs ./treefmt.nix;
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
        src = ./.;
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
        };
        exchan = craneLib.buildPackage {
          inherit src cargoArtifacts;
          strictDeps = true;

          doCheck = true;
        };
        cargo-clippy = craneLib.cargoClippy {
          inherit cargoArtifacts src;
          cargoClippyExtraArgs = "--verbose -- --deny warnings";
        };
        cargo-doc = craneLib.cargoDoc {
          inherit cargoArtifacts src;
        };
        llvm-cov = craneLib.cargoLlvmCov {
          inherit cargoArtifacts src;
          cargoExtraArgs = "--locked";
          cargoLlvmCovCommand = "test";
          cargoLlvmCovExtraArgs = "--html --output-dir $out";
        };
        llvm-cov-text = craneLib.cargoLlvmCov {
          inherit cargoArtifacts src;
          cargoExtraArgs = "--locked";
          cargoLlvmCovCommand = "test";
          cargoLlvmCovExtraArgs = "--text --output-dir $out";
        };
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        packages.default = exchan;
        packages.doc = cargo-doc;
        packages.runDB = pkgs.writeShellScriptBin "db-runner.sh" ''
          ${pkgs.surrealdb."1.4.2"}/bin/surreal start memory -A --auth --user test-db --pass test-db
        '';
        packages.llvm-cov = llvm-cov;
        packages.llvm-cov-text = llvm-cov-text;

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
        apps.runDB = flake-utils.lib.mkApp {
          drv = self.packages.${system}.runDB;
        };

        checks = {
          inherit exchan cargo-clippy cargo-doc llvm-cov llvm-cov-text;
          formatting = treefmtEval.config.build.check self;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rust
            pkgs.surrealdb."1.4.2"
            pkgs.surrealist."2.0.5"
            pkgs.nixd
          ];

          shellHook = ''
            export PS1="\n[nix-shell:\w]$ "
          '';
        };
      });
}
