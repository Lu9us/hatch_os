# HatchOS

Playing with QEMU and a minimal OS in Rust.

## Prerequisites

```
rustup update nightly
rustup override set nightly
rustup component add llvm-tools-preview
rustup component add rust-src
cargo install bootimage
```

## Build/Run

```
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-hatch_os.bin
```

## References/Sources

* https://os.phil-opp.com/minimal-rust-kernel/
