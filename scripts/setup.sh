#!/bin/bash

sudo apt-get update
sudo apt-get install qemu-user
qemu-riscv64 --version

wget https://static.dev.sifive.com/dev-tools/freedom-tools/v2020.08/riscv64-unknown-elf-gcc-10.1.0-2020.08.2-x86_64-linux-ubuntu14.tar.gz
tar -xvf riscv64-unknown-elf-gcc-10.1.0-2020.08.2-x86_64-linux-ubuntu14.tar.gz
echo export PATH=$PATH:$(pwd)/riscv64-unknown-elf-gcc-10.1.0-2020.08.2-x86_64-linux-ubuntu14/bin > ~/.bashrc
source ~/.bashrc

riscv64-unknown-elf-gcc --version