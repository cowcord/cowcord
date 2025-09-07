{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        with pkgs;
        {
          devShells.default = mkShell {
            packages = [
              # required tools
              dioxus-cli
              just

              # required for rust-analyzer
              openssl
              pkg-config

              # required for dioxus
              gtk3
              libayatana-appindicator
              webkitgtk_4_1
              glib
              pango
              gdk-pixbuf
              cairo
              atkmm
              libsoup_3
              pkg-config
            ];
          };
        }
      );
}
