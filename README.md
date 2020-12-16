# namespacer

[![Version][version-badge]][changelog]
[![MIT License][license-badge]][license]
[![Rust][rust-badge]][rust]

namespacer is a command line tool to automatically fix wrong `namespace` declarations in PHP files
according to PSR-4 and PSR-12.

## Requirements

### Development

- [Docker][docker]
- [Docker-Compose][docker-compose]

## Installation

1. Clone this repository: `$ git clone git@github.com:thled/namespacer.git`
1. Change to project directory: `$ cd namespacer`
1. Build and start the docker containers: `$ docker-compose up -d`
1. Build the app: `$ docker-compose exec app cargo build --release`
1. Copy binary: `$ cp app/target/release/namespacer TARGET`

## Usage

`$ ./namespacer FILE [ BASE_DIR [ VENDOR [ PREFIX ] ] ]`

### Examples

- `$ ./namespacer Controller/Login.php` => `namespace App\Controller;`
- `$ ./namespacer src/Controller/Login.php src` => `namespace App\Controller;`
- `$ ./namespacer src/Controller/Login.php src Acme` => `namespace Acme\Controller;`
- `$ ./namespacer src/Controller/Login.php src Acme Foo` => `namespace Acme\Foo\Controller;`

## Developing

### Linting

`$ cargo clippy`

### Testing

`$ cargo test`

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
