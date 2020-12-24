#[path = "./lib.rs"]
mod lib;
use lib::*;

fn main() {
    let ptr = OptionBox::new(Some(123));
    let b = ptr.unwrap();
}
