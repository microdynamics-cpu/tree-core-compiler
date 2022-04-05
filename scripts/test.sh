#!/bin/bash

zodiac="./target/debug/zodiac"
res_dir="./res"

try() {
    expected="$1"
    input="$2"

    ${zodiac} "$input" > ${res_dir}/tmp.s
    gcc -static -o ${res_dir}/tmp ${res_dir}/tmp.s
    ${res_dir}/tmp
    actual="$?"

    if [ "$actual" != "$expected" ]; then
        echo "$input expected, but got $actual"
        exit 1
    fi
}

cargo build
mkdir -p $res_dir

try 0 0
try 42 42

echo OK
