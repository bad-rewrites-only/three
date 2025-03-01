{
  pkgs,
  lib,
  ...
}: rec {
  # https://devenv.sh/basics/
  env.LD_LIBRARY_PATH = "${lib.makeLibraryPath packages}";

  # https://devenv.sh/packages/
  packages = with pkgs; [
    pkg-config

    wayland
    openssl
    libGL
    libxkbcommon
  ];

  # https://devenv.sh/languages/
  # languages.rust.enable = true;

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

  cachix.enable = false;
}
