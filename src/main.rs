mod ffi;
mod poll;


fn main() {
    println!("Hello, world!");
    let message = String::from("This is my first run for rust async book");
    syscall(message);
}

#[cfg(target_os="linux")]
#[inline(never)]
fn syscall(message: String) {
    use std::arch::asm;

    let msg_ptr = message.as_ptr();
    let len = message.len();

    println!("{:?}", msg_ptr);

    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",
            in("rsi") msg_ptr,
            in("rdx") len,
            out("rax") _,
            out("rdi") _,
            lateout("rsi") _,
            lateout("rdx") _,
            );
    }
}
