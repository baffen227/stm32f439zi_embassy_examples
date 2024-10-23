// TODO: check that this comment is also true for STM32
// Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
fn main() {
	println!("cargo:rustc-link-arg-bins=--nmagic");
	println!("cargo:rustc-link-arg-bins=-Tlink.x");
	println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
