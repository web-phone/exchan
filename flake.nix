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
        rust-nightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "llvm-tools-preview" "rust-src" ];
        });
        src = ./.;

        # Rust Stable version
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
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

        # Rust Nightly version
        craneLib-nightly = (crane.mkLib pkgs).overrideToolchain rust-nightly;
        cargoArtifacts-nightly = craneLib-nightly.buildDepsOnly {
          inherit src;
        };
        exchan-nightly = craneLib-nightly.buildPackage {
          inherit src cargoArtifacts-nightly;
          strictDeps = true;

          doCheck = true;
        };
        cargo-clippy-nightly = craneLib-nightly.cargoClippy {
          inherit cargoArtifacts-nightly src;
          cargoClippyExtraArgs = "--verbose -- --deny warnings";
        };
        cargo-doc-nightly = craneLib-nightly.cargoDoc {
          inherit cargoArtifacts-nightly src;
        };
        llvm-cov-nightly = craneLib-nightly.cargoLlvmCov {
          inherit cargoArtifacts-nightly src;
          cargoExtraArgs = "--locked";
          cargoLlvmCovCommand = "test";
          cargoLlvmCovExtraArgs = "--html --output-dir $out";
        };
        llvm-cov-text-nightly = craneLib-nightly.cargoLlvmCov {
          inherit cargoArtifacts-nightly src;
          cargoExtraArgs = "--locked";
          cargoLlvmCovCommand = "test";
          cargoLlvmCovExtraArgs = "--text --output-dir $out";
        };
      in
      {
        formatter = treefmtEval.config.build.wrapper;

        packages = {
          stable = {
            default = exchan;
            doc = cargo-doc;
            llvm-cov = llvm-cov;
            llvm-cov-text = llvm-cov-text;
          };

          runDB = pkgs.writeShellScriptBin "db-runner.sh" ''
            ${pkgs.surrealdb."1.4.2"}/bin/surreal start memory -A --auth --user test-db --pass test-db
          '';

          nightly = {
            default = exchan-nightly;
            doc = cargo-doc-nightly;
            llvm-cov = llvm-cov-nightly;
            llvm-cov-text = llvm-cov-text-nightly;
          };
        };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
        apps.runDB = flake-utils.lib.mkApp {
          drv = self.packages.${system}.runDB;
        };

        checks = {
          inherit exchan cargo-clippy cargo-doc cargo-clippy-nightly cargo-doc-nightly llvm-cov llvm-cov-text llvm-cov-nightly llvm-cov-text-nightly;
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
