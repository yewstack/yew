use super::VNode;

#[derive(Debug, PartialEq)]
pub struct VNodeStruct {
    vnode: VNode,
    children: Option<Box<Vec<VNodeStruct>>>,
}

impl VNodeStruct {
    pub fn new(vnode: VNode) -> Self {
        match vnode {
            VNode::VTag(ref vtag) => VNodeStruct {
                vnode: vnode.clone(),
                children: if !vtag.children.is_empty() {
                    Some(Box::new(vec![VNodeStruct::new(VNode::VList(
                        vtag.children.clone(),
                    ))]))
                } else {
                    None
                },
            },
            VNode::VText(_) => VNodeStruct {
                vnode,
                children: None,
            },
            VNode::VList(ref vlist) => VNodeStruct {
                vnode: vnode.clone(),
                children: if !vlist.children.is_empty() {
                    Some(Box::new(
                        vlist
                            .children
                            .clone()
                            .into_iter()
                            .map(VNodeStruct::new)
                            .collect(),
                    ))
                } else {
                    None
                },
            },
            VNode::VComp(_) => VNodeStruct {
                vnode,
                children: None,
            },
            VNode::VRef(_) => VNodeStruct {
                vnode,
                children: None,
            },
        }
    }
}
