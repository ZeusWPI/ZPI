{
  description = "devshell zpi";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { self, nixpkgs }:

    let

      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
    in
    {
      devShells.${system}.default =
        pkgs.mkShell
        {
          buildInputs = with pkgs; [
            imagemagick
            nodejs_24
          ];

          shellHook = ''
            export MAGICK_PATH=${pkgs.imagemagick}/bin/magick
            echo  "devShell activated"
          '';
        };

  };
}