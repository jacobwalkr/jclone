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
