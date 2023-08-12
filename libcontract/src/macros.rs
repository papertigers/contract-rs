#[macro_export]
macro_rules! libcall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { ::contract_sys::$fn($($arg, )*) };
        if res == 0 {
            Ok(res)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }};
}
