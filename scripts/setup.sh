#!/bin/bash

apt-get update
apt-get install qemu-user

wget https://static.dev.sifive.com/dev-tools/freedom-tools/v2020.08/riscv64-unknown-elf-gcc-10.1.0-2020.08.2-x86_64-linux-ubuntu14.tar.gz
tar -xvf riscv64-unknown-elf-gcc-10.1.0-2020.08.2-x86_64-linux-ubuntu14.tar.gz
cp riscv64-unknown-elf-gcc-10.1.0-2020.08.2-x86_64-linux-ubuntu14/* /usr/ -r