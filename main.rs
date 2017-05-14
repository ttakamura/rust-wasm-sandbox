fn main() {
    println!("Hello World");
}

#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn hello(_len: i32, ptr: *const f32) {
    let len = _len as usize;
    let slice: &[f32] = unsafe { std::slice::from_raw_parts(ptr, len) };
    for n in slice {
        println!("{}", n);
    }
}
