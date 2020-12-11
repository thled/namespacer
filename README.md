# namespacer

[![Version][version-badge]][changelog]
[![MIT License][license-badge]][license]
[![Rust][rust-badge]][rust]

namespacer is a command line tool to automatically fix wrong `namespace` declarations in PHP files
according to PSR-4 and PSR-12.

## Requirements

todo

### Development

- [Docker][docker]
- [Docker-Compose][docker-compose]

## Installation

todo

## Usage

`$ ./namespacer FILE BASE_DIR`

todo

## Developing

todo

### Linting

`$ cargo clippy`

### Testing

todo

#### Watcher

`$ cargo watch -cx test -i tests/data`

## Contribute

Please do contribute! Issues and pull requests are welcome.

[version-badge]: https://img.shields.io/badge/version-0.1.0-blue.svg
[changelog]: ./CHANGELOG.md
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: ./LICENSE
[rust-badge]: https://img.shields.io/badge/Rust-1.48-blue.svg
[rust]: https://blog.rust-lang.org/2020/11/19/Rust-1.48.html
[docker]: https://docs.docker.com/install/
[docker-compose]: https://docs.docker.com/compose/install/
