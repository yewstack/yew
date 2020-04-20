fn main() {
    #[cfg(feature = "webgl_stdweb")]
    yew::start_app::<webgl::Model>();
}
