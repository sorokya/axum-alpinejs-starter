[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/sorokya/axum-alpinejs-starter/blob/master/LICENSE)

# Axum + Alpine.js Example Project

This is an opinionated starter project that combines the power of [Axum](https://github.com/tokio-rs/axum), [Alpine.js](https://alpinejs.dev/), and [Alpine-Ajax](https://alpine-ajax.js.org/) to create a simple web application with a Rust backend and a lightweight frontend.

## Features

- [Axum](https://github.com/tokio-rs/axum) for building the backend server
- [Alpine.js](https://alpinejs.dev/) (with [Alpine-Ajax](https://alpine-ajax.js.org/)) for lightweight frontend interactivity
- [Biome](https://biomejs.dev/) for code formatting and linting
- [Modern Normalize](https://github.com/sindresorhus/modern-normalize) for CSS normalization

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

- `src/`: Contains the Rust source code for the Axum server.
  - `main.rs`: Entry point of the application.
  - `app.rs`: Application setup and route definitions.
  - `error.rs`: Application error enum (implements axum::response::IntoResponse).
  - `render.rs`: Template rendering helper.
  - `routes/`: Route handlers
  - `views/`: Askama template structures
- `assets/`: Contains static assets like CSS and JavaScript files.
- `biome.json`: Configuration file for Biome code formatter and linter.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
