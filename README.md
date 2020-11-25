# Linux-0.11-Rust [WIP]

## Usage

Install all dependences

```sh
rustup target add i686-unknown-linux-gnu
cargo install cargo-xbuild
```

Build and run OS (WIP)

```sh
make
make qemu
```

## TODO List

- [x] Run `bootsect.s` and `setup.s`
- [x] Run `main.rs` with `head.s`
- [ ] Add `trap_init` function
