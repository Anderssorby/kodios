# KodiOS

A fun little OS written in Rust.

## Building

QEMU is used for testing the kernel.

### With Nix

Nix can install all necessary dependencies easily. Install nix, activate flakes and run:

```
nix develop
cargo bootimage
```

### Without nix

Install bootimage
```
cargo install bootimage
```

Build for x86_64
```
cargo bootimage
```

## Running

### Simple

Run default:

```
cargo run
```


Using QEMU for x86_64 arch.

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-kodios/debug/bootimage-kodios.bin
```
