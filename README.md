# Hammer plugin template (Rust)

A starting point for a [Hammer](https://hammer.ink) plugin written in Rust. It builds a working plugin,
**Word count**, which adds a "Count words" item to Project Home's and the scene editor's menus and replies
with each scene's word count. Replace it with your own.

## Make it yours

1. Choose **Use this template** on GitHub to make your own repository from this one.
2. Rename the plugin. In `Cargo.toml`, set the package `name`; the built module and package are named
   after it. In `manifest.toml`, set `id` (lowercase, and unique among Hammer's plugins), `name`, and
   `description`.
3. Ask for what your plugin needs. `[permissions] operations` in `manifest.toml` lists the operations it
   may call; the writer sees that list when they install it.
4. Replace `src/lib.rs` with your plugin's logic. It calls Hammer through `hammer::ops`, a typed function
   for each operation, such as `ops::scene::read(&project, id)`.
5. Choose a license. This template is under 0BSD, so you can use it however you like without keeping
   its notice; replace `LICENSE` with your plugin's own.

## Build and test

You need [rustup](https://rustup.rs), its WebAssembly target, and `zip`:

```sh
rustup target add wasm32-unknown-unknown
cargo test     # run the tests on your machine
./build.sh     # build build/word-count.hammerplugin
```

Install the package in Hammer on desktop: Settings, Plugins, Install plugin. Pick the `.hammerplugin`,
approve its permissions, and restart Hammer.

The kit is [`hammer-plugin-kit`](https://crates.io/crates/hammer-plugin-kit) on crates.io, used as
`hammer`. To move to a newer kit, change its `version` in `Cargo.toml`.

## Layout

| Path | What |
| --- | --- |
| `manifest.toml` | What the plugin adds to Hammer, and the operations it may call |
| `src/lib.rs` | The plugin: the `action` export Hammer calls, its logic, and its tests |
| `build.sh` | Builds the module for `wasm32-unknown-unknown` and packs it with the manifest |
| `.github/workflows/build.yml` | Tests and packages every push; a `v*` tag makes a GitHub release with the `.hammerplugin` |

## Learn more

- [The Hammer developer site](https://developers.hammer.ink): the API reference, with the manifest, every
  kind of extension, and all of Hammer's operations.
- [Recipes](https://github.com/Darkrock-Studios/hammer-plugin-sdk/blob/main/RECIPES.md): a small example of
  each kind of plugin.
- [Hammer's own plugins](https://github.com/Darkrock-Studios/hammer-plugins), fuller examples.
