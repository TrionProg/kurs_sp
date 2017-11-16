extern crate rand;


mod signal;
use signal::gen_signal;

mod float;
mod fixed;

pub const N:usize=8;
pub const LEN:usize=1024;

fn main() {
    gen_signal();
}
