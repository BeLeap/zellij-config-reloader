# Zellij config reloader

A headless Zellij WASM plugin that reloads the active configuration when it receives a pipe message.

## Compatibility

The plugin pins `zellij-tile` 0.46.0. Build and run it with Zellij 0.46.0. Zellij 0.45.1 does not provide `reload_config_file()`.

The plugin requests the `Reconfigure` permission when it loads. Grant this permission in the Zellij plugin configuration.

## Build

Run `nix build` from this repository. The output contains:

```text
share/zellij/plugins/config-reloader.wasm
```

The flake exports the package as `packages.<system>.default`.

## Use as a flake input

```nix
inputs.zellij-config-reloader.url = "github:BeLeap/zellij-config-reloader";
inputs.zellij-config-reloader.inputs.nixpkgs.follows = "nixpkgs-unstable";
```

Use `inputs.zellij-config-reloader.packages.${system}.default` to build the plugin.
