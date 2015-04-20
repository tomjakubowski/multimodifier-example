#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(synex)]

struct Blah;

impl Blah {
    #[dare]
    fn blahhh(&self) {}
}


fn main() {
}
