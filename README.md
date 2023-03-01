# Raspberry Pi Servo Test

---

Japanese version is [here](README_JP.md).

---

## Install Rust on Raspberry Pi

Reference: [rust-lang](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Check your pin assign

```bash
pinout
```

## Build and Run

```bash
cargo run
```

If you want to set range of motion manually, run like below.

```bash
cargo run -- -h 1800 -l 1200
```

## Kill process

You can kill this process by `Ctrl+c`.