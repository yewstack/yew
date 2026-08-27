use std::cell::{self, RefCell};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::rc::Rc;

use super::{DomSlotVariant, Node};

type PhantomNotSendNorSync = PhantomData<*const u8>;

/// A dummy struct that serves as a marker for the borrow of [`LINK_FOREST`].
#[derive(Default)]
pub struct LinkForest {
    _priv: (),
}

thread_local! {
    static LINK_FOREST: RefCell<LinkForest> = {
        RefCell::new(LinkForest::default())
    };
}

pub fn with_forest<R>(f: impl FnOnce(&mut LinkForest) -> R) -> R {
    LINK_FOREST.with_borrow_mut(f)
}

#[derive(Clone)]
struct RawLink(
    /// SAFETY: Comes from `Rc::into_raw`
    NonNull<RefCell<Link>>,
);

impl RawLink {
    fn new(link: Link) -> Self {
        let the_rc = Rc::new(RefCell::new(link));
        let ptr = Rc::into_raw(the_rc).cast_mut();
        // Pointers from Rc::into_raw are always non-null!
        Self(NonNull::new(ptr).unwrap())
    }

    fn id(&self) -> usize {
        self.0.as_ptr().addr()
    }

    fn inc_strong(&self) {
        unsafe { Rc::increment_strong_count(self.0.as_ptr()) };
    }

    fn into_rc(self) -> Rc<RefCell<Link>> {
        unsafe { Rc::from_raw(self.0.as_ptr()) }
    }

    fn as_ref(&self) -> &RefCell<Link> {
        unsafe { &*self.0.as_ptr() }
    }
}

impl PartialEq for RawLink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(PartialEq)]
pub struct LinkOwner {
    id: Option<RawLink>,
    // The link is tied to this specific thread and can't be accessed elsewhere
    _phantom: PhantomNotSendNorSync,
}

impl Debug for LinkOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:x}", self.id())
    }
}

impl LinkOwner {
    pub fn handle(&self) -> LinkHandle {
        LinkHandle::from_raw(self.id.as_ref().unwrap().clone())
    }

    fn into_inner(self) -> RawLink {
        self.id.unwrap()
    }

    fn take(&mut self) -> LinkOwner {
        Self {
            id: self.id.take(),
            _phantom: PhantomData,
        }
    }

    fn id(&self) -> usize {
        match &self.id {
            Some(id) => id.id(),
            None => 0,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LinkHandle {
    id: RawLink,
    // The link is tied to this specific thread and can't be accessed elsewhere
    _phantom: PhantomNotSendNorSync,
}
impl Debug for LinkHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#<{:x}>", self.id.id())
    }
}

impl LinkHandle {
    const fn from_raw(id: RawLink) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    fn to_owner(&self) -> LinkOwner {
        self.id.inc_strong();
        LinkOwner {
            id: Some(self.id.clone()),
            _phantom: PhantomData,
        }
    }
}

#[allow(unused)]
macro_rules! trace {
    ($msg:literal $(,)?) => {
        ::gloo::console::log!(
            ::std::format!("%c[{}:{}] ", ::std::file!(), ::std::line!()),
            "font-weight: bold",
            ::std::format!($msg)
        )
    };
    ($msg:literal , $( $args:tt ),*) => {
        ::gloo::console::log!(
            ::std::format!("%c[{}:{}] ", ::std::file!(), ::std::line!()),
            "font-weight: bold",
            ::std::format!($msg, $( $args ),* ),
        )
    }
}

struct AsRefRef<'a>(cell::Ref<'a, Link>);
impl AsRef<Link> for AsRefRef<'_> {
    fn as_ref(&self) -> &Link {
        &self.0
    }
}
struct AsRefMut<'a>(cell::RefMut<'a, Link>);
impl AsRef<Link> for AsRefMut<'_> {
    fn as_ref(&self) -> &Link {
        &self.0
    }
}
impl AsMut<Link> for AsRefMut<'_> {
    fn as_mut(&mut self) -> &mut Link {
        &mut self.0
    }
}

