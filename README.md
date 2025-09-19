# Glues

[![crates.io](https://img.shields.io/crates/v/glues.svg)](https://crates.io/crates/glues)
[![LICENSE](https://img.shields.io/crates/l/glues.svg)](https://github.com/gluesql/glues/blob/main/LICENSE)
![Rust](https://github.com/gluesql/glues/workflows/Rust/badge.svg)
[![Chat](https://img.shields.io/discord/780298017940176946?logo=discord&logoColor=white)](https://discord.gg/C6TDEgzDzY)

## Vim-inspired, privacy-first TUI note-taking app with multiple storage options

Glues is a Vim-inspired, terminal-based (TUI) note-taking application that offers flexible and secure storage options. You can store your notes locally, choose Git for distributed version control, or opt for MongoDB for centralized data management.

Glues is designed with a core architecture that operates independently of the TUI, providing robust state management and action handling. Although the current frontend is TUI-based, the architecture allows for easy integration with other frontends such as GUI, iOS, Android, or even running headlessly without a UI. The TUI interface clearly displays the current state and available actions, making it intuitive and easy to use.

With no reliance on third-party services, Glues ensures that your data remains private and fully under your control. Currently, it supports Git and MongoDB for storage, and we plan to integrate additional storage options through [GlueSQL](https://github.com/gluesql/gluesql), giving you even more flexibility in managing your data. The core concept behind Glues is to empower users to choose how their data is handled—whether through local files, Git, MongoDB, or future storage options—without any dependence on a central authority. This makes Glues a sync-enabled application that prioritizes user autonomy and privacy.

<img width="1497" alt="image" src="https://github.com/user-attachments/assets/581c0586-67d9-4e13-9200-454b6ae8c50c" />

## Installation

First, ensure [Rust](https://www.rust-lang.org/tools/install) is installed. Then, install Glues by running:

```bash
cargo install glues
```

For Arch Linux users, Glues is available [in the AUR](https://aur.archlinux.org/packages/glues/):

```bash
paru -S glues # user your favorite AUR helper
```

We're working on making Glues available through more package managers soon.

## Usage

Glues offers various storage options to suit your needs:

* **Instant**: Data is stored in memory and only persists while the app is running. This option is useful for testing or temporary notes as it is entirely volatile.
* **Local**: Notes are stored locally as separate files. This is the default option for users who prefer a simple, file-based approach without any remote synchronization.
* **Git**:
  - Git storage requires three inputs: `path`, `remote`, and `branch`.
  - The `path` should point to an existing local Git repository. For example, you can clone a GitHub repository and use that path.
  - The `remote` and `branch` specify the target remote repository and branch for synchronization.
  - When you modify notes or directories, Glues will automatically sync changes with the specified remote repository, allowing for distributed note management.

  To see how notes and directories are stored using Git, you can refer to the [Glues sample repository](https://github.com/gluesql/glues-sample-note).
* **MongoDB**:
  - MongoDB storage allows you to store your notes in a MongoDB database, providing a scalable and centralized solution for managing your notes.
  - You need to provide the MongoDB connection string and the database name. Glues will handle storing and retrieving notes from the specified database.
  - This option is ideal for users who need centralized data management or work in team environments where notes are shared.
* **Proxy**:
  - Point Glues at an HTTP proxy that exposes the same set of operations as the local backend.
  - Run the bundled proxy server with `cargo run -p glues-proxy-server -- memory` (replace `memory` with `file`, `git`, or `mongo` as needed). The server listens on `127.0.0.1:4000` by default; use `--listen` to change the address.
  - In the TUI entry menu choose `[5] Proxy`, enter the proxy URL (e.g. `http://127.0.0.1:4000`), and Glues will talk to the remote backend just like it does locally.

### Theme Presets

Glues includes several built-in color schemes. The application starts with the
`dark` palette by default, but you can switch presets with the `--theme` option:

```bash
glues --theme pastel
```

The built-in themes are:

* `dark` – 256-color palette
* `light` – 256-color palette
* `pastel` – a light theme defined using RGB values
* `sunrise` – a warm light theme defined using RGB values
* `midnight` – a blue-toned dark theme defined using RGB values
* `forest` – a nature-inspired dark theme defined using RGB values

## Roadmap

Here is our plan for Glues and the features we aim to implement. Below is a list of upcoming improvements to make Glues more useful and versatile. If you have suggestions for new features, please feel free to open a GitHub issue.

* **[In Progress] MCP Server Integration**
  - Integration with an MCP server is currently the top priority to enable secure interaction between Glues and external LLMs.
  - The setup will include **API token-based access control**, allowing permission scoping such as directory-level access and note read/write operations.
  - With this approach, LLMs will operate strictly within the boundaries defined by the user.
  - The MCP server runs locally, ensuring that LLM-based features are fully managed and authorized by the user.
* **Enhanced Note Content Support:** Add support for richer note content, including tables and images, in addition to plain text. This will help users create more detailed and organized notes.
* **Search and Tagging Improvements:** Improve search with tag support and advanced filtering to make it easier to find specific notes.
* **Customizable Themes:** Allow users to personalize the TUI interface with customizable themes.
* **Additional Package Manager Support:** Expand distribution beyond Cargo, making Glues available through more package managers like Homebrew, Snap, and APT for easier installation.
* **Storage Migration:** Add a feature to migrate data between different storage options, such as from local files to Git.
* **More Vim Keybindings:** Integrate Vim keybindings for users who prefer Vim-like shortcuts.
* **Additional Storage Backends:** Support more storage options like Redis and object storage for greater flexibility.

## Development

### Coverage

Install [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) once with:

```bash
cargo install cargo-llvm-cov
```

Key commands exposed via project aliases:

- `cargo coverage` - run the workspace under coverage and emit an HTML report at `target/llvm-cov/html/index.html`
- `cargo coverage-report` - print a text summary (run after `cargo coverage --no-report`; append `--fail-under-lines <N>` to guard CI)
- `cargo coverage-lcov` - export `target/llvm-cov/lcov.info` for external services
- `cargo coverage-clean` - remove coverage artifacts

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](https://github.com/gluesql/glues/blob/main/LICENSE) file for details.
