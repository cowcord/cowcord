{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
    strictDeps = true;

    nativeBuildInputs = with pkgs; [
      dioxus-cli

      gcc
      openssl
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

    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  }