impl LinkForest {
    pub fn insert(&mut self, link: DomSlotVariant) -> LinkOwner {
        let raw = RawLink::new(Link::new(link));
        LinkOwner {
            id: Some(raw),
            _phantom: PhantomData,
        }
    }

    fn node<'h>(&self, link: &'h LinkHandle) -> LinkRef<impl 'h + AsRef<Link>> {
        LinkRef::new(link.id.id(), AsRefRef(link.id.as_ref().borrow()))
    }

    fn node_mut<'h>(
        &mut self,
        link: &'h LinkHandle,
    ) -> LinkRef<impl 'h + AsRef<Link> + AsMut<Link>> {
        LinkRef::new(link.id.id(), AsRefMut(link.id.as_ref().borrow_mut()))
    }

    fn remove_node(&mut self, link: LinkOwner) -> Option<LinkRef<Link>> {
        // TODO: this should take the owner by value, but that's incompatible with calling it
        // inside of a Drop method without adding a new "invalid" state.
        let inner = link.into_inner();
        let id = inner.id();
        let owned = inner.into_rc();
        let inner = Rc::try_unwrap(owned).ok()?.into_inner();
        Some(LinkRef::new(id, inner))
    }

    pub fn remove(&mut self, link: &mut LinkOwner) {
        self.remove_link(link.take());
    }

    fn remove_link(&mut self, link: LinkOwner) {
        let mut n = link;
        loop {
            let Some(mut node) = self.remove_node(n) else {
                break;
            };
            debug_assert!(
                node.right().is_none(),
                "can't have children in the represented tree"
            );
            let l = node.left().cloned();
            let rep_p = node.replace_rep_parent(None);
            let p = node.into_inner().parent;
            if let LinkParent::AuxParent(p) = &p {
                // debug_assert!(self.node(p).right() == Some(&n.handle()));
                self.node_mut(p).set_right(l.clone());
            }
            if let Some(l) = l {
                self.node_mut(&l).parent = p;
            }
            let Some(rep_p) = rep_p else {
                break;
            };
            n = rep_p;
        }
    }

    pub fn reassign(&mut self, link: &LinkHandle, new_parent: DomSlotVariant) {
        // removes `link` from its represented tree and moves it to `new_parent`.
        let old_parent_id = self.node_mut(link).rep_parent();
        let (new_parent, new_parent_id) = match new_parent {
            DomSlotVariant::Chained(link) => {
                (LinkParent::PathParent(link.link.clone()), Some(link.link))
            }
            DomSlotVariant::Node(data) => (LinkParent::Root(data), None),
        };
        if old_parent_id == new_parent_id && old_parent_id.is_some() {
            // reassigned to its existing parent, no need to modify.
            return;
        }
        self.splay(link);
        let left = self.node(link).left().cloned();
        let parent = self.node_mut(link).parent.replace(new_parent);
        self.node_mut(link).set_left(None);
        let new_parent_id = new_parent_id.map(|handle| handle.to_owner());
        let old_parent_id = self.node_mut(link).replace_rep_parent(new_parent_id);
        if let Some(l) = left {
            self.node_mut(&l).parent = parent;
        }
        if let Some(old_parent_id) = old_parent_id {
            self.remove_link(old_parent_id);
        }
    }

    pub fn find_root(&mut self, link: &LinkHandle) -> Option<Node> {
        self.access(link);
        match &self.node(link).parent {
            LinkParent::Root(node) => node.clone(),
            _ => unreachable!("access method buggy"),
        }
    }

    // Splay operations on the auxiliary tree
    // In fact, none of the splay operations change the refcount, since they do not modify the
    // represented tree.
    fn splay_parent(&self, link: &LinkHandle) -> Result<LinkHandle, SplayResult> {
        // Due to borrow issues (fixed with polonius?) we can't borrow data here, and do that
        // in the caller with a double match :/
        match &self.node(link).parent {
            LinkParent::AuxParent(parent) => Ok(parent.clone()),
            LinkParent::PathParent(link) => Err(SplayResult::Link(link.clone())),
            LinkParent::Root(_) => Err(SplayResult::Root()),
        }
    }

    fn rotate(&mut self, x: &LinkHandle, p: &LinkHandle) {
        // shift the middle node `m` from `x` to `p`.
        let m;
        debug_assert!(self.node(p).right() == Some(x) || self.node(p).left() == Some(x));
        if self.node(p).left() == Some(x) {
            m = self.node(x).right().cloned();
            self.node_mut(x).set_right(Some(p.clone()));
            self.node_mut(p).set_left(m.clone());
        } else {
            m = self.node(x).left().cloned();
            self.node_mut(x).set_left(Some(p.clone()));
            self.node_mut(p).set_right(m.clone());
        };
        if let Some(m) = m {
            // debug_assert_eq!(self.node_mut(m).parent, LinkParent::AuxParent(x));
            self.node_mut(&m).parent = LinkParent::AuxParent(p.clone());
        }
        // attach `x` to the parent of `p`
        let g = self
            .node_mut(p)
            .parent
            .replace(LinkParent::AuxParent(x.clone()));
        if let LinkParent::AuxParent(g) = &g {
            let mut g = self.node_mut(g);
            debug_assert!(g.right() == Some(p) || g.left() == Some(p));
            if g.left() == Some(p) {
                g.set_left(Some(x.clone()));
            } else {
                g.set_right(Some(x.clone()));
            }
        }
        self.node_mut(x).parent = g;
    }

    fn splay(&mut self, link: &LinkHandle) -> SplayResult {
        let x = link;
        loop {
            let mut p = match self.splay_parent(x) {
                Ok(p) => p,
                Err(done) => return done,
            };
            if let Ok(g) = self.splay_parent(&p) {
                // check for zig-zig or zig-zag
                // zig-zig can be implemented by first rotating p and g, followed by x and p
                // zig-zag can be implemented by first rotating x and p, followed by x and g
                let x_is_left = self.node(&p).left() == Some(x);
                let p_is_left = self.node(&g).left() == Some(&p);
                if x_is_left == p_is_left {
                    self.rotate(&p, &g);
                } else {
                    self.rotate(x, &p);
                    p = g;
                }
            }
            self.rotate(x, &p);
        }
    }

    // Link/cut operations
    fn access(&mut self, link: &LinkHandle) {
        // We use an iterative approach to traverse a possible long chain of references.
        // See issue #3043 for why a recursive call is impossible for large lists in vdom.
        // Also does not change any refcounts
        let (mut curr, mut prev) = (link.clone(), None);
        loop {
            let link = self.splay(&curr);
            // found a path-parent pointer. now we cut this one
            let d = self.node_mut(&curr).right().cloned();
            if let Some(ref prev) = prev {
                // debug_assert_eq!(self.node(prev).parent, LinkParent::PathParent(curr));
                self.node_mut(prev).parent = LinkParent::AuxParent(curr.clone());
                // small deviation from the original paper: we do not remove the tail
                // of the preferred path the first node is already on.
                // this would originally run unconditionally of prev.is_some()
                self.node_mut(&curr).set_right(Some(prev.clone()));
                if let Some(d) = d {
                    // debug_assert_eq!(self.node(d).parent, LinkParent::AuxParent(curr));
                    self.node_mut(&d).parent = LinkParent::PathParent(curr.clone());
                }
            }
            let SplayResult::Link(link) = link else { break };
            (prev, curr) = (Some(curr), link);
        }
        // now link is on the preferred path, so splay it one last time to put it on top.
        let res = self.splay(link);
        debug_assert!(matches!(res, SplayResult::Root()));
    }
}

