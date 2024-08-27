# Glues

[![crates.io](https://img.shields.io/crates/v/glues.svg)](https://crates.io/crates/glues)
[![LICENSE](https://img.shields.io/crates/l/glues.svg)](https://github.com/gluesql/glues/blob/main/LICENSE)
![Rust](https://github.com/gluesql/glues/workflows/Rust/badge.svg)
[![Chat](https://img.shields.io/discord/780298017940176946?logo=discord&logoColor=white)](https://discord.gg/C6TDEgzDzY)

## Flexible, privacy-first TUI note-taking app with multiple storage options

Glues is a lightweight, terminal-based (TUI) note-taking application that offers flexible and secure storage options. You can choose to store your notes locally in CSV or JSON formats, or utilize Git as a storage option for version control. This flexibility allows you to manage your notes in the way that best suits your workflow, whether you prefer the simplicity of local files or the robustness of Git integration.

Glues is designed with a core architecture that operates independently of the TUI, providing robust state management and action handling. The TUI interface clearly displays the current state and available actions, making it intuitive and easy to use.

With no reliance on third-party services, Glues ensures that your data remains private and fully under your control. Currently, it supports Git for storage, but we plan to integrate additional storage options through [GlueSQL](https://github.com/gluesql/gluesql), giving you even more flexibility in managing your data. The core concept behind Glues is to empower users to choose how their data is handled—whether through local files, Git, or future storage options—without any dependence on a central authority. This makes Glues a sync-enabled application that prioritizes user autonomy and privacy.

[![Glues Demo](http://img.youtube.com/vi/unqsDSU9HFs/0.jpg)](https://youtu.be/unqsDSU9HFs "Watch the Glues Demo Video")

*Click the image above to watch the demo video and see Glues in action!*

## Installation

First, ensure [Rust](https://www.rust-lang.org/tools/install) is installed. Then, install Glues by running:

```bash
cargo install glues
```

We're working on making Glues available through more package managers soon. For now, installing via Cargo is the way to go.


## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](https://github.com/gluesql/glues/blob/main/LICENSE) file for details.
