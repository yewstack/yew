#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[function_component]
fn SvgWithDropShadow() -> Html {
    html! {
        <svg id="test-svg">
            <defs>
                <filter id="glow">
                    <feDropShadow dx="0" dy="0" stdDeviation="10" flood-color="red"/>
                </filter>
            </defs>
            <rect width="100" height="100" filter="url(#glow)" />
        </svg>
    }
}

#[wasm_bindgen_test]
async fn svg_camel_case_elements_render() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create container
    let container = document.create_element("div").unwrap();
    container.set_id("test-container");
    body.append_child(&container).unwrap();

    // Mount component
    yew::Renderer::<SvgWithDropShadow>::with_root(container.clone().unchecked_into()).render();

    // Wait for render to complete
    yew::platform::time::sleep(std::time::Duration::from_millis(1000)).await;

    // Get the SVG element to verify it exists
    let svg_element = document
        .get_element_by_id("test-svg")
        .expect("SVG element should exist");

    // Get SVG bounds for pixel analysis
    let rect = svg_element.get_bounding_client_rect();

    // Use getDisplayMedia to capture the screen
    let navigator = window.navigator();
    let media_devices = navigator.media_devices().unwrap();

    // Create options for getDisplayMedia
    let constraints = web_sys::DisplayMediaStreamConstraints::new();
    // Set preferCurrentTab via reflection (Chrome-only feature)
    let _ = js_sys::Reflect::set(
        &constraints,
        &JsValue::from("preferCurrentTab"),
        &JsValue::from(true),
    );
    // Set video with frameRate
    let video_constraints = js_sys::Object::new();
    let _ = js_sys::Reflect::set(
        &video_constraints,
        &JsValue::from("frameRate"),
        &JsValue::from(30),
    );
    constraints.set_video(&video_constraints);

    // Try to get the display media stream
    let stream_promise = media_devices
        .get_display_media_with_constraints(&constraints)
        .unwrap();
    let stream_result = wasm_bindgen_futures::JsFuture::from(stream_promise).await;

    // Check if getDisplayMedia failed (likely Firefox without user interaction)
    let stream = match stream_result {
        Ok(stream) => stream.dyn_into::<web_sys::MediaStream>().unwrap(),
        Err(_) => {
            // We are likely in Firefox, there is no way of granting permission
            // for screen capture without user interaction in automated tests
            web_sys::console::log_1(
                &"getDisplayMedia failed - likely Firefox, skipping pixel test".into(),
            );
            return;
        }
    };

    // Create a video element to capture frames
    let video = document
        .create_element("video")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()
        .unwrap();
    video.set_autoplay(true);
    video.set_muted(true);
    js_sys::Reflect::set(&video, &JsValue::from("playsInline"), &JsValue::from(true));
    video.set_src_object(Some(&stream));

    // Add video to DOM (invisible but positioned at 0,0)
    let style = video.dyn_ref::<web_sys::HtmlElement>().unwrap().style();
    style.set_property("position", "fixed").unwrap();
    style.set_property("top", "0").unwrap();
    style.set_property("left", "0").unwrap();
    style.set_property("pointer-events", "none").unwrap();
    style.set_property("visibility", "hidden").unwrap();
    body.append_child(&video).unwrap();

    // Wait for video to start playing
    let play_promise = video.play().unwrap();
    wasm_bindgen_futures::JsFuture::from(play_promise)
        .await
        .ok();

    // Wait a bit for the video feed to stabilize
    yew::platform::time::sleep(std::time::Duration::from_millis(500)).await;

    // Get video track settings to know dimensions
    let tracks = stream.get_video_tracks();
    let track = tracks
        .get(0)
        .dyn_into::<web_sys::MediaStreamTrack>()
        .unwrap();
    let settings = track.get_settings();

    let width = settings.get_width().unwrap_or(1920) as u32;
    let height = settings.get_height().unwrap_or(1080) as u32;

    // Create canvas and draw the video frame
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width);
    canvas.set_height(height);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Draw the video frame to canvas
    ctx.draw_image_with_html_video_element(&video, 0.0, 0.0)
        .unwrap();

    // Stop the capture
    let tracks = stream.get_tracks();

    // Get image data for pixel analysis
    let image_data = ctx
        .get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap();
    let data = image_data.data();

    // Analyze pixels around the center where the rect should be
    let center_x = (rect.left() + rect.width() / 2.0) as i32;
    let center_y = (rect.top() + rect.height() / 2.0) as i32;

    let mut has_non_white_pixels = false;
    let mut sample_pixels = Vec::new();

    // Check a grid of pixels around the center
    for dy in (-60..=60).step_by(10) {
        for dx in (-60..=60).step_by(10) {
            let x = center_x + dx;
            let y = center_y + dy;

            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let idx = ((y * width as i32 + x) * 4) as usize;
                let r = data[idx];
                let g = data[idx + 1];
                let b = data[idx + 2];

                // Log some sample pixels for debugging
                if sample_pixels.len() < 10 {
                    sample_pixels.push(format!("({},{}): rgb({},{},{})", x, y, r, g, b));
                }

                // Check if pixel is not white (with tolerance)
                if r < 250 || g < 250 || b < 250 {
                    has_non_white_pixels = true;
                }
            }
        }
    }

    // Convert canvas to base64 for inspection
    let data_url = canvas.to_data_url().unwrap();

    // Log pixel analysis
    web_sys::console::log_1(&format!("Has non-white pixels: {}", has_non_white_pixels).into());
    web_sys::console::log_1(&format!("Sample pixels: {:?}", sample_pixels).into());
    web_sys::console::log_1(
        &format!(
            "Screenshot data URL (copy and paste in browser): {}",
            data_url
        )
        .into(),
    );

    // Also log the SVG bounds and center position
    web_sys::console::log_1(
        &format!(
            "SVG bounds: left={}, top={}, width={}, height={}",
            rect.left(),
            rect.top(),
            rect.width(),
            rect.height()
        )
        .into(),
    );
    web_sys::console::log_1(&format!("Center position: ({}, {})", center_x, center_y).into());
    web_sys::console::log_1(&format!("Canvas size: {}x{}", width, height).into());

    assert!(has_non_white_pixels, "Expected the rect to render");

    for i in 0..tracks.length() {
        let track = tracks.get(i);
        if !track.is_undefined() {
            track
                .dyn_into::<web_sys::MediaStreamTrack>()
                .unwrap()
                .stop();
        }
    }
}
