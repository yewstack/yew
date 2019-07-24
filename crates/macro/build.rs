extern crate autocfg;

pub fn main() {
    if autocfg::new().probe_rustc_version(1, 36) {
        println!("cargo:rustc-cfg=has_maybe_uninit");
    }
}
