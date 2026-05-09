# Arkenfox-nix

This repository provides a Nix Home Manager module that integrates the [Arkenfox user.js](https://github.com/arkenfox/user.js) configurations into Firefox, enhancing privacy and security.

## Table of contents

- [Features](#features)
- [Getting started](#getting-started)
- [Comparison with arkenfox-nixos](#comparison-with-arkenfox-nixos)
- [CLI Tools](#cli-tools)
- [Acknowledgments](#acknowledgments)
- [License](#license)

## Features

- **Automatic Integration:** Seamlessly apply Arkenfox settings to your Firefox profiles using Nix.
- **Version Control:** Choose the Arkenfox version that suits your needs, including the latest master branch or specific releases (v91.0+).
- **Cross-platform:** Works on any system supported by Nix and Home Manager.
- **Granular Control:** Enable/disable specific sections, subsections, and individual preferences.

## Getting started

To begin using Arkenfox-nix, add the module to your Nix configuration and enable it for your preferred browser.

#### Example Configuration

Below is an example of how to integrate Arkenfox with Firefox using this module:

```nix
{ inputs, ... }:
{
  imports = [ inputs.arkenfox.modules.homeManager.arkenfox ]; # or inputs.arkenfox.homeModules.arkenfox

  programs.firefox = {
    enable = true;

    arkenfox = {
      enable = true;
      version = "140.0"; # Set version here, defaults to master branch

      profiles.example-profile = {
        # Set this to enable all sections by default
        enableAllSections = true;

        settings = {
          # To enable/disable specific sections
          "0100" = {
            enable = true;
          };

          # To enable/disable specific subsections
          "0300" = {
            enable = true;
            "0335".enable = false; # Disable Firefox Home telemetry
          };

          # To enable/disable specific preferences
          "1200" = {
            enable = true;
            "1201"."security.ssl.require_safe_negotiation".value = false;
          };
        };
      };
    };

    profiles.example-profile = {
      name = "Example";
    };
  };
}
```

## Comparison with arkenfox-nixos

Arkenfox-nix is based on [dwarfmaster/arkenfox-nixos](https://github.com/dwarfmaster/arkenfox-nixos) but changes the configuration approach for better organization and user experience.

### Configuration Architecture

The main improvement is moving from profile-level arkenfox configuration to a centralized approach:

**arkenfox-nixos (profile-level configuration):**

```nix
{
  programs.firefox = {
    enable = true;

    arkenfox = {
      enable = true;
      version = "140.0";
    };

    profiles.default = {
      name = "Default";
      arkenfox = {
        enable = true;
        "0000".enable = true;
        "0100" = {
          enable = true;
          "0101"."browser.shell.checkDefaultBrowser".value = true;
        };
        # arkenfox config mixed with profile definition
      };
    };
  };
}
```

**arkenfox-nix (centralized configuration):**

```nix
{
  programs.firefox = {
    enable = true;

    # Clean separation: arkenfox config is centralized
    arkenfox = {
      enable = true;
      version = "140.0";

      profiles.default = {
        enableAllSections = true;
        settings = {
          "0100"."0101"."browser.shell.checkDefaultBrowser".value = true;
        };
      };
    };

    # Profile definitions stay clean
    profiles.default = {
      name = "Default";
    };
  };
}
```

### Benefits of the New Approach

1. **Cleaner Separation:** Arkenfox configuration is separate from profile definitions
2. **Centralized Management:** All arkenfox settings in one place under `programs.firefox.arkenfox`
3. **Better Organization:** Profile definitions focus on profile-specific settings, arkenfox settings are centralized
4. **Easier Maintenance:** All arkenfox-related configuration is in one location

### Migration from arkenfox-nixos

Migration involves moving arkenfox configuration from individual profiles to the centralized location:

```nix
{
  # Before (arkenfox-nixos)
  programs.firefox = {
    arkenfox = {
      enable = true;
      version = "140.0";
    };
    profiles.default.arkenfox = {
      enable = true;
      enableAllSections = true;
      "0100"."0101".value = false;
    };
  };

  # After (arkenfox-nix)
  programs.firefox = {
    arkenfox = {
      enable = true;
      version = "140.0";
      profiles.default = {
        enableAllSections = true;
        settings."0100"."0101".value = false;
      };
    };
  };
}
```

## CLI Tools

Arkenfox-nix includes powerful CLI tools for working with user.js files:

```bash
# Extract preferences from a user.js file
arkenfox-nix extract path/to/user.js

# Generate all available versions
arkenfox-nix generate
```

The generate command creates:

1. A `data/` directory containing all output files
2. Individual JSON files for each version (`master.json`, `91.0.json`, etc.)
3. A `default.nix` file with NixOS expressions using relative paths

**Directory structure after generation:**

```
data/
├── default.nix
├── master.json
├── 91.0.json
├── 92.0.json
└── ... (more version files)
```

## Acknowledgments

- [@dwarfmaster](https://github.com/dwarfmaster) for the original [arkenfox-nixos](https://github.com/dwarfmaster/arkenfox-nixos) module that inspired this project.
- The [Arkenfox team](https://github.com/arkenfox/user.js) for their comprehensive Firefox privacy and security configurations.

## License

This project is licensed under the [MIT License](LICENSE).
