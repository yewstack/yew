#![no_implicit_prelude]

// Shadow primitives
#[allow(non_camel_case_types)]
pub struct bool;
#[allow(non_camel_case_types)]
pub struct char;
#[allow(non_camel_case_types)]
pub struct f32;
#[allow(non_camel_case_types)]
pub struct f64;
#[allow(non_camel_case_types)]
pub struct i128;
#[allow(non_camel_case_types)]
pub struct i16;
#[allow(non_camel_case_types)]
pub struct i32;
#[allow(non_camel_case_types)]
pub struct i64;
#[allow(non_camel_case_types)]
pub struct i8;
#[allow(non_camel_case_types)]
pub struct isize;
#[allow(non_camel_case_types)]
pub struct str;
#[allow(non_camel_case_types)]
pub struct u128;
#[allow(non_camel_case_types)]
pub struct u16;
#[allow(non_camel_case_types)]
pub struct u32;
#[allow(non_camel_case_types)]
pub struct u64;
#[allow(non_camel_case_types)]
pub struct u8;
#[allow(non_camel_case_types)]
pub struct usize;

fn main() {
    // Ensure Rust keywords can be used as element names.
    // See: #1771
    _ = ::yew::html! {
        <a class="btn btn-primary" href="https://example.org/">
            <svg class="bi" fill="currentColor">
                <use href="/bootstrap-icons.svg#wrench"/>
            </svg>
            <span>{"Go to example.org"}</span>
        </a>
    };

    // some general SVG
    _ = ::yew::html! {
        <svg width="149" height="147" viewBox="0 0 149 147" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M60.5776 13.8268L51.8673 42.6431L77.7475 37.331L60.5776 13.8268Z" fill="#DEB819"/>
            <path d="M108.361 94.9937L138.708 90.686L115.342 69.8642" stroke="black" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
            <g filter="url(#filter0_d)">
                <circle cx="75.3326" cy="73.4918" r="55" fill="#FDD630"/>
                <circle cx="75.3326" cy="73.4918" r="52.5" stroke="black" stroke-width="5"/>
            </g>
            <circle cx="71" cy="99" r="5" fill="white" fill-opacity="0.75" stroke="black" stroke-width="3"/>
            <defs>
                <filter id="filter0_d" x="16.3326" y="18.4918" width="118" height="118" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
                    <feGaussianBlur stdDeviation="2"/>
                    <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"/>
                </filter>
            </defs>
        </svg>
    };
}
