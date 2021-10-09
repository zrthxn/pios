# pios
### kernel project

## About
A barebones operating system kernel which should (at the very least)
be able to give you a terminal and file handling on a Armv8 (AArch64) based SBC.
For this, I'm using the Raspberry Pi 3b+.

**Why?** Just for fun. I like being this close to hardware and its a huge challenge
programming with Rust **without** a standard library and writing safe code.

### Goals


### What's done until now


## Development

### Setup
Dockerization for build tools was not set up for this because it was not a priority right now.
I do have a script that can sort of do the required setup.

1. Install build tools (aarch64-gcc toolchain) from Arm website
    ```bash
    chmod +x ./setup.sh
    sudo ./setup.sh
    ```
2. Install Make. I assume you'll know how to do this on your system.
3. Install Rust and cargo nightly. Also install the `aarch64-unknown-none-softfloat` target
    ```bash
    rustup +nightly target add aarch64-unknown-none-softfloat
    ```

If there are problems, Rust will tell you what to do.

### Building and running
If you have QEMU then this can be tested on that for now.
Just run
```bash
make qemu
```