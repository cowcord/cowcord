{
  pkgs,
}:
let
  deps = import ./deps.nix { inherit pkgs; };
  platform = if pkgs.stdenv.isLinux then "linux" else "macos";
in
pkgs.rustPlatform.buildRustPackage {
  inherit (deps) buildInputs;
  nativeBuildInputs = deps.nativeBuildInputs ++ [ pkgs.wrapGAppsHook3 ];

  pname = "cowcord";
  version = (builtins.fromTOML (builtins.readFile ../crates/app/Cargo.toml)).package.version;
  src = ../.;

  cargoHash = "sha256-VeKXoCHPwtnCxds7PDIJmjTfun6qs/Od1NkaVgTtOxc=";
  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "const-serialize-0.8.0-alpha.0" = "sha256-d5RkjiRNn83d/OPUOGa2FOE/YP2LbqQ+MgPMTN00M8U=";
    };
  };

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";

  preBuild = ''
    export HOME=$(mktemp -d)

    export PACKAGE_MANAGER="Nix"
    tailwindcss -i ./crates/app/input.css -o ./crates/app/assets/tailwind.css
    dx build --release --platform ${platform}
  '';

  installPhase = ''
    mkdir -p $out/bin
    cp -r target/dx/cowcord/release/linux/app/. $out/bin/

    wrapProgram $out/bin/cowcord \
      --set GIO_MODULE_DIR "${pkgs.glib-networking}/lib/gio/modules/"
  '';

  checkPhase = ''
    dx check
  '';

  dontCargoBuild = true;
  dontCargoInstall = true;
}
