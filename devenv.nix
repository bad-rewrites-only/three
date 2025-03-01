{
  pkgs,
  lib,
  inputs,
  ...
}: let
  pkgs-unstable = import inputs.nixpkgs-unstable {system = pkgs.stdenv.system;};
in rec {
  # https://devenv.sh/basics/
  env.LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";
  env.RUST_FLAG = "'--cfg getrandom_backend=\"wasm_js\"";

  # https://devenv.sh/packages/
  packages = with pkgs-unstable; [
    pkg-config
    wayland
    openssl
    fontconfig
    libGL
    libxkbcommon
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libX11

    trunk
    wasm-bindgen-cli
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    targets = ["wasm32-unknown-unknown"];
    channel = "nightly";
  };
  languages.javascript = {
    npm.enable = true;
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.emu.exec = ''
    emulator -avd aemu -netdelay none -netspeed full
  '';

  enterShell = ''
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/

  # cachix.enable = false;
}
