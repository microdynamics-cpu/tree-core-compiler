#!/bin/bash

zodiac="./target/debug/zodiac"
res_dir="./res"

check() {
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

check 0 0
check 42 42
check 21 '5+20-4'
check 41 ' 12 + 34 -5 '
check 36 '1+2+3+4+5+6+7+8'
check 153 '1+2+3+4+5+6+7+8+9+10+11+12+13+14+15+16+17'
check 10 '2*3+4'
check 14 '2+3*4'
check 26 '2*3+4*5'
check 5 '50/10'
check 9 '6*3/2'
check 6 '1+10*4/8'
check 103 '25*4+24/8'
echo OK
