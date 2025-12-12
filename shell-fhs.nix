{
  pkgs ? import <nixpkgs> { },
}:

(pkgs.buildFHSEnv {
  name = "TauriFHS";
  targetPkgs = pkgs : (with pkgs; [
    # Rust packages
    cargo
    cargo-tauri
    rustc
    rustfmt
    rust-analyzer
    pkg-config

    # JS packages if using JS for UI
    nodejs
    pnpm # preferred JS package manager

    # Build dependencies
    glib.dev
    atk.dev
    gdk-pixbuf.dev
    libsoup_3.dev
    cairo.dev
    pango.dev
    webkitgtk_4_1.dev
    harfbuzz.dev
    glib.dev
    gtk3.dev
    openssl.dev
    librsvg.dev
    libappindicator-gtk3.dev
    stdenv.cc

    # Other required packages
    zlib
    xdg-utils
    gsettings-desktop-schemas
  ]);

  runScript = "bash";
}).env

