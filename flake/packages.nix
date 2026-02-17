{
  perSystem =
    { pkgs, self', ... }:
    {
      packages = {
        default = self'.packages.arkenfox-nix;
        arkenfox-nix = pkgs.callPackage ../arkenfox-nix { };
      };
    };
}
