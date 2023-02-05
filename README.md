# Leptos TODOs

Client-Side Rendered TODO web application.
A simple toy project to mess around with Leptos and Rust WASM.

## Setup

### Using Docker

If you have Docker and docker-compose installed, you can simply run
`docker compose up --build` and wait a few minutes. After it finishes,
you can open a browser at `http://localhost:8080` to see the application.

### Manually

This project will work using the Stable Rust Toolchain as well, contrary
to most examples on the [official Leptos repository](https://github.com/leptos-rs/leptos).

1. Add WASM to your Rust toolchain using: `rustup target add wasm32-unknown-unknown`.
2. This project uses Tailwind, so install the Tailwind CLI using: `npm i -g tailwindcss`.
3. Install [Trunk](https://trunkrs.dev/#install) for compiling Rust to WASM.
4. Run `trunk serve --open` to run a dev server that hot reloads any changes to your code.
