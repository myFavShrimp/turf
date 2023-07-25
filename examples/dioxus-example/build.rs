fn main() {
    println!("cargo:rerun-if-changed=src/**/*.scss");
    println!("cargo:rerun-if-changed=variables/**/*.scss");
}
