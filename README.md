# open-coroutine

## Prerequisites

1. install ebpf deps
check https://github.com/xdp-project/xdp-tutorial/blob/master/setup_dependencies.org

2. build llvm-project in local
check https://github.com/aya-rs/aya/issues/324

```bash
LLVM_SYS_160_PREFIX=/home/parallels/Desktop/projects/llvm-project/build cargo install bpf-linker --no-default-features
```

3. build-ebpf and run
check article https://blog.csdn.net/dwh0403/article/details/127817919

install io_uring syscalls
```bash
sudo dnf install liburing-devel
```

## Build eBPF

```bash
cargo xtask build-ebpf
```

To perform a release build you can use the `--release` flag.
You may also change the target architecture with the `--target` flag.

## Run Userspace program

```bash
RUST_LOG=info cargo xtask run -- --iface enp0s5
```
