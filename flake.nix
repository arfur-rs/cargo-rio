{
  inputs = {
    nci.url = "github:yusdacra/nix-cargo-integration";
  };
  outputs = inputs: inputs.nci.lib.makeOutputs {
    root = ./.;

    overrides.shell = common: prev: {
      packages =
        prev.packages
        ++ (with common.pkgs; [
          pkg-config
          openssl.dev
          rust-analyzer
          cargo-release
          git-cliff
        ]);
      };
  };
}
