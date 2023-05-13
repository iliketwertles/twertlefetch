use std::env;

mod pkgs;
mod extra;
mod basic;
pub use crate::pkgs::pack;
pub use crate::extra::extra::pickle_fetch;
pub use crate::basic::basic::normal_ascii;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        normal_ascii();
    } else if args.len() >= 2 {
        match args[1].as_str() {
            "-p" => {
                pickle_fetch();
            }
            _ => {
                normal_ascii()
            }
        }
    }
}
