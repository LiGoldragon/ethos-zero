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
          ...
        }:
        let
          lib = pkgs.lib;
          src = lib.cleanSourceWith {
            src = lib.cleanSource ./.;
            filter = path: type: lib.cleanSourceFilter path type;
            name = "source";
          };
          package = workspace.workspaceMembers."ethos-zero".build;
          # These limits run inside each derivation, so test children inherit them.
          resourcePolicy = ''
            ulimit -v 8388608
          '';
        in
        {
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
