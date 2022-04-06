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
    else
        echo "$input => $actual"
    fi
}

cargo build
mkdir -p $res_dir

try 0 0
try 42 42
try 21 '5+20-4'
try 41 ' 12 + 34 -5 '
try 36 '1+2+3+4+5+6+7+8'
try 153 '1+2+3+4+5+6+7+8+9+10+11+12+13+14+15+16+17'

echo OK
