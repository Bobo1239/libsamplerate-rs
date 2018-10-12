.PHONY: src/bindings.rs

src/bindings.rs:
	bindgen libsamplerate/src/samplerate.h -o $@
	sed "s/type _bindgen_ty_1 = u32;/type _bindgen_ty_1 = i32;/" -i src/bindings.rs