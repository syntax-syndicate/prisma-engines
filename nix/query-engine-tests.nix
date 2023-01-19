{ craneLib, pkgs, flakeInputs, self', ... }:

{
  packages.query-engine-tests = pkgs.stdenv.mkDerivation {
    name = "query-engine-tests";

    inherit (self'.packages.prisma-engines) src buildInputs nativeBuildInputs;

    buildPhase = ''
      mkdir .cargo
      ln -s ${self'.packages.prisma-engines-deps}/config.toml .cargo/config.toml
      cargo build --test query_engine_tests
    '';

    installPhase = ''
      set -euxo pipefail
      mkdir -p target/bin
      BIN=$(find target/debug/deps -name 'query_engine_tests*' | grep -v '.d$' | tr -d "\n")
      cp $BIN target/bin/query-engine-tests
    '';
  };
}

