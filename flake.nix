{
  description = "Tectonic, a modernized TeX/LaTeX engine";

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      supportedSystems = [
        "aarch64-linux"
        "aarch64-darwin"
        "i686-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      foreachSystem = f: lib.genAttrs supportedSystems (system: f {
        pkgs = nixpkgs.legacyPackages.${system};
        /** final packages set (of a given system) provided in this flake */
        final = self.packages.${system};
      });
      addCargoFeatures = x: lib.unique (x ++ [
        /** currently nix flake does not work well with git submodules, so we
            ensure that the nixpkgs provided harfbuzz is used instead. */
        "external-harfbuzz"
      ]);
    in
    {
      packages = foreachSystem ({ pkgs, final }: {

        /** package definition from nixpkgs, with local overrides */
        tectonic-unwrapped = pkgs.tectonic-unwrapped.overrideAttrs (
          { meta, cargoBuildFeatures, cargoCheckFeatures, ... }: {
            name = "tectonic";
            src = ./.;
            cargoDeps = pkgs.rustPlatform.importCargoLock {
              lockFile = ./Cargo.lock;
            };
            cargoBuildFeatures = addCargoFeatures cargoBuildFeatures;
            cargoCheckFeatures = addCargoFeatures cargoCheckFeatures;

            /*
              For `flake.nix` contributors: put additional overrides here.
              If the changes are also applicable to the `tectonic` package
              in nixpkgs, consider first improving the definition there,
              and then update the `flake.lock` here.
            */

            meta = {
              # to correctly generate meta.position for back trace:
              inherit (meta) description;

              # maintainers for the local overrides:
              maintainers = with lib.maintainers; [ bryango ];
            };
          }
        );

        /** a version of biber that works with the current tectonic bundle */
        inherit (pkgs) biber-for-tectonic;

        /** tectonic wrapped with the correct version of biber; this provides
            a partial fix for issue #893. */
        tectonic = pkgs.tectonic.override {
          inherit (final)
            tectonic-unwrapped
            biber-for-tectonic;
        };

        /** the default package to build & install */
        default = final.tectonic;
      });

      devShells = foreachSystem ({ pkgs, final }: {
        default = final.tectonic-unwrapped.overrideAttrs ({ nativeBuildInputs, ... }: {

          nativeBuildInputs = with pkgs.buildPackages; [
            cargo # with shell completions, instead of cargo-auditable
            mold-wrapped # fast linker for development
          ] ++ nativeBuildInputs;

          env = {
            # for developments, e.g. symbol lookup in std library
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

            # use mold, a fast linker
            CARGO_BUILD_RUSTFLAGS = ''
              -C link-arg=-fuse-ld=mold
            '';
          };
        });
      });

      checks = foreachSystem ({ pkgs, final }:
        let
          tectonic-unwrapped =
            final.tectonic-unwrapped.overrideAttrs (prevAttrs: {
              preCheck = prevAttrs.preCheck or "" + ''
                export RUST_BACKTRACE=1
              '';
            });
        in
        {
          inherit tectonic-unwrapped;
          tectonic = final.tectonic.override {
            inherit tectonic-unwrapped;
          };
        });
    };
}
