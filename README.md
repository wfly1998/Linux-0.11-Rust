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
- [ ] Finish trap (It's toooooooo difficult to add it module by module)
- [ ] Finish mem (Unknown bug will occur to make tty boom)

## Progress

Now my os can receive timer interrupt, but garbled will occur.

And unknown bug will occur...

When I add crate `mm` to `init/Cargo.toml`, os will not boot.

