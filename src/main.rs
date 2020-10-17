macro_rules! error_type {
    ($name:ident, $what:expr) => {
        #[derive(Debug, thiserror::Error)]
        // This doesn't work
        #[error(concat!("invalid ", $what))]
        // This does work
        // #[error("invalid")]
        struct Error;
    }
}

error_type!(Error, "foo");

fn main() {
    println!("error: {}", Error);
}
