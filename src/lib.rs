extern crate libsamplerate_sys;

#[cfg(test)]
mod tests {
    use libsamplerate_sys::*;
    use std::os::raw::c_int;

    #[test]
    fn linking_works() {
        unsafe {
            let mut error = 0 as c_int;
            let _state = src_new(SRC_SINC_BEST_QUALITY as c_int, 2, &mut error);
        }
    }
}
