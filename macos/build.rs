fn main() {
    let target = std::env::var("TARGET").unwrap();
    println!("cargo:rustc-cfg=target=\"{target}\"");
}