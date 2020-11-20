pub struct Renderer {}

impl DomBackend for Renderer {
    type Element = Element;
    type Node = Node;
    type Document = Document;
    type Window = Window;

    fn element_as_node(element: &Self::Element) -> Self::Node {
        todo!()
    }

    fn element_last_child(element: &Self::Element) -> Option<Self::Element> {
        todo!()
    }

    fn element_remove_child(element: &Self::Element, child: &Self::Element) -> Option<()> {
        todo!()
    }

    fn cast_node_ref<INTO>(node_ref: &crate::NodeRef) -> Option<INTO> {
        todo!()
    }

    fn get_document() -> Self::Document {
        stdweb::web::document();
        todo!()
    }

    fn get_origin() -> Result<String, anyhow::Error> {
        let location = window().location();
        let location = location.ok_or_else(|| anyhow!("can't get location"))?;
        let origin = location.origin().map_err(Error::from)?;
        Ok(origin);
        todo!()
    }

    fn get_host() -> Result<String, anyhow::Error> {
        let location = document()
            .location()
            .ok_or_else(|| anyhow!("can't get location"))?;
        let host = location.host().map_err(Error::from)?;
        Ok(host);
        todo!()
    }

    fn get_window() -> Self::Window {
        stdweb::web::window();
        todo!()
    }
}
