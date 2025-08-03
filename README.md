Jacob's Clone Tool (jclone)
===========================

https://naff.software

A command line tool for cloning and organising git repositories, written in Rust.

What can I say? I like to keep my repos organised.

# Usage

By default, clones the given repository into `$HOME/src/<path/to/repo>`. For example:

```
$ jclone git@github.com:rust-lang/rustlings.git
```

Gives you:

```
Cloning repository to "/home/jacob/src/github.com/rust-lang/rustlings"...
🎉 Done!

```

# Configuration

You can configure jclone by creating a configuration file at `$HOME/.jclone.toml`:

```toml
base_dir = "/your/base/path" # defaults to "$HOME/src"
use_host_dir: false # default: true
use_full_path: false # default: true
```

Yields:

```
$ jclone git@github.com:rust-lang/rustlings.git
Cloning repository to "/your/base/path/rustlings"...
🎉 Done!

```

# Planned features

- [x] Clone a repository
- [x] Clone to a default base directory
- [x] Configurable base directory
- [x] Toggleable host directory (e.g. `.../github.com/`)
- [x] Toggleable full path
- [ ] Per-host configuration by exact match on host
- [ ] Per-host configuration by pattern matching host
- [ ] Stream git output so user can see clone progress
- [ ] Quiet option to suppress all output
- [ ] Git-only output option so only git output is printed
- [ ] Check repo exists before creating any directories
- [ ] Tidy up any created directories on error
- [ ] Pass git args to command
- [ ] Configure default git args
- [ ] Check several locations for config file
- [ ] deb/rpm packages
- [ ] Automated releases
- [ ] Windows support lol

## Changes needed to support Windows

Noting here any of the changes I might need to make when I look at supporting Windows.

- Specific references to environment, specifically `$HOME` - might be better to use the dir or dirs crate
- The `target_dir` method refers to Windows-specific `Prefix` that might need to be handled
