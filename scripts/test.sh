#!/bin/bash

# to print the color in terminal
INFO="\033[0;33m"
ERROR="\033[0;31m"
RIGHT="\033[0;32m"
END="\033[0m"

zodiac="./target/debug/zodiac"
res_dir="./res"

cargo build
mkdir -p $res_dir

check() {
    expected="$1"
    input="$2"
    arch="$3"
    ${zodiac} "$input" ${arch} > ${res_dir}/tmp.s
    if [ ${arch} == "x" ] 
        then
            gcc -static -o ${res_dir}/tmp ${res_dir}/tmp.s
        else
            riscv64-unknown-elf-gcc -o ${res_dir}/tmp ${res_dir}/tmp.s
    fi

    ${res_dir}/tmp
    actual="$?"

    if [ "$actual" != "$expected" ]; then
        echo "$input expected, but got $actual"
        exit 1
    else
        echo "$input => $actual"
    fi
}

unit_test() {
    arch="$1"
    check 0 0 ${arch}
    check 42 42 ${arch}
    check 21 '5+20-4' ${arch}
    check 41 ' 12 + 34 -5 ' ${arch}
    check 36 '1+2+3+4+5+6+7+8' ${arch}
    check 153 '1+2+3+4+5+6+7+8+9+10+11+12+13+14+15+16+17' ${arch}
    check 10 '2*3+4' ${arch}
    check 14 '2+3*4' ${arch}
    check 26 '2*3+4*5' ${arch}
    check 5 '50/10' ${arch}
    check 9 '6*3/2' ${arch}
    check 6 '1+10*4/8' ${arch}
    check 103 '25*4+24/8' ${arch}
    # echo OK
}

help_info() {
    echo -e "${RIGHT}Usage:${END}"
    echo -e "${RIGHT}test.sh [-x] [-r]${END}"
    echo -e "${RIGHT}Description:${END}"
    echo -e "${RIGHT}-x: test x86/64 backend.${END}"
    echo -e "${RIGHT}-r: test riscv backend.${END}"
    exit 0
}

while getopts 'hxr' OPT; do
    case $OPT in
        h) help_info;;
        x) unit_test x;;
        r) unit_test r;;
        ?) echo -e "${ERROR}invalid parameters!!!${END}"
        help_info;;
    esac
done