use std::io::{self, Read};
fn main() {
    for b in io::stdin().bytes() {
        let c = b.unwarp();
        print!("{}", c)
    }
}
