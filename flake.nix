{
  description = "Telegram bot for Xinux community";

  inputs = {
    # Too old to work with most libraries
    # nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";

    # Perfect!
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    # The flake-utils library
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs
    , flake-utils
    , ...
    } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};

      binary = pkgs.callPackage ./. { };

      docker = pkgs.dockerTools.streamLayeredImage {
        name = "xinuxmgr";
        tag = "latest";
        contents = [ binary ];
        config = {
          Cmd = [ "${binary}/bin/xinuxmgr" ];
        };
      };
    in
    {
      # Nix script formatter
      formatter = pkgs.nixpkgs-fmt;

      # Development environment
      devShells.default = import ./shell.nix { inherit pkgs; };

      # Output package
      packages = {
        inherit docker;
        default = binary;
      };
    });
}
