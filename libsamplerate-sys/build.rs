extern crate cmake;

fn main() {
    println!(
        "cargo:rustc-link-search=native={}/lib",
        cmake::build("libsamplerate").display()
    );
    println!("cargo:rustc-link-lib=static=samplerate");
}
