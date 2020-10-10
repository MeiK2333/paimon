extern crate libc;

#[link(name = "paimon")]
extern "C" {
    fn process();
}

fn main() {
    unsafe {
        process();
    }
}
