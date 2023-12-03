{
  description = "A Python environment for Ouroboros";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-23.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            # Replace packages with specific (or modified) versions:
            # ```
            # (self: super: { python3 = super.python39; })
            # ```
          ];
        };
        pythonEnv = pkgs.python3.withPackages (ps: with ps; [
            # Add Python dependencies here:
            # ```
            # numpy
            # pandas
            # ```
        ]);
      in
      {
        devShell = pythonEnv;
      }
    );
}