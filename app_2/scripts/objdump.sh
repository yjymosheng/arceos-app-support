#!/bin/bash

HEADER=header.bin
TARGE=app_2.bin
SOURCE=target/riscv64gc-unknown-none-elf/release/app_2
END_OUTPUT_FILE=apps2.bin


cargo build --target riscv64gc-unknown-none-elf --release

rust-objcopy --binary-architecture=riscv64 --strip-all -O binary $SOURCE ./$TARGE

SIZE=$(stat -c %s $TARGE)

BIN_SIZE=$((SIZE+4))

dd if=/dev/zero of=./$END_OUTPUT_FILE bs=1 count=$BIN_SIZE

echo "$SIZE" "$BIN_SIZE"

printf "%08x" "$SIZE" | tac -rs .. | xxd -r -p > $HEADER
dd if=$HEADER of=$END_OUTPUT_FILE conv=notrunc bs=1

dd if=./$TARGE of=./$END_OUTPUT_FILE  bs=1 seek=4 conv=notrunc

mkdir -p ../arceos/payload
mv ./$END_OUTPUT_FILE ../arceos/payload/$END_OUTPUT_FILE