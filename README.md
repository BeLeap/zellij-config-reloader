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

## Run without a visible pane

Add the built plugin URL to the `load_plugins` block in Zellij's `config.kdl`:

```kdl
load_plugins {
    "file:/absolute/path/to/config-reloader.wasm"
}
```

Zellij loads these plugins in the background when a session starts. Start a new Zellij session after adding this entry.

Send a pipe message to the loaded plugin to reload the active configuration:

```sh
zellij pipe --plugin file:/absolute/path/to/config-reloader.wasm --name reload-config -- reload
```

Use the same plugin URL in both places. Run the pipe command only after the background plugin loads. Otherwise, Zellij opens a visible plugin pane.

Do not use `zellij action start-or-reload-plugin` for this headless plugin. That action opens a visible plugin pane.

## Use as a flake input

```nix
inputs.zellij-config-reloader.url = "github:BeLeap/zellij-config-reloader";
inputs.zellij-config-reloader.inputs.nixpkgs.follows = "nixpkgs-unstable";
```

Use `inputs.zellij-config-reloader.packages.${system}.default` to build the plugin.
