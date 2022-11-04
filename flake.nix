{
  description = "Examples from the Rust book";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ...}:
  let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in with pkgs;
  {
    devShell.${system} = pkgs.mkShell {
      buildInputs = [
        cargo
      ];
    }; 
  };
}
