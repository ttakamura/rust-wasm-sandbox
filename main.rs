#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Person {
        println!("create {}", name);
        return Person {
            name: name,
            age: age,
        };
    }
}
impl Drop for Person {
    fn drop(&mut self) {
        println!("destroyed {:?}", self);
    }
}

// =============================================

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

#[no_mangle]
pub extern "C" fn init_person(age: i32) -> *mut Person {
    unsafe {
        let tom = Box::new(Person::new("Tom".to_string(), age));
        let ptr: *mut Person = std::mem::transmute(tom);
        // let ptr = &foo as *const Person;
        print_person(ptr);
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn print_person(ptr: *const Person) {
    unsafe {
        let tom: &Person = &*ptr;
        println!("{:?}", tom);
    }
}
