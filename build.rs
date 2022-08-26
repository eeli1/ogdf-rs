extern crate cmake;

fn main() {
    let dst = cmake::build("c_wrapper");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=c_wrapper");
    // println!("cargo:rustc-link-search=c_wrapper");
    // println!("cargo:rustc-link-lib=static=COIN");
    // println!("cargo:rustc-link-lib=static=OGDF");
}
