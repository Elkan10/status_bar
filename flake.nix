{
  description = "Rust status bar for Hyprland";

  inputs = { };

  outputs = { ... }: {
    homeManagerModules.default = {config, lib, pkgs, ...}: {
      config = lib.mkIf config.status_bar.enable
      (let
        generatedConfig = builtins.toJSON config.status_bar.settings;
      in {
        home.packages = [
          (pkgs.rustPlatform.buildRustPackage {
            pname = "status_bar";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.cmake pkgs.libxkbcommon pkgs.pkg-config ];
            cargoLock.outputHashes = {
              "widgets-0.1.0" = "sha256-w8BSUiQTe4h0NJWhwSmG7syrFBtGR9fB8unaAD5Giag=";
            };
          })
        ];
        home.file.".config/status_bar/config.json".text = generatedConfig;
      });
      options = {
        status_bar.enable = lib.mkEnableOption "Status Bar";
        status_bar.settings = {
          icons = lib.mkOption {
            description = "The icons for workspaces";
            type = lib.types.listOf lib.types.str;
          };
          width = lib.mkOption {
            description = "Width (in pixels) of window";
            type = lib.types.int;
          };
          height = lib.mkOption {
            description = "Height (in pixels) of window";
            type = lib.types.int;
          };
          exclusive = lib.mkOption {
            description = "Exclusize zone height (in pixels)";
            type = lib.types.int;
          };
          cpu_label = lib.mkOption {
            description = "CPU label for temp reading";
            default = "Tctl";
            type = lib.types.str;
          };
          text_size = lib.mkOption {
            description = "Text size (in points)";
            type = lib.types.int;
          };
          text_height = lib.mkOption {
            description = "Text position (in y)";
            type = lib.types.int;
          };
          bg_color = lib.mkOption {
            description = "Background color";
            type = lib.types.str;
          };
          text_color = lib.mkOption {
            description = "Text color";
            type = lib.types.str;
          };
          selected_color = lib.mkOption {
            description = "Color of selected workspace";
            type = lib.types.str;
          };
          font = lib.mkOption {
            description = "Name of font to use for text";
            type = lib.types.str;
          };
        };
      };
    };
  };
}
