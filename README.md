<div align="center">

# odds

[![crates.io](https://img.shields.io/crates/v/odds-rueck.svg)](https://crates.io/crates/odds-rueck)

#### A smarter `cd`. You know where you want to go. Now your shell does to.

</div>

## What is it?

odds silently learns from every directory change you make. The more you navigate, the better it gets at predicting where you want to go next — combining where you've been, how recently, and what you tend to do from your current location.

Type a keyword or two and odds finds the best match. Confident enough? It jumps immediately. Not sure? It shows you a list to pick from. Never been there before? It searches your filesystem and ranks what it finds.
```bash
o config
# → /home/user/projects/myapp/config

o proj api
# → /home/user/projects/myapp/api
```

## Installation

### Prerequisites

- Linux or macOS
- Shell: `bash` or `zsh`
- [Rust toolchain](https://rustup.rs/)

### Recommended: install with Cargo

```bash
cargo install odds-rueck
```

### Alternative: Build from source
```bash
git clone https://github.com/JulianRueck/odds.git
cd odds
cargo build --release
```

Then move the binary somewhere on your `$PATH`:
```bash
cp target/release/odds ~/.local/bin/      # user only
cp target/release/odds /usr/local/bin/    # system-wide
```

### Quickest: download precompiled binary

No Rust toolchain required. Grab the latest binary for your platform from the [releases page](https://github.com/JulianRueck/odds/releases/latest):

```bash
# Linux x86_64
curl -L https://github.com/JulianRueck/odds/releases/download/v0.1.2-alpha/odds-linux-x86_64 -o odds
chmod +x odds
mv odds ~/.local/bin/

# Linux arm64
curl -L https://github.com/JulianRueck/odds/releases/download/v0.1.2-alpha/odds-linux-arm64 -o odds
chmod +x odds
mv odds ~/.local/bin/

# macOS Apple Silicon
curl -L https://github.com/JulianRueck/odds/releases/download/v0.1.2-alpha/odds-macos-arm64 -o odds
chmod +x odds
mv odds ~/.local/bin/

# macOS Intel
curl -L https://github.com/JulianRueck/odds/releases/download/v0.1.2-alpha/odds-macos-x86_64 -o odds
chmod +x odds
mv odds ~/.local/bin/
```

### Shell integration

Add the following to your `.bashrc` or `.zshrc`:
```bash
eval "$(odds init bash)"    # for bash
eval "$(odds init zsh)"     # for zsh
```

Then reload your shell:
```bash
source ~/.zshrc  # or ~/.bashrc
```

> The shell function wraps the `o` binary so that directory changes affect your current shell session.

## Usage

### Jump to a directory
```bash
o <keywords>
```
```bash
o config         # → /home/user/projects/myapp/config
o proj api       # → /home/user/projects/myapp/api
o work client    # → /home/user/work/client
```

Keywords are matched against path segments in any order — `o api proj` and `o proj api` produce the same results. Partial matches are allowed and scored proportionally, so you don't need to remember exact names.

### Picker mode

When odds isn't confident about the best match, it shows a numbered list of up to 9 candidates. Type the number and press enter.

```
$ o api

Select a directory (1-3):
1) /home/user/projects/myapp/api
2) /home/user/projects/legacy/api
3) /home/user/work/client/api
Enter number: 2
# → /home/user/projects/legacy/api
```

### Seeding from shell history

On a fresh install odds has no history to learn from. The `seed` command bootstraps it from your existing shell history file, extracting `cd` commands and building an initial frecency and Markov dataset:
```bash
odds seed
```

odds automatically detects your history file via `$HISTFILE`, falling back to `~/.zsh_history` and `~/.bash_history`. You can also point it at a specific file:
```bash
HISTFILE=~/.bash_history odds seed
```

Running `seed` multiple times is safe — it merges into existing data rather than overwriting it.

## Data storage

`odds` stores its history and session data in `~/.local/share/odds/`.

## How does this compare to [zoxide](https://github.com/ajeetdsouza/zoxide)?

Zoxide is a mature, battle-tested tool that learns from your navigation history and gets you back to places you've been before — it's excellent at what it does.
Odds is exploring a slightly different idea: getting you to directories you *haven't necessarily visited before*, by searching the live filesystem and ranking results using history, session context, and navigation patterns. A key ambition is that the tool learns continuously from your behaviour and gets smarter the more you use it — something that better ML algorithms could take much further over time. Whether that's actually useful in practice remains to be seen — Odds is still in its infancy and the ranking model has a lot of maturing to do. If you need something reliable today, zoxide is the right choice. If the idea sounds interesting and you don't mind rough edges, contributions and feedback are very welcome.
