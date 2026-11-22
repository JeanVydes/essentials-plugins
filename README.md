# essentials-plugins

Plugins that add optional tools to Norte Code. Each plugin is a separate `cdylib`
crate in this workspace, so Norte Code installs only the ones a person wants.

## browser

Declares the `browser_use` tool from Norte's `norte-tools` crate. The executor
drives a Chromium based browser that is already running with a CDP endpoint, by
default `http://127.0.0.1:9222`. The tool can navigate, click, type, scroll,
wait, extract interactive elements, take screenshots and manage tabs.

The implementation stays in Norte. This plugin only returns the tool to the
host; Norte Code registers it, applies tool rules and reports its origin.

```sh
norte-code plugins add https://github.com/JeanVydes/essentials-plugins --package essentials-plugin-browser
```

Restart `norte` after installing, updating or removing a plugin. Norte Code
loads plugins at startup.

## Development

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```
