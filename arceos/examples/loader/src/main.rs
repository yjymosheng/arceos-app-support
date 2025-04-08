#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

const PLASH_START: usize = 0xffff_ffc0_2200_0000;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    let apps_start = PLASH_START as *const u8;

    // 将 image前面4个字节的位置 添加一个数字,使用小端存储. 表示 这个app的长度信息
    let get_app_size = |start : *const u8 |{
        let size = unsafe { core::slice::from_raw_parts(apps_start, 4) };
        u32::from_le_bytes([size[0], size[1], size[2], size[3]]) as usize
    };



    let apps_size = get_app_size(apps_start);

    println!("Load payload ...");

    let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size) };
    println!("content: {:?}: ", code);

    println!("Load payload ok!");
}