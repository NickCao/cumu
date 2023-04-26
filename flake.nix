{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
      with pkgs;{
        devShells.default = mkShell {
          buildInputs = [
            cargo
            rustc
            rustPlatform.bindgenHook
            cudaPackages_12.cuda_cudart
            cudaPackages_12.cuda_nvcc
          ];
        };
      });
}
