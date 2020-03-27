mod ser;
mod de;
mod errors;

pub use self::errors::{Result, WXFError};
pub use self::ser::{WXFSerializer};

#[test]
fn main() {
    println!("Hello Wolfram!")
}
