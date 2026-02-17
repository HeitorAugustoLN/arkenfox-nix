{
  flake.modules.homeManager.arkenfox =
    { config, lib, ... }:
    let
      cfg = config.programs.firefox.arkenfox;
    in
    {
      options.programs.firefox.arkenfox = {
        enable = lib.mkEnableOption "arkenfox support in profiles";

        version = lib.mkOption {
          type = lib.types.enum (builtins.attrNames (import ../../data));
          default = "master";
          example = "131.0";
          description = "The version of arkenfox user.js used.";
        };

        profiles = lib.mkOption {
          type = lib.types.attrsOf (
            lib.types.submodule (
              { config, ... }:
              let
                data = (import ../../data).${cfg.version};
              in
              {
                config.flatSettings = lib.optionalAttrs config.enable (
                  builtins.foldl' (x: y: lib.recursiveUpdate x y) { } (
                    lib.mapAttrsToList (name: _: config.settings.${name}.flatSettings) data
                  )
                );

                imports = lib.mapAttrsToList (
                  name: _:
                  { config, ... }:
                  {
                    settings.${name}.enable = lib.mkDefault config.enableAllSections;
                  }
                ) data;

                options = {
                  enable = lib.mkEnableOption "arkenfox for this profile" // {
                    default = builtins.any (x: x.enable) (builtins.attrValues config.settings);
                    defaultText = "`true` when `settings` has any section enabled.";
                  };

                  enableAllSections = lib.mkEnableOption "all sections by default";

                  flatSettings = lib.mkOption {
                    type = lib.types.attrsOf lib.types.anything;
                    description = "All preferences.";
                    readOnly = true;
                  };

                  settings =
                    let
                      sectionOption = import ./_lib/section-option.nix { inherit lib; };
                    in
                    builtins.mapAttrs sectionOption data;
                };
              }
            )
          );

          default = { };
          description = "Configurations for each profile.";
        };
      };

      config = lib.mkIf cfg.enable {
        programs.firefox.profiles = builtins.mapAttrs (_: profile: {
          settings = profile.flatSettings;
        }) cfg.profiles;
      };
    };
}
