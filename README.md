# Glues

[![crates.io](https://img.shields.io/crates/v/glues.svg)](https://crates.io/crates/glues)
[![LICENSE](https://img.shields.io/crates/l/glues.svg)](https://github.com/gluesql/glues/blob/main/LICENSE)
![Rust](https://github.com/gluesql/glues/workflows/Rust/badge.svg)
[![Chat](https://img.shields.io/discord/780298017940176946?logo=discord&logoColor=white)](https://discord.gg/C6TDEgzDzY)

## Flexible, privacy-first TUI note-taking app with multiple storage options

Glues is a lightweight, terminal-based (TUI) note-taking application that offers flexible and secure storage options. You can choose to store your notes locally in CSV or JSON formats, or utilize Git as a storage option for version control. This flexibility allows you to manage your notes in the way that best suits your workflow, whether you prefer the simplicity of local files or the robustness of Git integration.

Glues is designed with a core architecture that operates independently of the TUI, providing robust state management and action handling. Although the current frontend is TUI-based, the architecture allows for easy integration with other frontends such as GUI, iOS, Android, or even running headlessly without a UI. The TUI interface clearly displays the current state and available actions, making it intuitive and easy to use.

With no reliance on third-party services, Glues ensures that your data remains private and fully under your control. Currently, it supports Git for storage, but we plan to integrate additional storage options through [GlueSQL](https://github.com/gluesql/gluesql), giving you even more flexibility in managing your data. The core concept behind Glues is to empower users to choose how their data is handled—whether through local files, Git, or future storage options—without any dependence on a central authority. This makes Glues a sync-enabled application that prioritizes user autonomy and privacy.

[![Glues Demo](http://img.youtube.com/vi/unqsDSU9HFs/0.jpg)](https://youtu.be/unqsDSU9HFs "Watch the Glues Demo Video")

*Click the image above to watch the demo video and see Glues in action!*

## Installation

First, ensure [Rust](https://www.rust-lang.org/tools/install) is installed. Then, install Glues by running:

```bash
cargo install glues
```

We're working on making Glues available through more package managers soon. For now, installing via Cargo is the way to go.

## Usage

Glues offers five storage options to suit your needs:

* **Instant**: Data is stored in memory and only persists while the app is running. This is useful for testing purposes.
* **CSV**: Notes are saved in CSV format. When you provide a path, Glues will load existing data if available or create a new file if none exists.
* **JSON**: Notes are stored in JSON format, specifically using JSONL (JSON Lines). This functions similarly to CSV storage, allowing you to provide a path to load or create data.
* **File**: Glues uses a custom storage format where each note and directory is stored as separate files. This is ideal for users who prefer a more granular file-based approach.
* **Git**:
  - Git storage requires three inputs: `path`, `remote`, and `branch`.
  - The `path` should point to an existing local Git repository, similar to the file storage path. For example, you can clone a GitHub repository and use that path.
  - The `remote` and `branch` specify the target remote repository and branch for synchronization.
  - When you modify notes or directories, Glues will automatically sync changes with the specified remote repository.

  To see how notes and directories are stored using Git, you can refer to the [Glues sample repository](https://github.com/gluesql/glues-sample-note).

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](https://github.com/gluesql/glues/blob/main/LICENSE) file for details.
