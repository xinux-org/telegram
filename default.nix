# For more, refer to:
# https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md
{ pkgs ? import <nixpkgs> { } }:
let
  lib = pkgs.lib;
  getLibFolder = pkg: "${pkg}/lib";
  getFramwork = pkg: "${pkg}/Library/Frameworks";
  darwinOptions =
    if pkgs.stdenv.isDarwin then ''
      -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.Security)}
      -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.CoreFoundation)}
      -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.CoreServices)}
      -F${(getFramwork pkgs.darwin.apple_sdk.frameworks.SystemConfiguration)}
    '' else "";
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  binary = (pkgs.lib.importTOML ./bots/xinuxmgr/Cargo.toml).package;
in
pkgs.rustPlatform.buildRustPackage {
  pname = "xinuxmgr";
  version = binary.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  nativeBuildInputs = with pkgs; [
    gcc
    nixd
    rustc
    cargo
    cmake
    clippy
    gnumake
    libiconv
    cargo-watch
    pkg-config
    nixpkgs-fmt
    rust-analyzer
    llvmPackages.llvm
    llvmPackages.clang
  ];

  # Having hard times nix running from macOS 15 Beta?
  # add these to your buildInputs:
  # darwin.apple_sdk.frameworks.Security
  # darwin.apple_sdk.frameworks.CoreServices
  # darwin.apple_sdk.frameworks.CoreFoundation
  # darwin.apple_sdk.frameworks.SystemConfiguration
  buildInputs = with pkgs; [
    openssl
    libressl
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    (getLibFolder pkgs.gcc)
    (getLibFolder pkgs.libiconv)
    (getLibFolder pkgs.llvmPackages.llvm)
  ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  NIX_LDFLAGS = "-L${(getLibFolder pkgs.libiconv)} ${darwinOptions}";

  # If you wanna get thorny
  # RUST_BACKTRACE = 1;

  meta = with lib; {
    homepage = manifest.workspace.package.homepage;
    description = "Telegram bots for Xinux community";
    license = with lib.licenses; [ mit asl20 ];

    platforms = with platforms; linux ++ darwin;

    maintainers = [
      {
        name = "Sokhibjon Orzikulov";
        email = "sakhib@orzklv.uz";
        handle = "orzklv";
        github = "orzklv";
        githubId = 54666588;
        keys = [{
          fingerprint = "00D2 7BC6 8707 0683 FBB9  137C 3C35 D3AF 0DA1 D6A8";
        }];
      }
    ];
  };
}
