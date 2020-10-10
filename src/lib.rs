#[macro_use]
extern crate ctor;

#[no_mangle]
fn process() {
    println!("process");
}

#[ctor]
fn constructor() {
    println!("constructor");
}

#[dtor]
fn shutdown() {
    eprintln!("shutdown");
}
