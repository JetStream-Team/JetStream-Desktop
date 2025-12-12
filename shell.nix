{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "JetS";

  nativeBuildInputs = with pkgs; [
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

    # Other required packages
    zlib
    xdg-utils
    gsettings-desktop-schemas
  ];

  buildInputs = with pkgs; [
    # Build dependencies
    glib
    atk
    gdk-pixbuf
    libsoup_3
    cairo
    pango
    webkitgtk_4_1
    harfbuzz
    glib
    gtk3
    openssl
    librsvg
    libappindicator-gtk3
    stdenv.cc
  ];
}

