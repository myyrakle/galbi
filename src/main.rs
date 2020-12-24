#[path = "./lib.rs"]
mod lib;
use lib::*;

fn main() {
    //let ptr = OptionBox::new(Some(123));
    //let b = ptr.as_ref().unwrap();
    let arc = ArcMutex::new(234);
    let foo = arc.lock().unwrap();
    println!("{}", foo);
}
