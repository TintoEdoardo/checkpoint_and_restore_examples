#![no_std]

unsafe extern  "C" {
    unsafe fn print(iter: i32);
}

#[unsafe(no_mangle)]
pub extern "C" fn fibonacci(iter: i32) {

    let mut f_0 : i32 = 0;
    let mut f_1 : i32 = 1;
    let mut f_2 : i32; 

    for _i in 0..iter {

        f_2 = f_0 + f_1;

        f_0 = f_1;
        f_1 = f_2;

        unsafe { print(f_2); }

    }

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
