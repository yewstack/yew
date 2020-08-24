use yew_config::{YewConfig};

#[no_mangle]
pub fn yew_config() -> YewConfig {
    YewConfig::new(
        "minimal".to_string(),
    )
}