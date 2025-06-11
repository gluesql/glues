# Glues

[![crates.io](https://img.shields.io/crates/v/glues.svg)](https://crates.io/crates/glues)
[![LICENSE](https://img.shields.io/crates/l/glues.svg)](https://github.com/gluesql/glues/blob/main/LICENSE)
![Rust](https://github.com/gluesql/glues/workflows/Rust/badge.svg)
[![Chat](https://img.shields.io/discord/780298017940176946?logo=discord&logoColor=white)](https://discord.gg/C6TDEgzDzY)

## Vim-inspired, privacy-first TUI note-taking app with multiple storage options

Glues is a Vim-inspired, terminal-based (TUI) note-taking application that offers flexible and secure storage options. You can store your notes locally, choose Git for distributed version control, or opt for MongoDB for centralized data management. This flexibility allows you to manage your notes in the way that best suits your workflow, whether you prefer the simplicity of local files, the collaboration capabilities of Git, or the scalability of MongoDB. For additional support, log file formats such as CSV and JSON are also available.

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
* **CSV or JSON**:
  - These formats store notes as simple log files, ideal for quick data exports or reading logs.
  - CSV saves data in comma-separated format, while JSON uses JSONL (JSON Lines) format.

### Theme Presets

Glues includes several built-in color schemes. Choose one with the `--theme` option:

```bash
glues --theme pastel
```

Available presets are `dark`, `light`, and `pastel`. The pastel palette is defined
in `pastel-theme.toml` and can be used as a template when contributing new themes.

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
* **Storage Migration:** Add a feature to migrate data between different storage options, such as from CSV to Git.
* **More Vim Keybindings:** Integrate Vim keybindings for users who prefer Vim-like shortcuts.
* **Additional Storage Backends:** Support more storage options like Redis and object storage for greater flexibility.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](https://github.com/gluesql/glues/blob/main/LICENSE) file for details.
