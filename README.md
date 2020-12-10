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
- [ ] Finish process (task)
- [ ] Finish syscall
- [ ] Finish fs

## Progress

Now my os can receive timer interrupt, but garbled will occur.

And `mem_init` will `panic` when whole function finished ant run `ret` instruction.

