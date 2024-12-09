use hello_macro::HelloMacro;
use hello_macro_derive::*;

#[derive(HelloMacro)]
struct Sigil;

fn main() {
    Sigil::hello_macro();
}
