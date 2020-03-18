mod ser;
mod de;
mod errors;

pub use self::errors::{Result, WXFError};

#[test]
fn main() {
    println!("Hello Wolfram!")
}
