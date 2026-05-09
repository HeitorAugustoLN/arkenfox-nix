{
  lib,
  rustPlatform,
  openssl,
  pkg-config,
  versionCheckHook,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "arkenfox-nix";
  version = "1.0.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  doInstallCheck = true;
  nativeInstallCheckInputs = [ versionCheckHook ];
  versionCheckProgram = "${placeholder "out"}/bin/arkenfox-nix";

  meta = {
    changelog = "https://github.com/HeitorAugustoLN/arkenfox-nix/releases/tag/v${finalAttrs.version}";
    description = "CLI for arkenfox-nix";
    homepage = "https://github.com/HeitorAugustoLN/arkenfox-nix";
    license = lib.licenses.mit;
    mainProgram = "arkenfox-nix";
    maintainers = [ lib.maintainers.HeitorAugustoLN ];
  };
})
