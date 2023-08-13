use std::env;
mod basic;
mod extra;
pub mod gpu;
mod pkgs;
pub use crate::basic::basic::normal_ascii;
pub use crate::extra::extra::pickle_fetch;
pub use crate::pkgs::pack;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        normal_ascii();
    } else if args.len() >= 2 {
        match args[1].as_str() {
            "-p" => {
                pickle_fetch();
            }
            _ => normal_ascii(),
        }
    }
}
