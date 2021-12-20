use strum::Display;
use strum::EnumString;

#[derive(Debug, Clone, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum YewPackage {
    Yew,
    YewAgent,
    YewRouter,
}

impl YewPackage {
    pub fn as_labels(&self) -> &'static [&'static str] {
        match self {
            YewPackage::Yew => &["A-yew", "A-yew-macro", "macro"],
            YewPackage::YewAgent => &["A-yew-agent"],
            YewPackage::YewRouter => &["A-yew-router", "A-yew-router-macro"],
        }
    }
}
