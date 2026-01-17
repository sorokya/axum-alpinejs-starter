[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/sorokya/axum-alpinejs-starter/blob/master/LICENSE)

# Axum + Alpine.js Example Project

This is an opinionated starter project that combines the power of [Axum](https://github.com/tokio-rs/axum), [Alpine.js](https://alpinejs.dev/), and [Alpine-Ajax](https://alpine-ajax.js.org/) to create a simple web application with a Rust backend and a lightweight frontend.

## Features

- [Axum](https://github.com/tokio-rs/axum) for building the backend server
- [Askama](https://askama.readthedocs.io/en/stable/) for server-side HTML templating
- [Alpine.js](https://alpinejs.dev/) (with [Alpine-Ajax](https://alpine-ajax.js.org/)) for lightweight frontend progressive enhancement
- [Biome](https://biomejs.dev/) for code formatting and linting
- [Modern Normalize](https://github.com/sindresorhus/modern-normalize) for CSS normalization
- [esbuild](https://esbuild.github.io/) for fast JavaScript and CSS bundling
  - You could drop all of the front end tooling and just use plain HTML/CSS/JS if you prefer

## Pre-requisites

- Rust and Cargo installed (Windows installer: https://win.rustup.rs/x86_64)
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
- Node.js and PNPM installed
    ```bash
    # Install nvm (Node Version Manager) recommended
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash

    # Install Node.js (LTS version)
    nvm install --lts

    # Install PNPM
    npm install -g pnpm
    ```
- [cargo-watch](https://crates.io/crates/cargo-watch) for automatic server reloading during development
    ```bash
    cargo install cargo-watch
    ```

## Getting Started

To get started with this project, follow these steps:

1. Clone the repository:
   ```bash
    git clone https://github.com/sorokya/axum-alpinejs-starter.git
    cd axum-alpinejs-starter
    ```
2. Install dependencies:
    ```bash
    pnpm install
    ```
3. Run the development server:
    ```bash
    pnpm dev
    ```
4. Open your browser and navigate to http://localhost:3000 to see the application in action.

## Project Structure

```bash
src
├── app.rs # Application setup and route definitions
├── error.rs # Application error enum (implements axum::response::IntoResponse)
├── main.rs # Entry point of the application
├── render.rs # Template rendering helper
├── routes # Route handlers
└── views # Askama template structures
templates # Askama template markup files
public # Compiled static assets + anything else to be served statically
assets # JS/CSS source files (compiled with esbuild)
```
## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
