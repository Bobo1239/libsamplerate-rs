#![allow(non_camel_case_types, non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linking_works() {
        unsafe {
            let mut error = 0;
            let _state = src_new(SRC_SINC_BEST_QUALITY, 2, &mut error);
        }
    }
}
