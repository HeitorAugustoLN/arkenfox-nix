{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem.treefmt = {
    programs = {
      nixf-diagnose = {
        enable = true;
        priority = -1;
      };

      nixfmt.enable = true;

      prettier = {
        enable = true;
        includes = [ "*.md" ];
      };

      rustfmt.enable = true;
    };

    settings.on-unmatched = "info";
  };
}
