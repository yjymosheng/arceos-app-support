#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#![feature(asm_const)]

#[cfg(feature = "axstd")]
use axstd::println;
#[cfg(feature = "axstd")]
use axstd::process::exit;
const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let mut apps_start = PLASH_START as *const u8;

    let load_app = |apps_start: &mut *const u8| {
        // 将 image前面4个字节的位置 添加一个数字,使用小端存储. 表示 这个app的长度信息
        let get_app_size = |start: *const u8| {
            let size = unsafe { core::slice::from_raw_parts(start, 4) };
            u32::from_le_bytes([size[0], size[1], size[2], size[3]]) as usize
        };
        // 帮助函数
        let load_app_help = |app_start: *const u8, app_size: usize| {
            let code = unsafe { core::slice::from_raw_parts(app_start, app_size) };
            println!("content: {:?}: ", code);
            code
        };
        
        let size = get_app_size(*apps_start);
        *apps_start = unsafe { apps_start.add(4) };
        let code = load_app_help(*apps_start, size);
        *apps_start = unsafe { apps_start.add(size) };
        (code, size)
    };
    println!("Load payload app 1");

    let (code_app_1, size_app_1) = load_app(&mut apps_start);

    // println!("debug : start {:x}", apps_start as usize);
    let (code_app_2, size_app_2) = load_app(&mut apps_start);

    println!("Load payload ok!");

    // app running aspace
    // SBI(0x80000000) -> App <- Kernel(0x80200000)
    // va_pa_offset: 0xffff_ffc0_0000_0000
    const RUN_START: usize = 0xffff_ffc0_8010_0000;

    // println!("debug : len {}",size_app_1);
    // println!("debug : len {}",size_app_2);

    //进行移动,并返回起始地址
    let move_code = |start: usize, offset: usize, size: usize, code: &[u8]| {
        let start = start + offset * 8;
        let run_code = unsafe { core::slice::from_raw_parts_mut(start as *mut u8, size) };
        run_code.copy_from_slice(code);
        println!("run code {:?}; address [{:?}]", run_code, run_code.as_ptr());
        start
    };

    let start_1 = move_code(RUN_START, 0, size_app_1, code_app_1);

    let start_2 = move_code(RUN_START, size_app_1, size_app_2, code_app_2);

    println!("Move payload ok!");

    println!("Execute app ...");



    register_abi(SYS_HELLO, abi_hello as usize);
    register_abi(SYS_PUTCHAR, abi_putchar as usize);
    register_abi(SYS_TERMINATE, abi_shutdown as usize);


	// println!("Execute app ...");
    let _arg0: u8 = b'A';
    let code : u8 =2; 

    // execute app
    unsafe { core::arch::asm!("
        li      t0, {abi_num}
        slli    t0, t0, 3
        la      t1, {abi_table}
        add     t1, t1, t0
        ld      t1, (t1)
        jalr    t1
        li      t2, {run_start}
        jalr    t2
        j       .",
        run_start = const RUN_START,
        abi_table = sym ABI_TABLE,
        //abi_num = const SYS_HELLO,
        abi_num = const SYS_TERMINATE,
        in("a0") code,
    )}

    // // execute app
    // unsafe {
    //     core::arch::asm!(
    //         // 保存 t1 和 t2 到栈中

    //         "
    //         jalr t2 

    //         jalr t1 

    //         j .",                 
    //         in("t2") start_1,
    //         in("t1") start_2,
    //     )
    // }
}


const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_TERMINATE :usize = 3; 

static mut ABI_TABLE: [usize; 16] = [0; 16];

fn register_abi(num: usize, handle: usize) {
    unsafe { ABI_TABLE[num] = handle; }
}

fn abi_hello() {
    println!("[ABI:Hello] Hello, Apps!");
}

fn abi_putchar(c: char) {
    println!("[ABI:Print] {c}");
}

fn abi_shutdown (code: u8) -> ! {
    exit(code.into() )
}