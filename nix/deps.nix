{ pkgs }:
{
  nativeBuildInputs = with pkgs; [
    dioxus-cli
    tailwindcss_4

    cacert
    git
    cmake
    pkg-config
    rustPlatform.bindgenHook
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
}
