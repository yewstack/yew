use super::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

fn construct_root_node(extension: &mut DevToolsExtension) {
    extension.handle_message(
        serde_json::to_string(&ComponentMessage {
            event: ComponentEvent::Created,
            data: Some(DebugComponent {
                name: "App".to_string(),
                selector: Some("body".to_string()),
            }),
        })
        .unwrap(),
    );
}

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
    construct_root_node(&mut extension);
    assert_eq!(extension.component_tree.iter().count(), 1);
    assert!(extension.root_node.is_some());
}

#[test]
#[cfg(feature = "logic_test")]
fn test_add_child_nodes() {
    let mut extension = DevToolsExtension::new();
    construct_root_node(&mut extension);
    assert_eq!(extension.component_tree.iter().count(), 1);
    assert!(extension.root_node.is_some());
    assert_eq!(
        extension
            .component_tree
            .get(extension.root_node.unwrap())
            .unwrap()
            .get()
            .selector,
        "body"
    );
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
    construct_root_node(&mut extension);
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
fn test_mount_component() {
    let mut extension = DevToolsExtension::new();
    construct_root_node(&mut extension);
    let mount_root_node_message = ComponentMessage {
        event: ComponentEvent::Mounted,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(serde_json::to_string(&mount_root_node_message).unwrap());
    assert_eq!(
        extension
            .component_tree
            .get(extension.root_node.unwrap())
            .unwrap()
            .get()
            .is_in_dom,
        true
    );
}

#[test]
#[cfg(feature = "logic_test")]
fn test_unmount_component() {
    let mut extension = DevToolsExtension::new();
    construct_root_node(&mut extension);
    let mount_root_node_message = ComponentMessage {
        event: ComponentEvent::Mounted,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(serde_json::to_string(&mount_root_node_message).unwrap());
    assert_eq!(
        extension
            .component_tree
            .get(extension.root_node.unwrap())
            .unwrap()
            .get()
            .is_in_dom,
        true
    );
    let mount_root_node_message = ComponentMessage {
        event: ComponentEvent::Unmounted,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(serde_json::to_string(&mount_root_node_message).unwrap());
    assert_eq!(
        extension
            .component_tree
            .get(extension.root_node.unwrap())
            .unwrap()
            .get()
            .is_in_dom,
        false
    );
}

#[test]
#[cfg(feature = "logic_test")]
fn test_delete_component() {
    let mut extension = DevToolsExtension::new();
    construct_root_node(&mut extension);
    let destroy_root_node_message = ComponentMessage {
        event: ComponentEvent::Destroyed,
        data: Some(DebugComponent {
            name: "App".to_string(),
            selector: Some("body".to_string()),
        }),
    };
    extension.handle_message(serde_json::to_string(&destroy_root_node_message).unwrap());
    assert!(extension.root_node.is_none());
    assert_eq!(
        extension.component_tree.iter().next().unwrap().is_removed(),
        true
    );
}

#[test]
fn test_delete_nested_component() {
    let mut extension = DevToolsExtension::new();
    construct_root_node(&mut extension);
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
    let remove_first_child_message = ComponentMessage {
        event: ComponentEvent::Destroyed,
        data: Some(DebugComponent {
            name: "Header".to_string(),
            selector: Some("body/h1".to_string()),
        }),
    };
    extension.handle_message(
        serde_json::to_string(&remove_first_child_message).expect("Couldn't encode component."),
    );
    assert_eq!(
        extension.component_tree.iter().nth(1).unwrap().is_removed(),
        true
    );
}

#[test]
#[cfg(feature = "logic_test")]
fn test_render_component_tree() {
    let mut extension = DevToolsExtension::new();
    construct_root_node(&mut extension);
    assert_eq!(extension.component_tree.iter().count(), 1);
    assert!(extension.root_node.is_some());
    assert_eq!(
        extension
            .component_tree
            .get(extension.root_node.unwrap())
            .unwrap()
            .get()
            .selector,
        "body"
    );
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
    let rendered_tree = match extension.render_component_tree(extension.root_node.unwrap()) {
        yew::virtual_dom::VNode::VTag(tag) => tag,
        _ => panic!("Expected a vtag."),
    };
    assert_eq!(rendered_tree.tag(), "div");
    assert_eq!(
        rendered_tree.attributes.get("class"),
        Some(&"node".to_string())
    );
    let first_child = match rendered_tree.children.children.get(0).unwrap() {
        yew::virtual_dom::VNode::VTag(tag) => tag,
        _ => panic!("Expected a vtag."),
    };
    assert_eq!(first_child.tag(), "div");
    assert_eq!(
        first_child.attributes.get("class"),
        Some(&"parent".to_string())
    );
    let h3_text = match match match first_child.children.get(0).unwrap() {
        yew::virtual_dom::VNode::VList(tag) => tag,
        _ => panic!(
            "Expected a vtag, found a {:?}",
            first_child.children.get(0).unwrap()
        ),
    }
    .children
    .get(0)
    .unwrap()
    {
        yew::virtual_dom::VNode::VTag(tag) => tag.children.get(0).unwrap(),
        _ => panic!("Expected a vtag."),
    } {
        yew::virtual_dom::VNode::VText(text) => text,
        _ => panic!("Expected vtext"),
    };
    assert_eq!(h3_text.text, "App");
}
