# Building and shipping

See also: [architecture](ARCHITECTURE.md)

## Running in development

```sh
cargo run -p cao_app
```

## A Windows executable from macOS

The project cross-compiles to Windows without a Windows machine.

Prerequisites, once:

```sh
rustup target add x86_64-pc-windows-gnu
brew install mingw-w64          # Debian/Ubuntu: apt install mingw-w64
```

Then:

```sh
./scripts/build-windows.sh
```

The executable comes out at `target/x86_64-pc-windows-gnu/release/cao.exe`
(~29 MB). It depends on nothing but the system DLLs of Windows — no mingw DLL
to carry alongside: the `.exe` on its own is enough, copied onto the Windows
machine and double-clicked.

The linker is already set up in `.cargo/config.toml`, so
`cargo build --release -p cao_app --target x86_64-pc-windows-gnu` works
directly too.

### Details

- In release the executable is marked as a graphical application
  (`windows_subsystem = "windows"`): no black console window on launch. In
  debug the console stays, so panics and logs can be read.
- The target is `gnu` (mingw) rather than `msvc`, because `msvc` wants the
  Microsoft headers and libraries, which are not freely redistributable.
  Rendering goes through DX12 or Vulkan either way.

## When it crashes

A graphical application on Windows opens no console: a panic leaves nothing to
read, and "it crashed" is all one gets. Every panic is therefore appended to a
file:

| System | File |
| --- | --- |
| macOS | `~/Library/Application Support/dev.cao.cao/plantages.log` |
| Windows | `%APPDATA%\cao\cao\data\plantages.log` |
| Linux | `~/.local/share/cao/plantages.log` |

It holds the time, the message and the call stack. The message also goes to the
terminal when there is one.

## Continuous integration

`.github/workflows/ci.yml` runs on **every push, on any branch** — pull request
or not. Four jobs in parallel: formatting (`cargo fmt --all --check`), Clippy
(`--workspace --all-targets`, `-D warnings`), the tests
(`cargo test --workspace`), and the cross-compilation to Windows through the
very same `scripts/build-windows.sh` as above.

That last job **publishes the executable as an artefact** (`cao-windows`), but
only from `main`: keeping one `.exe` per branch push would serve nobody and
fill the quota.

What is checked is the **tip of the branch**, not its merge with `main`: a
branch that went green last week can still break `main` today, and that is what
the rebase before a merge is for.

The first three also run locally before every commit
(`scripts/verify.sh`, called by `.githooks/pre-commit`), and
`crates/app/tests/gate.rs` fails if the two lists stop agreeing.

## Other platforms

Linux and macOS compile natively with `cargo build --release -p cao_app`. There
are no installer packages yet (`.msi`, `.dmg`, AppImage): that comes when the
software is distributed.
