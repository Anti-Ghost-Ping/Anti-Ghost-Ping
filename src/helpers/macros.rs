#[macro_export]
macro_rules! unwrap {
    ($func:expr) => {
        match $func {
            Some(item) => item,
            None => return Ok(()),
        }
    };
}
