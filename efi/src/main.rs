#![no_main]
#![no_std]

use r_efi::efi;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

#[export_name = "efi_main"]
pub extern fn main(_h: efi::Handle, _st: *mut efi::SystemTable) -> efi::Status {
    let stdout = unsafe {(*_st).con_out};
    let string = "Hello UEFI!".as_bytes();
    let mut buf = [0u16; 32];

    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }

    unsafe {
        ((*stdout).reset)(stdout, efi::Boolean::FALSE);
        ((*stdout).output_string)(stdout, buf.as_mut_ptr());
    }

    for i in 0..9999999{

    }

    efi::Status::SUCCESS
}
