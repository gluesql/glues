{
  description = "Glues - terminal note-taking toolkit (TUI, core, server)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Modern Rust build framework with split dependency / workspace caching.
    crane.url = "github:ipetkov/crane";

    # Unified formatter integration (runs on Nix files only here - Rust and
    # TOML formatting is left to `cargo fmt` / `taplo` from the dev shell
    # so the flake never rewrites upstream-owned source).
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      # Flake-wide outputs. Exposing an overlay lets downstream flakes pull
      # glues in via `overlays = [ glues.overlays.default ]`.
      flake = {
        overlays.default = _final: prev: {
          glues = self.packages.${prev.stdenv.hostPlatform.system}.glues;
        };
      };

      perSystem = { config, system, lib, ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
          };

          # Respect the project's rust-toolchain.toml (channel 1.93, plus
          # rustfmt / clippy / llvm-tools-preview). A single source of
          # truth for the Rust version used by package, checks and shell.
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

          workspaceCargoToml = lib.importTOML ./Cargo.toml;
          binCargoToml = lib.importTOML ./bin/glues/Cargo.toml;

          # Minimal source: only inputs cargo actually needs to build the
          # workspace. Editing README.md, docs, CI, AGENTS.md or the flake
          # itself no longer busts the build cache.
          src = lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              ./bin
              ./core
              ./server
              ./tui
            ];
          };

          nativeBuildInputs = [ pkgs.pkg-config ];

          # openssl-sys is pulled in transitively via reqwest's blocking
          # feature. Darwin additionally links several Apple SDK frameworks
          # (native-tls, arboard, tokio signals).
          buildInputs = with pkgs;
            [ openssl zlib ]
            ++ lib.optionals stdenv.hostPlatform.isDarwin (
              [ libiconv ]
              ++ (with pkgs.darwin.apple_sdk.frameworks; [
                AppKit
                Cocoa
                CoreFoundation
                Security
                SystemConfiguration
              ])
            );

          # Shared args for every crane invocation. All derivations - the
          # package, clippy, nextest, audit - share one dependency build
          # via `cargoArtifacts`.
          commonArgs = {
            inherit src nativeBuildInputs buildInputs;
            pname = "glues-workspace";
            version = workspaceCargoToml.workspace.package.version;

            strictDeps = true;

            # Prefer the system OpenSSL over the vendored copy.
            OPENSSL_NO_VENDOR = "1";
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          glues = craneLib.buildPackage (commonArgs // {
            inherit cargoArtifacts;

            pname = "glues";
            # Build every workspace binary (glues, glues-tui, glues-server).
            cargoExtraArgs = "--workspace --bins";

            # Tests run in a dedicated nextest check below; `nix build`
            # stays focused on producing the binaries.
            doCheck = false;

            meta = {
              description = lib.strings.removeSuffix "." binCargoToml.package.description;
              homepage = workspaceCargoToml.workspace.package.repository;
              changelog = "${workspaceCargoToml.workspace.package.repository}/releases";
              license = lib.licenses.asl20;
              mainProgram = "glues";
              platforms = lib.platforms.unix;
            };
          });
        in
        {
          # Share the overlay-augmented pkgs with sibling flake-parts
          # modules (treefmt-nix) so there is exactly one nixpkgs instance.
          _module.args.pkgs = pkgs;

          packages = {
            default = glues;
            glues = glues;
          };

          apps.default = {
            type = "app";
            program = lib.getExe glues;
          };

          devShells.default = pkgs.mkShell {
            inherit nativeBuildInputs buildInputs;

            packages = with pkgs; [
              rustToolchain
              # rust-toolchain.toml omits these, so expose them here for
              # editor integrations and coverage workflow from AGENTS.md.
              rust-analyzer
              cargo-llvm-cov
              # Cargo ergonomics
              cargo-edit
              cargo-watch
              cargo-outdated
              cargo-audit
              cargo-deny
              cargo-nextest
              # Nix ergonomics
              nil
              config.treefmt.build.wrapper
            ];

            env = {
              OPENSSL_NO_VENDOR = "1";
              RUST_BACKTRACE = "1";
              RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            };

            shellHook = ''
              echo "glues dev shell - $(rustc --version)"
            '';
          };

          # `nix flake check` runs the full matrix: workspace build, strict
          # clippy across all targets, and the full test suite via nextest.
          # cargo-audit / cargo-deny are left as dev-shell tools because
          # the upstream dependency tree currently surfaces RUSTSEC
          # advisories the flake PR shouldn't gate on - run them manually
          # from the dev shell.
          checks = {
            inherit glues;

            clippy = craneLib.cargoClippy (commonArgs // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--workspace --all-targets -- -D warnings";
            });

            nextest = craneLib.cargoNextest (commonArgs // {
              inherit cargoArtifacts;
              cargoNextestExtraArgs = "--workspace --no-fail-fast";
              partitions = 1;
              partitionType = "count";
            });
          };

          # `nix fmt` formats the Nix files in this repo. Rust and TOML
          # formatting is deliberately not wired in here because it would
          # rewrite upstream-owned source outside this flake's scope -
          # run `cargo fmt` / `taplo fmt` from the dev shell if desired.
          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixpkgs-fmt.enable = true;
          };
        };
    };
}
