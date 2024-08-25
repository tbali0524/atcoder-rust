# AtCoder solutions in Rust by TBali

![rust v1.70](https://shields.io/badge/rust-1.70-blue?logo=rust)
![build](https://img.shields.io/github/actions/workflow/status/tbali0524/atcoder-rust/qa.yml)
![license](https://img.shields.io/github/license/tbali0524/atcoder-rust)

* [AtCoder website](https://atcoder.jp/)
* My AtCoder username: `TBali`
* Link to this repo on [GitHub](https://github.com/tbali0524/atcoder-rust)

## Usage

```sh
# -- setup
# install Rust: https://www.rust-lang.org/tools/install
rustup update stable
cargo version
cargo install
# -- lint
cargo fmt
cargo clippy
# -- test
cargo test
# -- run
cargo build --release --bin PUZZLE_ID
target/release/PUZZLE_ID.exe
# -- shortcut qa
./qa.bat
# -- cleanup
cargo clean
```
