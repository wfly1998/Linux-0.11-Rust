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

- [x] Run bootloader (`bootsect.s` and `setup.s`)
- [x] Run `main.rs` with `head.s`
- [x] Finish tty (console + serial)
- [ ] Finish trap
- [ ] Finish mem

