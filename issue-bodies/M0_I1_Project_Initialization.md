### Summary

This issue covers the initial setup of the monorepo, including the Rust backend with Tauri and the Svelte frontend. The goal is to have a single command (`cargo tauri dev`) that successfully builds and launches a blank window.

### Breakdown of Tasks

-   **[ ] Initialize Rust Project:**
    -   Create a new Rust binary project using `cargo new nyx-agent --bin`.
    -   Set up the workspace structure inside `src-tauri/Cargo.toml` to prepare for future library crates.

-   **[ ] Integrate Tauri:**
    -   Add `tauri` as a dependency in `src-tauri/Cargo.toml`.
    -   Run `cargo tauri init` to generate the required Tauri configuration files (`tauri.conf.json`, `build.rs`).
    -   Configure `tauri.conf.json` with the correct application identifier and settings.

-   **[ ] Initialize Svelte Frontend:**
    -   Inside the project root, initialize a new Svelte project in the `src/` directory using `npm create svelte@latest src`.
    -   Follow the prompts to set up a skeleton project (TypeScript is recommended).
    -   Run `npm install` to fetch frontend dependencies.

-   **[ ] Link Frontend and Backend:**
    -   Modify the `tauri.conf.json` file.
    -   Set `build.devPath` to `http://localhost:5173` (or the default Svelte dev server port).
    -   Set `build.distDir` to `../src/build` (or the Svelte build output directory).

-   **[ ] Validation:**
    -   Run `cargo tauri dev` from the root directory.
    -   The command should successfully compile the Rust backend, start the Svelte dev server, and open a desktop window displaying the default Svelte welcome page.