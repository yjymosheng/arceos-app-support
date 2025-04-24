#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() {
    let c = 's';
    // let s = "hello world";
    putchar(c);
    print_registers();
    // puts(s);
    // hello();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE: usize = 3;

fn putchar(c: char) {
    let c = c as u32;
    unsafe {
        core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    ra, t1
            ",
            abi_num = const SYS_PUTCHAR,
            in("a0") c,
        );
    }
}

fn puts(s: &str) {
    for c in s.chars() {
        let c = c as u32;
        unsafe {
            core::arch::asm!("
            li      t0, {abi_num}
            slli    t0, t0, 3
            add     t1, a7, t0
            ld      t1, (t1)
            jalr    ra , t1
            ",
                abi_num = const SYS_PUTCHAR,
                in("a0") c,
            );
        }
    }
}

fn hello() {
    unsafe {
        core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        add     t1, a7, t0
        ld      t1, (t1)
        jalr    ra, t1
        ",
            abi_num = const SYS_HELLO,
        );
    }
}


fn print_registers() {
    let mut val: usize;

    macro_rules! print_reg {
        ($name:literal, $reg:tt) => {{
            unsafe {
                core::arch::asm!(
                    concat!("mv {0}, ", $reg),
                    out(reg) val,
                );
            }
            puts($name);
            puts(": ");
            print_hex(val);
            putchar('\n');
        }};
    }

    print_reg!("ra", "ra");
    print_reg!("sp", "sp");
    print_reg!("gp", "gp");
    print_reg!("a0", "a0");
    print_reg!("a1", "a1");
    print_reg!("a2", "a2");
    print_reg!("a3", "a3");
    print_reg!("a4", "a4");
    print_reg!("a5", "a5");
    print_reg!("a6", "a6");
    print_reg!("a7", "a7");
}

fn print_hex(mut val: usize) {
    let mut buf = [0u8; 16];
    let hex = b"0123456789ABCDEF";

    for i in (0..16).rev() {
        buf[i] = hex[(val & 0xF) as usize];
        val >>= 4;
    }

    for &b in &buf {
        putchar(b as char);
    }
}
