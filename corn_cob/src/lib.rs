#![feature(asm)]
#![feature(type_ascription)]
#![feature(const_fn)]

#[macro_use]
extern crate pest_derive;
extern crate pest;

#[macro_use]
extern crate lazy_static;

pub mod parser;
pub mod context;
pub mod preprocessor;
pub mod base_macro;
// pub mod gen2lyzh4ir;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
