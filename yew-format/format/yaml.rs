//! Contains an implementation of YAML serialization format.

/// A representation of a YAML data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```
/// // Converts (lazy) data to a Yaml
///# use yew::format::Yaml;
///
///# fn dont_execute() {
///# let data: String = unimplemented!();
/// let dump = Yaml(&data);
///
/// // Converts YAML string to a data (lazy).
/// let Yaml(data) = dump;
///# }
/// ```
#[derive(Debug)]
pub struct Yaml<T>(pub T);

text_format!(Yaml based on serde_yaml);

binary_format!(Yaml based on serde_yaml);
