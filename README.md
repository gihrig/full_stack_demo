<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/.github/main/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/.github/main/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Todo app built with Leptos Tailwind Sqlite and Axum

## A mashup of several Leptos example apps:

- [todo_app_sqlite_axum](https://github.com/leptos-rs/leptos/tree/main/examples/todo_app_sqlite_axum)
- [todomvc](https://github.com/leptos-rs/leptos/tree/main/examples/todomvc)
- [tailwind_axum](https://github.com/leptos-rs/leptos/tree/main/examples/tailwind_axum)
- [hackernews_islands_axum](https://github.com/leptos-rs/leptos/tree/main/examples/hackernews_islands_axum)
- [server_fns_axum](https://github.com/leptos-rs/leptos/tree/main/examples/server_fns_axum)
- [errors_axum](https://github.com/leptos-rs/leptos/tree/main/examples/errors_axum)


This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos --locked
```

Then run
```bash
cargo leptos new --git https://github.com/leptos-rs/start-axum
```

to generate a new project template.

```bash
cd full_stack_demo (or whatever you named your app)
```

to go to your newly created project.
Feel free to explore the project structure, but the best place to start with your application code is in `src/app.rs`.
Addtionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future
5. Run `npm install` in end2end subdirectory before test

## Compiling for Release
```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site

## Testing Your Project
Playwright will build the project before running tests. No other instance can be running.

Install or update Playwright and related test packages.
```bash
cd end2end
npm install
npx playwright install
cd ..
```

Commands below are run from the project root.
```bash
cargo leptos end-to-end

```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.

Local aliases (must be configured in OS)
```
dev = "cargo leptos watch"
release = "cargo leptos build --release"
start = start() {
    # Get the project name from current directory
    local proj_name=$(basename "$PWD")

    # Construct and execute the command
    ./target/release/$proj_name "$@"
}
e2e-test = "cargo leptos end-to-end"
e2e-release = "cargo leptos end-to-end --release"
e2e-report = "cargo leptos end-to-end --report"
```

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
full_stack_demo
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="full_stack_demo"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
