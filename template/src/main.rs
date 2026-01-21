mod a;

pub use algo::{inp, outp};
pub use algo::io::{reader, writer};
pub use std::io::Write;
pub type usz = usize;

fn main() {
    a::solve();
}