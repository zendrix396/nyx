### Summary

This issue covers the creation of essential "community health" files and a basic Continuous Integration (CI) pipeline. This establishes best practices for contributions, bug reporting, and automated testing from the very beginning, inspired by the structure of mature open-source projects.

### Breakdown of Tasks

-   **[ ] Create Contribution Guidelines:**

    -   Create a `CONTRIBUTING.md` file.

    -   Outline the branching strategy (e.g., feature branches off `main`).

    -   Define the pull request process and required formatting.

    -   Add instructions for setting up the local development environment.

-   **[ ] Implement GitHub Issue and PR Templates:**

    -   Create a `.github/ISSUE_TEMPLATE` directory.

    -   Add a template for bug reports (`bug_report.md`) that prompts for environment details, reproduction steps, and expected behavior.

    -   Add a template for feature requests (`feature_request.md`) that asks for the problem description and proposed solution.

    -   Create a `pull_request_template.md` in the `.github/` directory.

-   **[ ] Add Foundational Documentation:**

    -   Create a `CODE_OF_CONDUCT.md` file.

    -   Create a `SECURITY.md` file outlining the process for reporting vulnerabilities.

    -   Add a `LICENSE` file (e.g., MIT or Apache 2.0).

-   **[ ] Set Up Basic CI Workflow:**

    -   Create a `.github/workflows/rust-ci.yml` file.

    -   The workflow should trigger on pushes to `main` and on pull requests.

    -   Define jobs to:

        1.  Check code formatting with `cargo fmt -- --check`.

        2.  Lint the code with `cargo clippy -- -D warnings`.

        3.  Build the Rust backend in release mode with `cargo build --release`.

        4.  Run the test suite with `cargo test --workspace`.

