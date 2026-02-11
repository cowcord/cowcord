{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell rec {
    strictDeps = true;

    nativeBuildInputs = with pkgs; [
      dioxus-cli
      tailwindcss_4

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

    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  }

