#!/bin/bash
echo "[Downloading]"
cd /usr/local/bin
wget https://developer.arm.com/-/media/Files/downloads/gnu-a/10.3-2021.07/binrel/gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf.tar.xz

echo "[Inflating]"
rm -rf gcc-aarch64
rm /usr/local/bin/aarch64-none-elf-nm     \
 /usr/local/bin/aarch64-none-elf-gcc      \
 /usr/local/bin/aarch64-none-elf-objcopy  \
 /usr/local/bin/aarch64-none-elf-objdump  \
 /usr/local/bin/aarch64-none-elf-readelf
tar -xf gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf.tar.xz
rm gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf.tar.xz

echo "[Installing]"
mkdir gcc-aarch64
mv gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf/* gcc-aarch64
ln -s /usr/local/bin/gcc-aarch64/bin/aarch64-none-elf-nm /usr/local/bin/aarch64-none-elf-nm
ln -s /usr/local/bin/gcc-aarch64/bin/aarch64-none-elf-gcc /usr/local/bin/aarch64-none-elf-gcc
ln -s /usr/local/bin/gcc-aarch64/bin/aarch64-none-elf-objcopy /usr/local/bin/aarch64-none-elf-objcopy
ln -s /usr/local/bin/gcc-aarch64/bin/aarch64-none-elf-objdump /usr/local/bin/aarch64-none-elf-objdump
ln -s /usr/local/bin/gcc-aarch64/bin/aarch64-none-elf-readelf /usr/local/bin/aarch64-none-elf-readelf

rm -rf gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf

rustup +nightly target add aarch64-unknown-none-softfloat