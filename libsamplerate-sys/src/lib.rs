#![allow(non_camel_case_types, non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_int;

    #[test]
    fn linking_works() {
        unsafe {
            let mut error = 0 as c_int;
            let _state = src_new(SRC_SINC_BEST_QUALITY as c_int, 2, &mut error);
        }
    }
}
