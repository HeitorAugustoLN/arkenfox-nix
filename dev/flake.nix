{
  inputs = {
    arkenfox-nix = {
      type = "path";
      path = "../.";
    };

    nixpkgs.follows = "arkenfox-nix/nixpkgs";

    treefmt-nix = {
      type = "github";
      owner = "numtide";
      repo = "treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = _: { };
}
