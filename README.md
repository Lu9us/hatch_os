# HatchOS

Playing with QEMU and a minimal OS in Rust.

## Prerequisites

```
rustup update nightly
rustup override set nightly
rustup component add llvm-tools-preview
rustup component add rust-src
```

## Build/Run

```
cargo build
qemu-system-aarch64 -machine virt \
  -m 1024M \
  -cpu cortex-a53 \
  -nographic \
  -kernel target/arm/debug/hatch_os
```

## References/Sources

* https://os.phil-opp.com/minimal-rust-kernel/
* https://lowenware.com/blog/osdev/aarch64-bare-metal-program-in-rust/

