{
  description = "ethos-zero — the ethos schema language, version zero";

  inputs = {
    base.url = "github:LiGoldragon/build-base";
    # The dependencies' own ethos declarations, read by the built tool as a check.
    protos = {
      url = "github:LiGoldragon/protos";
      flake = false;
    };
    datom-codec = {
      url = "github:LiGoldragon/datom-codec";
      flake = false;
    };
  };

  # Cargo.nix is generated from Cargo.lock by the build base's crate2nix and
  # committed, so evaluation imports no derivation:
  #   nix run github:LiGoldragon/build-base#crate2nix -- generate
  outputs =
    {
      self,
      base,
      protos,
      datom-codec,
    }:
    base.lib.workspaceOutputs {
      cargoNix = ./Cargo.nix;
      extraChecks =
        {
          pkgs,
          toolchain,
          workspace,
          binaries,
          buildRustCrateForPkgsFor,
          ...
        }:
        let
          lib = pkgs.lib;
          manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          src = lib.cleanSourceWith {
            src = lib.cleanSource ./.;
            filter = path: type: lib.cleanSourceFilter path type;
            name = "source";
          };
          # The member's library, whose dependencies the generated crates
          # link, and its binaries, which generate them.
          library = workspace.workspaceMembers."ethos-zero".build;
          package = binaries "ethos-zero" [ "default" ];
          # The base's crate builder, under the feature-set name the
          # package's own build takes.
          buildRustCrateForPkgs = buildRustCrateForPkgsFor {
            packageId = workspace.workspaceMembers."ethos-zero".packageId;
            features = [ "default" ];
            runTests = false;
          };
          # These limits run inside each derivation, so test children inherit them.
          resourcePolicy = ''
            ulimit -v 8388608
          '';

          # A crate made of what the built tool generates from the fixtures,
          # built as its own derivation by the base's buildRustCrate. Its
          # dependencies are the very crate derivations ethos-zero links.
          fixtures = lib.fileset.toSource {
            root = ./fixtures;
            fileset = ./fixtures;
          };
          linked =
            name:
            lib.findFirst (dependency: dependency.crateName == name)
              (throw "ethos-zero links no ${name}")
              library.dependencies;
          rkyv = linked "rkyv";
          datomCodec = linked "datom-codec";
          generatedCrate =
            {
              name,
              declarations,
              libPath ? "src/lib.rs",
              root ? "",
              features ? [ ],
              dependencies,
              buildTests ? false,
            }:
            (buildRustCrateForPkgs pkgs) {
              crateName = name;
              version = "0.1.0";
              edition = manifest.package.edition;
              src = pkgs.runCommand "${name}-source" {
                inherit package fixtures root declarations;
              } (builtins.readFile ./checks/generated-source.sh);
              inherit
                libPath
                features
                dependencies
                buildTests
                ;
            };
          # Runs the test binaries of a generated crate's test build.
          runTests =
            name: build:
            pkgs.runCommand name { } ''
              ${resourcePolicy}
              for binary in ${build}/tests/*; do
                echo "running $(basename "$binary")"
                "$binary"
              done
              touch "$out"
            '';
          # A generated Signal contract built without its datom feature, and
          # with no datom-codec anywhere in its dependency graph.
          withoutDatom =
            name: declaration:
            let
              crate = generatedCrate {
                inherit name;
                declarations = [ declaration ];
                libPath = "generated/${lib.removeSuffix ".ethos" (baseNameOf declaration)}.rs";
                dependencies = [ rkyv ];
              };
              graph = map (dependency: dependency.crateName) crate.completeDeps;
            in
            pkgs.runCommand name { inherit crate; graph = toString graph; } ''
              echo "$crate depends on: $graph"
              for dependency in $graph; do
                if [ "$dependency" = datom-codec ]; then
                  echo "$crate has datom-codec in its dependency graph" >&2
                  exit 1
                fi
              done
              touch "$out"
            '';
          # The Flow Nexus contract under a feature set, its test build.
          flowContract =
            features: dependencies:
            generatedCrate {
              name = "flow-contract";
              declarations = map (stem: "print/flow-${stem}.ethos") [
                "library"
                "signal"
                "operation"
                "memory"
              ];
              root = ./tests/flow-contract/lib.rs;
              inherit features dependencies;
              buildTests = true;
            };
          # Producer outputs are libraries reused by the consumer and rejection proof.
          namespaceProducer =
            features: dependencies:
            generatedCrate {
              name = "signal";
              declarations = [ "print/flow-library.ethos" "namespace-ordinary.ethos" "namespace-recursive.ethos" ];
              root = ./tests/namespace-producer/lib.rs;
              inherit features dependencies;
            };
          # A real dependency named signal is linked into a separate consumer.
          namespaceContract =
            features: dependencies:
            generatedCrate {
              name = "namespace-contract";
              declarations = [ "namespace-owner.ethos" "namespace-emission.ethos" ];
              root = ./tests/namespace-contract/lib.rs;
              inherit features;
              dependencies = dependencies ++ [ (namespaceProducer features dependencies) ];
              buildTests = true;
            };
          # A rejected compilation is its own build output; the check only reads it.
          missingImport =
            let
              producer = namespaceProducer [ ] [ rkyv ];
              source = pkgs.runCommand "namespace-missing-import-source" {
                inherit package fixtures;
                root = "";
                declarations = [ "invalid-import/missing.ethos" ];
              } (builtins.readFile ./checks/generated-source.sh);
            in
            pkgs.runCommand "namespace-missing-import-compile" {
              nativeBuildInputs = [ toolchain ];
            } ''
              ${resourcePolicy}
              mkdir "$out"
              ${package}/bin/ethos-zero 'Check.${fixtures}/invalid-import/missing.ethos' > "$out/check.datom"
              grep -x Checked "$out/check.datom"
              flags=()
              for dependency in ${toString (map lib.getLib producer.completeDeps)}; do
                flags+=(-L "dependency=$dependency/lib")
              done
              signal_library=(${lib.getLib producer}/lib/libsignal-*.rlib)
              rkyv_library=(${lib.getLib rkyv}/lib/librkyv-*.rlib)
              if rustc --crate-name missing_import --crate-type lib --edition ${manifest.package.edition} \
                --emit=metadata --error-format=json \
                "''${flags[@]}" --extern "signal=''${signal_library[0]}" --extern "rkyv=''${rkyv_library[0]}" \
                ${source}/generated/missing.rs --out-dir "$TMPDIR" > "$out/stdout" 2> "$out/stderr"; then
                echo 'An absent external export compiled' >&2
                exit 1
              fi
              grep -F '"code":"E0412"' "$out/stderr"
              grep -F 'cannot find type `Absent` in module' "$out/stderr"
            '';
        in
        {
          namespace-missing-import = pkgs.runCommand "namespace-missing-import" { } ''
            test -s ${missingImport}/stderr
            cp ${missingImport}/stderr "$out"
          '';
          namespace-contract = runTests "namespace-contract" (namespaceContract [ ] [ rkyv ]);
          namespace-contract-datom = runTests "namespace-contract-datom" (
            namespaceContract [ "datom" ] [ rkyv datomCodec ]
          );
          signal-without-datom = withoutDatom "signal-without-datom" "orchestrate.ethos";
          operation-free-signal-without-datom = withoutDatom "operation-free-signal-without-datom" "empty-signal.ethos";
          flow-contract = runTests "flow-contract" (flowContract [ ] [ rkyv ]);
          flow-contract-datom = runTests "flow-contract-datom" (
            flowContract [ "datom" ] [
              rkyv
              datomCodec
            ]
          );
          fmt = pkgs.runCommand "ethos-zero-fmt" { nativeBuildInputs = [ toolchain ]; inherit src; } ''
            ${resourcePolicy}
            cd "$src"
            find src tests -name '*.rs' -print0 | xargs -0 rustfmt --check --edition 2024
            touch "$out"
          '';
          dependency-ethos = pkgs.runCommand "ethos-zero-dependency-ethos"
            {
              inherit package;
              declarations = [
                "${protos}/protos.ethos"
                "${protos}/protos-kinds.ethos"
                "${datom-codec}/datom-codec.ethos"
                "${datom-codec}/datom-codec-kinds.ethos"
              ];
            }
            ''
              ${resourcePolicy}
              ${builtins.readFile ./checks/dependency-ethos.sh}
            '';
          no-free-functions = pkgs.runCommand "ethos-zero-no-free-functions" { inherit src; } ''
            ${resourcePolicy}
            ${builtins.readFile ./checks/no-free-functions.sh}
          '';
          no-inherent-methods = pkgs.runCommand "ethos-zero-no-inherent-methods" { inherit src; } ''
            ${resourcePolicy}
            ${builtins.readFile ./checks/no-inherent-methods.sh}
          '';
        };
    };
}
