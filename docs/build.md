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

**Every one of them calls nothing but `cargo`.** No network, no secret, no
metered anything — which is why the gate is twelve seconds and stays there.
#388 proposed a fifth job reading each pull request against its issue with a
model, and it was refused for exactly that: an API key in this repository, and
a bill per pull request. The reading it wanted still happens, before the pull
request is opened, by the agent that wrote the branch — `open-a-task` and
`review-rust` ask for it. Anyone proposing a job that calls out to a service
should start from that refusal rather than from scratch.

## Waiting for a long command

**A command that is waited for announces its own end.** Run it in the
background and read what it leaves behind:

```sh
( cargo test --workspace > /tmp/ws.log 2>&1; echo $? > /tmp/ws.done ) &
until [ -e /tmp/ws.done ]; do sleep 5; done
```

It always ends, it hands back the exit code, and no pattern can recognise
itself.

**Never watch for a process by name.** `pgrep -f 'cargo test --workspace'`
reads whole command lines, and the shell running the loop has that very string
in its own: the pattern finds itself and the count never falls to zero. An
agent lost hours to that here, on a run that had finished in its first minute.
`pgrep -c` makes it worse, since the flag is GNU-only — on macOS and the BSDs
it prints a usage message and nothing at all on stdout, so the comparison never
matches and the loop is immortal.

Which is the wider rule: **a flag that is not in POSIX is checked before it is
written down.** `pgrep -c`, `sed -i`, `date -d` and `readlink -f` all differ
between GNU and BSD. This repository is written on macOS, and read by people
who are not.

## Other platforms

Linux and macOS compile natively with `cargo build --release -p cao_app`. There
are no installer packages yet (`.msi`, `.dmg`, AppImage): that comes when the
software is distributed.
