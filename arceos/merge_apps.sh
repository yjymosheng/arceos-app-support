#!/bin/bash

APP_1=payload/apps1.bin
# APP_2=payload/apps2.bin
OUT_APP=payload/apps.bin

TMP_APP=payload/tmp.bin
# 最开始使用cat, 但这样会导致cat 出巨大的 0 padding .
# version 1  修改了.bin文件的生成逻辑, 去掉了 大量冗余的 0 padding , 只保留了4(头文件数字,表示实际需要多大)+ 实际需要的大小

# if [[ ! -f $APP_1 ]] || [[ ! -f $APP_2 ]]; then
    # echo "应用程序文件不存在，执行其他生成脚本..."
    # 生成 APP_1
    cd ../app_1 && ./scripts/objdump.sh
    # 生成 APP_2
    # cd ../app_2 && ./scripts/objdump.sh

# fi

cd ../arceos || exit 

cat $APP_1  > $TMP_APP


# # 能否使用dd? 可以,但是这样会稍微复杂一些
# 需要生成一个至少32M的img

# qemu-system-riscv64: cfi.pflash01 device '/machine/virt.flash1' requires 33554432 bytes, pflash1 block backend provides 512 bytes

dd if=/dev/zero of=$OUT_APP bs=1M count=32 2>/dev/null

dd if=$TMP_APP of=$OUT_APP bs=1 conv=notrunc

rm $APP_1 