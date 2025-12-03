# hecto — terminal text editor (Rust)

A small terminal text editor written in Rust. It uses `crossterm` for terminal handling and supports Unicode grapheme-aware rendering (via `unicode-segmentation` and `unicode-width`). The editor is packaged as the `hecto` crate in this repository.

## Project overview

- Crate name: `hecto` (see `hecto/Cargo.toml`)
- Version: `0.1.0` (see `hecto/Cargo.toml`)
- Language: Rust (edition 2024)
- Terminal library: `crossterm`
- Unicode handling: `unicode-segmentation`, `unicode-width`

The source lives under `hecto/src/` and the key modules are:

- `main.rs` — program entry point (constructs `Editor` and runs it).
- `editor.rs` — top-level editor application (initialization, event loop, command processing).
- `editor/*` — editor internals:
	- `terminal.rs` — terminal wrapper (screen buffer, cursor, raw mode, resize handling).
	- `view.rs` and `view/*` — editor view, buffer, rendering logic, file handling.
	- `editorcommand.rs` — input -> command mapping (keyboard handling and command enums).
	- `line.rs` — representation of a single line with grapheme fragments.
	- `position.rs` / `size.rs` — small geometry types.
	- `statusbar.rs`, `messagebar.rs`, `commandbar.rs` — UI components.

## Features

- Terminal-based UI using `crossterm`.
- Unicode-aware rendering (grapheme clusters and width handling).
- Simple file open/save support.
- Basic navigation (arrows, page up/down, home/end), editing, and status bar.
- Resize handling.

### Advanced features

- Incremental search: enter the editor's Find prompt (via the Find command) and type a query — matches are highlighted live and a selected match can be navigated. Search state, directions, and match selection are managed by the `view` highlighter components.
- Syntax highlighting: a modular syntax highlighter system exists; a Rust-specific highlighter is included (highlighter logic lives under `uicomponents/view/highlighter`).
- Command bar / prompt: a `CommandBar` lets the editor collect text input for save-as, find, and other prompt workflows.
- UI components: `StatusBar`, `MessageBar`, `CommandBar` and view rendering are implemented as small components under `uicomponents`.
- Annotated rendering: the editor uses an `AnnotatedString` abstraction to attach annotations (matches, selections, syntax tokens) to ranges for rendering.
- Grapheme-aware buffer: lines are stored and rendered as grapheme clusters (via `unicode-segmentation`) so multi-codepoint characters behave correctly when moving the caret or editing.
- Robust resize handling: terminal resize events are handled and the editor reflows the view and UI components.

These features are implemented across these modules:

- `hecto/src/editor/editor.rs` — main app loop, command processing and high-level coordination.
- `hecto/src/editor/uicomponents/view` — view, search info, highlighters, and rendering pipeline.
- `hecto/src/editor/uicomponents/view/highlighter` — syntax and search highlighters (including `rustsyntaxhighlighter.rs`).
- `hecto/src/editor/line` — grapheme-aware line model and conversion helpers (byte <-> grapheme indices).
- `hecto/src/editor/terminal.rs` — terminal wrapper (raw mode, cursor, execution buffer).

If you want to modify or extend any behavior (add a new highlighter, change keybindings, or add editor commands), start in these modules.

## Build (native Linux)

On a Linux machine (or WSL2) with Rust installed, from repository root:

```bash
# change to the crate directory
cd hecto
# build in release mode
cargo build --release
# run the built binary
./target/release/hecto [optional-file-to-open]
```

If you want to build the debug binary (slower, with symbols):

```bash
cargo build
./target/debug/hecto
```

## Build from Windows (WSL / Docker / cross)

Recommended: use WSL2 or Docker when you need a Linux binary. Options:

- WSL2: follow the native Linux steps inside your WSL shell.
- Docker (from any host):

```bash
docker run --rm -it -v "$(pwd)":/usr/src/app -w /usr/src/app rust:latest bash -lc "cd hecto && cargo build --release"
```

- `cross` (uses Docker images to cross-compile):

```bash
cargo install cross
cross build --target x86_64-unknown-linux-gnu --release
```

Note: `crossterm` is pure Rust and normally requires no extra native dependencies.

## Run

Run the binary and optionally pass a file path to open:

```bash
# open no file
./target/release/hecto
# open a file
./target/release/hecto myfile.txt
```

Key bindings (common):

- Ctrl-S: save
- Ctrl-Q: quit (with confirmation if file is modified)
- Arrow keys: move caret (left/right/up/down)
- PageUp / PageDown / Home / End
- Typing characters inserts text (basic editing supported)

The exact command mapping is implemented in `hecto/src/editor/editorcommand.rs`.

## Development notes

- The editor uses a small `Terminal` wrapper in `editor/terminal.rs`. If arrow keys or caret behaviour seems swapped, check the mapping between `Position { row, col }` and how `Terminal::move_caret_to` consumes them (row vs col order must be consistent).
- Many modules are small and focused (line rendering, view buffer, UI components). See `hecto/src/editor/` for details.
- Unit tests: currently none included; you can run `cargo test` if you add tests.

## Contributing

- Open issues describing bugs or feature requests.
- Pull requests are welcome — keep changes small and focused.
- Follow Rust formatting and clippy lints (the project uses `clippy` warnings in `main.rs`).

## License

This repository includes a `LICENSE` file in the top-level. Follow that license for reuse and contributions.

## Contact

If you want help building, testing or adding features, open an issue or create a PR.

---

(README generated by an assistant; adjust wording or add screenshots/usage examples as you like.)