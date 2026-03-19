{
  pkgs ? import <nixpkgs> { },
}:
let
  deps = import ./nix/deps.nix { inherit pkgs; };
  platform = if pkgs.stdenv.isLinux then "linux" else "macos";
in
pkgs.mkShell {
  inherit (deps) buildInputs;
  nativeBuildInputs = deps.nativeBuildInputs ++ [ pkgs.rustup ];

  strictDeps = true;

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath deps.buildInputs;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";

  shellHook = ''
    ROOT=$(git rev-parse --show-toplevel)
    export PATH="$PATH:$ROOT/target/dx/cowcord/debug/${platform}/app:$ROOT/target/dx/cowcord/release/${platform}/app"
  '';
}
