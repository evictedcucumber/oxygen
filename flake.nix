{
  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  outputs = {
    nixpkgs,
    rust-overlay,
    ...
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;

      overlays = [(import rust-overlay)];
    };
  in {
    devShells.${system}.default = pkgs.mkShell {
      name = "o2c";
      packages = with pkgs; [
        openssl
        grcov
        lcov
        llvm
        valgrind
        (rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "llvm-tools-preview"];
        })
      ];
    };
  };
}
