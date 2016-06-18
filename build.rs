fn main() {
    println!("cargo:rustc-flags=-l framework=CoreFoundation");
    println!("cargo:rustc-flags=-l framework=CoreGraphics");
}
