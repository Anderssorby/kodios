# KodiOS

A fun little OS written in Rust.

## Building

Install bootimage
```
cargo install bootimage
```

Build for x86_64
```
cargo bootimage
```

## Running

Using QEMU for x86_64 arch.

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kodios/debug/bootimage-kodios.bin
```