enum SplayResult {
    Link(LinkHandle),
    Root(
        // &'a Option<Node>
    ),
}

#[derive(Debug)]
enum LinkParent {
    Root(Option<Node>),
    // parent is on the same preferred path
    AuxParent(LinkHandle),
    // "path-parent pointer" to some other preferred path
    PathParent(LinkHandle),
}

impl LinkParent {
    fn replace(&mut self, next: LinkParent) -> LinkParent {
        std::mem::replace(self, next)
    }
}

struct Link {
    parent: LinkParent,
    left_aux: Option<LinkHandle>,
    right_aux: Option<LinkHandle>,
    rep_parent: Option<LinkOwner>,
}

impl AsRef<Link> for Link {
    fn as_ref(&self) -> &Link {
        self
    }
}

impl AsMut<Link> for Link {
    fn as_mut(&mut self) -> &mut Link {
        self
    }
}

struct LinkRef<L> {
    id: usize,
    link: L,
}

impl<L: AsRef<Link>> Deref for LinkRef<L> {
    type Target = Link;

    fn deref(&self) -> &Self::Target {
        self.link.as_ref()
    }
}

impl<L: AsRef<Link> + AsMut<Link>> DerefMut for LinkRef<L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.link.as_mut()
    }
}

impl<L> LinkRef<L> {
    fn new(id: usize, link: L) -> Self {
        Self { id, link }
    }

