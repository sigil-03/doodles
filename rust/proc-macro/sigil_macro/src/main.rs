use hello_macro::HelloMacro;
use hello_macro_derive::*;

use fill_from::Fill;
use fill_from_macro::*;

struct Store {
    item_1: u8,
    item_2: String,
    item_3: String,
    item_4: u8,
}

#[derive(Fill)]
struct Output {
    pub item_1: String,
    pub item_2: String,
    pub item_3: u8,
}

impl Output {
    pub fn new() -> Self {
        Self {
            item_1: String::from("Hello"),
            item_2: String::from("Output"),
            item_3: 3,
        }
    }
}

#[derive(HelloMacro)]
struct Sigil;

fn main() {
    Sigil::hello_macro();
    let o = Output::generate::<String>(String::from("test"));
    println!("{}", o.item_1)
}
