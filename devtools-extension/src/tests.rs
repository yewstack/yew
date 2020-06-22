use super::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg(not(feature = "logic_test"))]
pub fn test_layout() {
    let document = yew::utils::document();
    let element = document.create_element("div").unwrap();
    yew::initialize();
    App::<DevToolsExtension>::new().mount(element.clone());
    assert_eq!(
        element.first_child().unwrap().text_content().unwrap(),
        "Yew DevTools"
    );
}

#[test]
#[cfg(feature = "logic_test")]
fn test_add_root_node() {
    let mut extension = DevToolsExtension::new();
    let message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(serde_json::to_string(&message).expect("Couldn't encode component."));
    assert_eq!(extension.component_tree.iter().count(), 1);
    assert!(extension.root_node.is_some());
}

#[test]
#[cfg(feature = "logic_test")]
fn test_add_child_nodes() {
    let mut extension = DevToolsExtension::new();
    let message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(serde_json::to_string(&message).expect("Couldn't encode component."));
    assert_eq!(extension.component_tree.iter().count(), 1);
    assert!(extension.root_node.is_some());
    let first_child_message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "Header".to_string(),
            selector: Some("body/h1".to_string()),
        }),
    };
    extension.handle_message(
        serde_json::to_string(&first_child_message).expect("Couldn't encode component."),
    );
    assert_eq!(
        extension
            .root_node
            .unwrap()
            .children(&extension.component_tree)
            .count(),
        1
    );
    let first_child_child_message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "Paragraph".to_string(),
            selector: Some("body/h1/p".to_string()),
        }),
    };
    extension.handle_message(
        serde_json::to_string(&first_child_child_message).expect("Couldn't encode component."),
    );
    assert_eq!(
        extension
            .root_node
            .unwrap()
            .children(&extension.component_tree)
            .next()
            .unwrap()
            .children(&extension.component_tree)
            .count(),
        1
    );
}

#[test]
#[cfg(feature = "logic_test")]
fn test_add_sibling_components() {
    let mut extension = DevToolsExtension::new();
    let root_component_message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(
        serde_json::to_string(&root_component_message).expect("Couldn't encode component."),
    );
    assert_eq!(extension.component_tree.iter().count(), 1);
    assert!(extension.root_node.is_some());

    let first_child_message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "Header".to_string(),
            selector: Some("body/h1".to_string()),
        }),
    };
    extension.handle_message(
        serde_json::to_string(&first_child_message).expect("Couldn't encode component."),
    );
    assert_eq!(
        extension
            .root_node
            .unwrap()
            .children(&extension.component_tree)
            .count(),
        1
    );

    let second_child_message = ComponentMessage {
        event: ComponentEvent::Created,
        data: Some(DebugComponent {
            name: "Header".to_string(),
            selector: Some("body/p".to_string()),
        }),
    };
    extension.handle_message(
        serde_json::to_string(&second_child_message).expect("Couldn't encode component."),
    );
    assert_eq!(
        extension
            .root_node
            .unwrap()
            .children(&extension.component_tree)
            .count(),
        2
    );
}

#[test]
#[cfg(feature = "logic_test")]
fn test_mount_component() {}

#[test]
#[cfg(feature = "logic_test")]
fn test_unmount_component() {}

#[test]
#[cfg(feature = "logic_test")]
fn test_delete_component() {}

#[test]
#[cfg(feature = "logic_test")]
fn test_render_component_tree() {}
