#[cfg(target = "aarch64-apple-ios")]
mod aarch64_apple_ios;
#[cfg(target = "aarch64-apple-ios")]
pub use aarch64_apple_ios::*;

#[cfg(not(target = "aarch64-apple-ios"))]
#[cfg(not(target = "aarch64-apple-ios-sim"))]
// this must be covered by Cargo.toml but https://github.com/rust-lang/cargo/issues/1197
#[cfg(not(target = "aarch64-apple-darwin"))]
#[cfg(not(target = "x86_64-apple-darwin"))]
compile_error!("prebuilt bindings are not supported for this target.");