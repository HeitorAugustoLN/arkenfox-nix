{
  perSystem =
    { pkgs, self', ... }:
    {
      devShells.default = pkgs.mkShell {
        inputsFrom = [ self'.packages.arkenfox-nix ];

        packages = [
          pkgs.cargo
          pkgs.clippy
          pkgs.rustc
          self'.packages.arkenfox-nix
        ];
      };
    };
}
