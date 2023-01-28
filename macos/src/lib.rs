#[cfg(target = "aarch64-apple-darwin")]
mod aarch64_apple_darwin;
#[cfg(target = "aarch64-apple-darwin")]
pub use aarch64_apple_darwin::*;

#[cfg(target = "x86_64-apple-darwin")]
mod x86_64_apple_darwin;
#[cfg(target = "x86_64-apple-darwin")]
pub use x86_64_apple_darwin::*;

#[cfg(not(target = "aarch64-apple-darwin"))]
#[cfg(not(target = "x86_64-apple-darwin"))]
// this must be covered by Cargo.toml but https://github.com/rust-lang/cargo/issues/1197
#[cfg(not(target = "aarch64-apple-ios"))]
#[cfg(not(target = "aarch64-apple-ios-sim"))]
compile_error!("prebuilt bindings are not supported for this target.");