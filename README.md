# odds 🚀

A smarter `cd`. You know where you want to go. Now your shell does too.

## What is it?

`odds` learns from your navigation history and gets you where you're going with minimal typing. Type a keyword and it jumps straight to the most likely match. Not confident enough to auto-jump? It shows you a numbered list to pick from.

```bash
o config
# → /home/user/projects/myapp/config
```

## How it works

When you run `o <keyword>`, odds:

1. **Searches your history** for directories whose name matches the keyword (exact, prefix, substring, or fuzzy match).
2. **Scores each candidate** using a combination of signals:
   - Match quality (exact > prefix > substring > fuzzy)
   - Frecency — how often and how recently you've visited (with exponential decay, ~3-day half-life)
   - Markov chain — what directories you tend to jump to *from* your current location
   - Session context — directories you've already visited this session
3. **If confident**, jumps immediately. If not, falls back to a filesystem search rooted at your current directory, git repository root, and home directory (up to 5 levels deep).
4. **If still ambiguous**, presents up to 9 options for you to pick from.

You can also pass an explicit path and `o` will jump directly:

```bash
o ./some/explicit/path
```

## Installation

`odds` is not yet available in a package repository and must be installed manually.

### Prerequisites

- Linux or macOS
- Shell: `bash` or `zsh`
- [Rust toolchain](https://rustup.rs/)

### Build from source

```bash
git clone https://github.com/JulianRueck/odds.git
cd odds
cargo build --release
```

Then move the binary somewhere on your `$PATH`:

```bash
cp target/release/odds ~/.local/bin/
```

### Shell integration

Add the following to your `.bashrc` or `.zshrc`:

```bash
eval "$(odds --init bash)"   # for bash
eval "$(odds --init zsh)"    # for zsh
```

Then reload your shell:

```bash
source ~/.zshrc  # or ~/.bashrc
```

> The shell function wraps the `o` binary so that directory changes affect your current shell session.

## Usage

### Jump to a directory

```bash
o <keyword>
```

```bash
o config    # → /home/user/projects/myapp/config
o tests     # → /home/user/projects/myapp/tests
o work      # → /home/user/work
```

### Picker mode

When odds isn't confident about the best match, it shows a numbered list of up to 9 candidates. Type the number and press enter.

```
$ o api

Select a directory (1-3), or 0 to cancel:
1) /home/user/projects/myapp/api
2) /home/user/projects/legacy/api
3) /home/user/work/client/api
Enter number: 2
# → /home/user/projects/legacy/api
```

Enter `0` to cancel without navigating.

## Data storage

`odds` stores its history and session data in `~/.local/share/odds/`.

## How does this compare to [zoxide](https://github.com/ajeetdsouza/zoxide)?

Zoxide is a mature, battle-tested tool that learns from your navigation history and gets you back to places you've been before — it's excellent at what it does.
Odds is exploring a slightly different idea: getting you to directories you *haven't necessarily visited before*, by searching the live filesystem and ranking results using history, session context, and navigation patterns. A key ambition is that the tool learns continuously from your behaviour and gets smarter the more you use it — something that better ML algorithms could take much further over time. Whether that's actually useful in practice remains to be seen — Odds is still in its infancy and the ranking model has a lot of maturing to do. If you need something reliable today, zoxide is the right choice. If the idea sounds interesting and you don't mind rough edges, contributions and feedback are very welcome.
