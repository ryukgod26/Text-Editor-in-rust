# Red - (Rust Editor)

This is a small text editor created in Rust named Red (Rust Editor). I am building this Project to build something big. I am Going to add many features in this text editor like: Plugins, Undo/redo etc. too. This Text Editor is still in development. It Currently support all the common keys and basic highlighting of text in C, C++ and Rust.

## Features

- Support of Arrow Keys.
- Support of Keys like Page Up, Page Down, Home and End.
- Save as feature
- Syntax Highlighting for C, Cpp and Rust Files.
- Simple file open/save support.
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

- `red/src/editor/editor.rs` — main app loop, command processing and high-level coordination.
- `red/src/editor/uicomponents/view` — view, search info, highlighters, and rendering pipeline.
- `red/src/editor/uicomponents/view/highlighter` — syntax and search highlighters (including `rustsyntaxhighlighter.rs`).
- `red/src/editor/line` — grapheme-aware line model and conversion helpers (byte <-> grapheme indices).
- `red/src/editor/terminal.rs` — terminal wrapper (raw mode, cursor, execution buffer).

If you want to modify or extend any behavior (add a new highlighter, change keybindings, or add editor commands), start in these modules.

## Build (native Linux)

On a Linux machine (or WSL2) with Rust installed, from repository root:

```bash
# change to the crate directory
cd red
# build in release mode
cargo build --release
# run the built binary
./target/release/red [optional-file-to-open]
```

If you want to build the debug binary (slower, with symbols):

```bash
cargo build
./target/debug/red
```

## Build from Windows (WSL / Docker / cross)

Recommended: use WSL2 or Docker when you need a Linux binary. Options:

- WSL2: follow the native Linux steps inside your WSL shell.
- Docker (from any host):

```bash
docker run --rm -it -v "$(pwd)":/usr/src/app -w /usr/src/app rust:latest bash -lc "cd red && cargo build --release"
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
./target/release/red
# open a file
./target/release/red file.txt
```

Key bindings (common):

- Ctrl-S: save
- Ctrl-Q: quit (with confirmation if file is modified)
- Arrow keys: move caret (left/right/up/down)
- PageUp / PageDown / Home / End
- Typing characters inserts text (basic editing supported)

## License

This repository includes a `LICENSE` file in the top-level. Follow that license for reuse and contributions.
