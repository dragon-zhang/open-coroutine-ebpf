# open-coroutine

check article https://blog.csdn.net/dwh0403/article/details/127817919 

check https://github.com/aya-rs/aya/issues/324

## Prerequisites

1. Install bpf-linker: `cargo install bpf-linker`

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Build Userspace

```bash
cargo build
```

## Run

```bash
RUST_LOG=warn cargo xtask run -- --iface lo
```
