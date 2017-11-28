extern crate libsamplerate_sys;

#[cfg(test)]
mod tests {
    use libsamplerate_sys::*;

    #[test]
    fn linking_works() {
        unsafe {
            let mut error = 0;
            let _state = src_new(SRC_SINC_BEST_QUALITY, 2, &mut error);
        }
    }
}
