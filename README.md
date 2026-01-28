![logo](https://i.imgur.com/E0oNrjO.png)

*A blazingly fast test/code editor written in **rust** and built on top of [egui](https://docs.rs/egui/latest/egui/) that works out of the box.*

---

# Installation

Rode is shipped across multiple platforms, mainly *Github Releases* and [crates.io](https://crates.io). Note that nightly builds are an ongoing process and are planned to arrive in the near future.
If you would like to compile from source, please follow the instructions below.

> **Prerequisites** - Ensure you have the `cargo` package manager installed since this project requires Rust and a few of its published crates

1. Clone the repository:

```
git clone https://github.com/kashsuks/rode
```
2. Run the startup command:
```
cargo run
```
You will see that the required packages will get installed and the application may (or may not) run depending on whether the current commit is stable. This will be discussed in the following note.

>[!NOTE]
> The Github commits are not guaranteed to be stable since they are meant to be developer logs (**not user friendly versions**).
> Do not expect them to work because I am a solo developer and I have too much shit to deal with.
> The same applies to nightly builds (when they do arrive).
> They will not be stable. Do not expect them to be stable.

---

# Features

- Custom theming options (in and out of the editor [GUI](https://en.wikipedia.org/wiki/Graphical_user_interface))
- Fuzzy finding (with Neovim keybinds)
- File tree navigation
- Vim motions (commands) currently for navigation
- Settings/preferences
- System default terminal usage

--- 

# License

Copyright (c) 2025-Present [Kashyap](https://github.com/kashsuks) and [Contributors](https://github.com/kashsuks/Rode/graphs/contributors). `Rode` is a free and open-source software licensed under the [GNU General Public License Version 3](https://www.gnu.org/licenses/gpl-3.0.en.html). Official logo was created by [Kashyap Sukshavasi](https://github.com/kashsuks).