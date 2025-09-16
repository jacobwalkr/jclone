Jacob's Clone Tool (jclone)
===========================

https://naff.software

A command line tool for cloning and organising git repositories, ~~over-engineered~~ _finely-crafted_ in Rust.

What can I say? I like to keep my repos organised.

# Download

[Head to the releases](https://github.com/jacobwalkr/jclone/releases) and download latest version as a compiled binary or a package for your preferred flavour of Linux. If you use the binary, don't forget to make sure it's placed somewhere in your `$PATH`!

# Usage

By default, clones the given repository into `$HOME/src/<path/to/repo>`. For example:

```
$ jclone git@github.com:rust-lang/rustlings.git
```

Gives you:

```
$ jclone git@github.com:rust-lang/rustlings.git
Cloning into '/home/ferris/src/rust-lang/rustlings'...
remote: Enumerating objects: 14038, done.
... more git output
Resolving deltas: 100% (8076/8076), done
🎉 Done!
```

# Configuration

You can configure jclone by creating a configuration file at `$HOME/.jclone.toml`:

```toml
base_dir = "/home/ferris/code" # defaults to "$HOME/src"
use_host_dir = false # default: true
use_full_path = false # default: true
output = "git-only" # options: "git-only", "no-git", "quiet", "default"
git_executable = "/usr/local/bin/git" # default: "git"

[[variant]]
host = "git.example.com"
base_dir = "/home/ferris/work"
use_full_path = true
output = "quiet"
```

Yields something like:

```
$ jclone git@github.com:rust-lang/rustlings.git
Cloning into '/home/ferris/code/rustlings'...
remote: Enumerating objects: 14038, done.
... more git output
Resolving deltas: 100% (8076/8076), done
```

or:

```
$ jclone git@git.example.com:my-department/backend/big-project.git

```

## Configuration precedence

Config values from the first matching variant for a given host, if any, come first. Any missing values are filled in from your base user config (the settings not in any variant) and then from jclone defaults.

## Choosing what jclone prints out

As noted above, you can choose what jclone prints to your terminal with the `output` option in your configuration file. Below are the possible values.

| Value                  | Clone progress | Git errors | Flavour text on errors | "🎉 Done!" |
|------------------------|----------------|------------|------------------------|------------|
| `"default"` or missing |       ✅       |     ✅     |           ✅           |     ✅     |
| `"git-only"`           |       ✅       |     ✅     |                        |            |
| `"no-git"`             |                |            |           ✅           |     ✅     |
| `"quiet"`              |                |            |                        |            |

Errors in jclone like config parsing issues and IO errors will always be printed.

# Planned features

- [x] Clone a repository
- [x] Clone to a default base directory
- [x] Configurable base directory
- [x] Toggleable host directory (e.g. `.../github.com/`)
- [x] Toggleable full path
- [x] Per-host configuration by exact match on host
- [ ] Per-host configuration by pattern matching host
- [x] Stream git output so user can see clone progress
- [x] Quiet option to suppress all output
- [x] Git-only output option so only git output is printed
- [x] Check repo exists before creating any directories
- [ ] ~~Tidy up any created directories on error~~
- [ ] Pass git args to command
- [ ] Configure default git args
- [ ] Check several locations for config file
- [x] deb/rpm packages
- [x] Automated releases
- [ ] Windows support lol
- [x] Exit status codes
- [x] Tidier CLI (e.g. missing argument shouldn't panic)

## Changes needed to support Windows

Noting here any of the changes I might need to make when I look at supporting Windows.

- Specific references to environment, specifically `$HOME` - might be better to use the dir or dirs crate
- The `target_dir` method refers to Windows-specific `Prefix` that might need to be handled
