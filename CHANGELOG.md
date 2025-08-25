# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `output` config field with options `default`, `git-only`, `no-git` and `quiet`

### Changed

- Send `git clone` progress output to stdout by default instead of "Cloning into..." message
- Leave directory creation to `git clone`

## [0.5.0] - 2025-08-19

### Added

- `.deb` and `.rpm` package build targets

### Changed

- Strip binary of all debug symbols (will look at debug and source packages later)
- Now licensed under BSD-3-Clause

## Releases before changelog

* [0.4.1] *2025-08-05* - Check repo exists before creating any directories and remove `lib` build target
* [0.4.0] *2025-08-05* - Adds support for `[[variants]]`: each has `host` field to match on repo host and overrides any config values for that host
* [0.3.2] *2025-07-31* - Adds `use_full_path` to config to toggle using the full path or just the repo name as directories
* [0.3.1] *2025-07-31* - Adds `use_host_dir` to config to toggle including repo host as directory in clone path
* [0.3.0] *2025-07-24* - Base dir (i.e. what `~/src` was in 0.2.0) now optionally set as `base_dir` in `~/.jclone.toml`
* [0.2.0] *2025-05-20* - Now accepts single argument (repo URL) and clones to folder under `~/src` based on host and path
* [0.1.0] *2025-05-20* - New project, forwards all arguments to `git clone` (licence BSD-3-Clause-No-Military)

[Unreleased]: https://github.com/jacobwalkr/jclone/compare/v0.5.0..main
[0.5.0]: https://github.com/jacobwalkr/jclone/compare/v0.4.1..v0.5.0
[0.4.1]: https://github.com/jacobwalkr/jclone/compare/v0.4.0..v0.4.1
[0.4.0]: https://github.com/jacobwalkr/jclone/compare/v0.3.2..v0.4.0
[0.3.2]: https://github.com/jacobwalkr/jclone/compare/v0.3.1..v0.3.2
[0.3.1]: https://github.com/jacobwalkr/jclone/compare/v0.3.0..v0.3.1
[0.3.0]: https://github.com/jacobwalkr/jclone/compare/v0.2.0..v0.3.0
[0.2.0]: https://github.com/jacobwalkr/jclone/compare/v0.1.0..v0.2.0
[0.1.0]: https://github.com/jacobwalkr/jclone/releases/tag/v0.1.0