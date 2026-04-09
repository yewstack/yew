fn main() {
    if version_check::is_feature_flaggable().unwrap_or(false) {
        println!("cargo:rustc-cfg=yew_macro_nightly");
    }
}
