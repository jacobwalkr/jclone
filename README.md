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

You can optionally configure the base path by creating a configuration file at `$HOME/.jclone.toml`:

```toml
base_dir = "/your/base/path"
```

Yields:

```
$ jclone git@github.com:rust-lang/rustlings.git
Cloning repository to "/your/base/path/github.com/rust-lang/rustlings"...
🎉 Done!

```

# Planned features

- [x] Clone a repository
- [x] Clone to a default base directory
- [x] Configurable base directory
- [ ] Toggleable host directory (e.g. `.../github.com/`)
- [ ] Toggleable full path
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