    fn into_inner(self) -> L {
        self.link
    }
}

impl<L: AsRef<Link>> LinkRef<L> {
    #[allow(unused)]
    fn debug(&self) -> impl '_ + std::fmt::Debug {
        #[derive(Debug)]
        struct Link<'a> {
            id: usize,
            parent: &'a LinkParent,
            left_aux: Option<usize>,
            right_aux: Option<usize>,
            rep_parent: Option<usize>,
        }
        Link {
            id: self.id,
            parent: &self.parent,
            left_aux: self.left_aux.as_ref().map(|l| l.id.id()),
            right_aux: self.right_aux.as_ref().map(|l| l.id.id()),
            rep_parent: self.rep_parent.as_ref().map(|l| l.id()),
        }
    }

    fn left(&self) -> Option<&LinkHandle> {
        self.left_aux.as_ref()
    }

    fn right(&self) -> Option<&LinkHandle> {
        self.right_aux.as_ref()
    }
}

impl<L: AsMut<Link>> LinkRef<L> {
    fn set_left(&mut self, left: Option<LinkHandle>) {
        self.link.as_mut().left_aux = left;
    }

    fn set_right(&mut self, right: Option<LinkHandle>) {
        self.link.as_mut().right_aux = right;
    }

    fn rep_parent(&mut self) -> Option<LinkHandle> {
        self.link
            .as_mut()
            .rep_parent
            .as_ref()
            .map(|parent| parent.handle())
    }

    fn replace_rep_parent(&mut self, rep_parent: Option<LinkOwner>) -> Option<LinkOwner> {
        std::mem::replace(&mut self.link.as_mut().rep_parent, rep_parent)
    }
}

impl Link {
    pub fn new(parent: DomSlotVariant) -> Self {
        let (parent, link) = match parent {
            DomSlotVariant::Node(node) => (LinkParent::Root(node), None),
            DomSlotVariant::Chained(handle) => (
                LinkParent::PathParent(handle.link.clone()),
                Some(handle.link.to_owner()),
            ),
        };
        Self {
            parent,
            left_aux: None,
            right_aux: None,
            rep_parent: link,
        }
    }
}

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    impl LinkForest {
        pub fn leak(&mut self, link: LinkOwner) -> LinkHandle {
            self.node_mut(&link.handle()).leak();
            LinkHandle::from_raw(link.into_inner())
        }
    }
    impl Link {
        fn leak(&mut self) {
            // self.dec_owner();
        }
    }
}
