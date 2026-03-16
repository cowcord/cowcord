{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell rec {
    strictDeps = true;

    nativeBuildInputs = with pkgs; [
      dioxus-cli
      tailwindcss_4
      # cargo

      cmake
      perl
      gcc
      # openssl
      pkg-config
      rustPlatform.bindgenHook # needed?
    ];

    buildInputs = with pkgs; [
      atkmm
      cairo
      gdk-pixbuf
      glib
      gtk3
      pango
      webkitgtk_4_1
      xdotool
    ];

    platform = if pkgs.stdenv.isLinux then "linux" else "macos";

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";

    shellHook = ''
      ROOT=$(git rev-parse --show-toplevel)
      export PATH="$PATH:$ROOT/target/dx/cowcord/debug/${platform}/app:$ROOT/target/dx/cowcord/release/${platform}/app"
    '';
  }

