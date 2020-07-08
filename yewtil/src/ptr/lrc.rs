//! A Reference Counted Pointer optimized for use with Yew.

use std::borrow::Borrow;
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr::NonNull;

use crate::ptr::takeable::Takeable;

type IsZero = bool;

// TODO consider renaming prev and next to new and old respectively.
#[derive(PartialEq, Debug)]
struct Node<T> {
    /// Ptr to previous node
    prev: Option<NonNull<Node<T>>>,
    /// The value at this node
    value: Takeable<T>,
    /// The reference count.
    /// **It keeps track of how many LRCs have this node as their heads**
    count: Cell<usize>,
    /// Ptr to next node.
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    /// Creates a new node
    fn new(value: T) -> Self {
        Node {
            prev: None,
            value: Takeable::new(value),
            count: Cell::new(1),
            next: None,
        }
    }

    /// Puts the node behind a non-null pointer.
    fn into_not_null(self) -> NonNull<Self> {
        unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(self))) }
    }

    /// Gets the reference count of the node
    fn get_count(&self) -> usize {
        self.count.get()
    }

    /// Increments the reference count of the node.
    fn inc_count(&self) {
        let mut count = self.count.get();
        count += 1;
        self.count.set(count);
    }

    /// Decrements the reference count of the node.
    /// It will return true if the count hits zero.
    /// This can be used to determine if the node should be deallocated.
    fn dec_count(&self) -> IsZero {
        let mut count = self.count.get();
        count -= 1;
        self.count.set(count);
        count == 0
    }
}

/// Decrement the ref count of a node, and deallocate the node if the ref-count reaches 0.
///
/// Deallocating involves attaching the node's prev's next to the node's next ptr,
/// and attaching the node's next's prev to the node's prev ptr.
/// This connects the nodes surrounding the provided node with each other.
unsafe fn decrement_and_possibly_deallocate<T>(node: NonNull<Node<T>>) {
    // If the heads ref-count is 0
    if node.as_ref().dec_count() {
        // Attach surrounding nodes to each other as this one is removed.
        if let Some(prev) = (*node.as_ptr()).prev.as_mut() {
            prev.as_mut().next = (*node.as_ptr()).next.take();
        }

        if let Some(next) = (*node.as_ptr()).next.as_mut() {
            next.as_mut().prev = (*node.as_ptr()).prev.take();
        }

        std::ptr::drop_in_place(node.as_ptr());
    }
}

// TODO: missing methods
//
// These linking operations could possibly create cycles. - I'm not sure if they should be exposed - or if they are, tagged with unsafe.
// possibly have safe variants that check for ptr_equality along every node, but that makes linking a O(n^2) operation, which is so bad that you would almost never want to use it.
// link_next - Join another Lrc as a next node. Both nodes must be terminating on the respective ends that are being linked.
// link_prev - Join another Lrc as a prev node. Both nodes must be terminating on the respective ends that are being linked.
//
// unlink_next, unlink_prev - Unlinks the lrc from other linked lrcs.
// into_raw - same as RC
// from_raw - same as RC

/// Linked List Reference Counted Pointer
///
/// A doubly linked list where the head node is used to represent the value contained by the pointer.
/// The remaining nodes represent shared pointers whose values have changed.
/// A `Lrc` pointer can swap its head node to point to the head node of another `Lrc` that it
/// is linked to.
///
/// The `Lrc` allows cheap cloning like an `Rc` pointer.
/// Like `Rc`, `Lrc` will need to allocate a new copy when mutating an instance that has been shared.
/// Unlike `Rc`, the newly allocated node will also have a pointer to the old node because another `Lrc` still exists that
/// has that node as its head.
/// Either `Lrc` is free to walk its head along this linked list, allowing it to update itself
/// to newer or older still-live nodes.
///
/// # Comparison
///
/// |        | Clone copies the | Reference Counted | Mutation                                         | Data of Cloned Smart Pointers |
/// |--------|------------------|-------------------|--------------------------------------------------|-------------------------------|
/// | `Lrc`  | Pointer          | Yes               | Allocate a linked copy of data, or edit in place | Can differ                    |
/// | `Rc`   | Pointer          | Yes               | Allocate a copy of data, or edit in place        | Always identical              |
/// | `Box`  | Data             | No                | Edit in place                                    | Can differ                    |
///
/// `Lrc` functions similarly to `Rc`, but has a small size and performance overhead due to dealing with
/// forward and back pointers, in addition to the reference counters they both have.
/// `Lrc` should be slightly easier to use than `Rc` for `Yew` related tasks,
/// and also gains the ability to perform "action at a distance" operations with `update`,
/// `advance_next`, and `advance_prev`.
///
/// # Terminology
/// * `Lrc` - Linked \[List\] Reference Counted Pointer.
/// * Node - A link that holds data, a reference counter, and prev and next pointers to other nodes.
/// * Head - The node held by a `Lrc`. The head holds the value that represents the `Lrc`,
/// even though the `Lrc`'s connected nodes may contain many other values.
///
/// # Example
/// ```
/// use yewtil::ptr::Lrc;
/// let mut lrc = Lrc::new("Some String".to_string());
///
/// let mut clone = lrc.clone();
///
/// assert!(Lrc::ptr_eq(&lrc, &clone));
/// assert_eq!(lrc.get_count(), 2);
/// assert_eq!(lrc.len(), 1);
///
/// lrc.set("Some new String".to_string());
///
/// assert_eq!(lrc.as_ref(), "Some new String");
/// assert_eq!(clone.as_ref(), "Some String");
/// assert!(!Lrc::ptr_eq(&lrc, &clone));
/// assert_eq!(lrc.get_count(), 1);
/// assert_eq!(lrc.len(), 2);
///
/// clone.update();
///
/// assert_eq!(lrc.as_ref(), "Some new String");
/// assert_eq!(clone.as_ref(), "Some new String");
/// assert!(Lrc::ptr_eq(&lrc, &clone));
/// assert_eq!(lrc.get_count(), 2);
/// assert_eq!(lrc.len(), 1);
///
/// std::mem::drop(clone);
///
/// assert_eq!(lrc.get_count(), 1);
/// assert_eq!(lrc.len(), 1);
/// ```
pub struct Lrc<T> {
    head: Option<NonNull<Node<T>>>,
}

#[allow(clippy::len_without_is_empty)] // If it is empty, the Lrc is destroyed, therefore is_empty is useless
impl<T> Lrc<T> {
    /// Allocates a value behind a `Lrc` pointer.
    ///
    /// This is done by allocating the `Lrc` on the heap next to a reference counter and next and previous pointers.
    pub fn new(value: T) -> Self {
        let node = Node::new(value);
        Lrc {
            head: Some(node.into_not_null()),
        }
    }

    /// Sets a new value as the head, making the previous head to the second node in the list.
    ///
    /// This will not allocate if this `Lrc` has exclusive access to the node whose value is being set.
    /// It will update the head nodes value in that case.
    ///
    /// If the Lrc's head is shared with another `Lrc`, it will push a new node onto its head containing
    /// the new value. Unless the `Lrc` is cloned, or another Lrc updates to point to this node, it will have
    /// exclusive access over this node, and calling `set` will remain cheap.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let mut lrc = Lrc::new(0);
    /// lrc.set(1);
    /// assert_eq!(lrc.as_ref(), &1);
    /// ```
    pub fn set(&mut self, value: T) {
        if self.is_exclusive() {
            // Directly assign the value if the ptr has exclusive access.
            *self.get_mut_head_node().value.as_mut() = value;
        } else {
            // If the ptr is shared, allocate a new node.
            self.push_head(Node::new(value));
        }
    }

    /// Gets a mutable reference to the owned value if this `Lrc` has exclusive ownership over its data.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let mut lrc = Lrc::new(1);
    ///
    /// let inner = lrc.get_mut();
    /// assert_eq!(inner, Some(&mut 1));
    ///
    /// let lrc_clone = lrc.clone();
    ///
    /// let inner = lrc.get_mut();
    /// assert_eq!(inner, None, "Can't get reference because lrc doesn't have exclusive ownership.");
    /// ```
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_exclusive() {
            let node = self.get_mut_head_node();
            // Only this node has ownership, or it is marked dead.
            Some(node.value.as_mut())
        } else {
            None
        }
    }

    /// Tries to get the value at the head of this `Lrc`.
    /// If it has exclusive access, then it will return the value.
    /// If it does not have exclusive access, then the whole `Lrc` will be returned as the Error.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let lrc = Lrc::new(0);
    /// assert_eq!(lrc.try_unwrap(), Ok(0));
    ///
    /// let lrc = Lrc::new(0);
    /// let _cloned_lrc = lrc.clone();
    /// assert!(lrc.try_unwrap().is_err())
    /// ```
    pub fn try_unwrap(self) -> Result<T, Self> {
        if self.is_exclusive() {
            let head: NonNull<Node<T>> = self.head.unwrap();
            unsafe {
                let value = (*head.as_ptr()).value.take();

                if let Some(prev) = (*head.as_ptr()).prev.as_mut() {
                    prev.as_mut().next = (*head.as_ptr()).next.take();
                }

                if let Some(next) = (*head.as_ptr()).next.as_mut() {
                    next.as_mut().prev = (*head.as_ptr()).prev.take();
                }

                // No need to decrement the count, it already is 1
                std::ptr::drop_in_place(head.as_ptr());

                Ok(value)
            }
        } else {
            Err(self)
        }
    }

    /// Indicates that the `Lrc` has linked nodes that are newer than its head.
    pub fn has_prev(&self) -> bool {
        self.get_ref_head_node().prev.is_some()
    }

    /// Indicates that the `Lrc` has linked nodes that are older than its head.
    pub fn has_next(&self) -> bool {
        self.get_ref_head_node().next.is_some()
    }

    /// If this `Lrc` is shared, and one or more of its shared `Lrc`s has been modified,
    /// this will update this `Lrc`'s head to have the most up-to-date node (held currently by one of its clones).
    ///
    /// The returned boolean will be `true` if the call did change the head pointer.
    ///
    /// # Note
    /// This method constitutes an example of [action at a distance](https://en.wikipedia.org/wiki/Action_at_a_distance_(computer_programming)),
    /// as it may not be obvious what value you are setting your `Lrc` to when you call this.
    /// While useful, be wary to not overuse this mechanism, as it can make control-flow difficult to follow.
    ///
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let mut lrc = Lrc::new(0);
    ///
    /// let mut cloned_lrc = lrc.clone();
    /// cloned_lrc.set(1);
    /// assert_eq!(lrc.as_ref(), &0);
    ///
    /// lrc.update();
    /// assert_eq!(lrc.as_ref(), &1);
    /// ```
    pub fn update(&mut self) -> bool {
        let did_update = self.has_prev();
        while let Some(prev) = self.next_back() {
            *self = prev;
        }
        did_update
    }

    /// Advances to the next node. The next node will be a node older than the current one.
    ///
    /// The returned boolean indicates if the attempt to advance to a new position was successful.
    ///
    /// Because advancing decrements the reference count as you move this `Lrc`'s head away from a node,
    /// if there are no other `Lrc`s with their heads pointing at that node, than that node will be deallocated.
    /// If you want an operation that advances to the next node, but doesn't risk deallocating the current node,
    /// try `next` instead, which returns a new `Lrc` instead of mutating one.
    ///
    /// # Note
    /// This method constitutes an example of [action at a distance](https://en.wikipedia.org/wiki/Action_at_a_distance_(computer_programming)),
    /// as it may not be obvious what value you are setting your `Lrc` to when you call this.
    /// While useful, be wary to not overuse this mechanism, as it can make control-flow difficult to follow.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let mut lrc = Lrc::new(0);
    /// let mut clone = lrc.clone();
    /// lrc.set(1);
    /// lrc.advance_next();
    ///
    /// assert_eq!(lrc.as_ref(), &0);
    /// ```
    pub fn advance_next(&mut self) -> bool {
        unsafe {
            let head_node: &mut NonNull<Node<T>> = self.head.as_mut().unwrap();
            let next: Option<NonNull<Node<T>>> = (*head_node.as_ptr()).next;
            if let Some(next) = next {
                decrement_and_possibly_deallocate(*head_node);

                // Increment the count, because the Lrc now has this node as its head
                next.as_ref().inc_count();
                self.head = Some(next);

                true
            } else {
                false
            }
        }
    }

    /// Advances to the previous node. The previous node will be a node newer than the current one.
    ///
    /// The returned boolean indicates if the attempt to advance to a new position was successful.
    ///
    /// Because advancing decrements the reference count as you move this `Lrc`'s head away from a node,
    /// if there are no other `Lrc`s with their heads pointing at that node, than that node will be deallocated.
    /// If you want an operation that advances to the next node, but doesn't risk deallocating the current node,
    /// try `next_back` instead, which returns a new `Lrc` instead of mutating one.
    ///
    /// # Note
    /// This method constitutes an example of [action at a distance](https://en.wikipedia.org/wiki/Action_at_a_distance_(computer_programming)),
    /// as it may not be obvious what value you are setting your `Lrc` to when you call this.
    /// While useful, be wary to not overuse this mechanism, as it can make control-flow difficult to follow.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let mut lrc = Lrc::new(0);
    /// let mut clone = lrc.clone();
    /// lrc.set(1);
    /// clone.advance_back();
    ///
    /// assert_eq!(clone.as_ref(), &1);
    /// ```
    pub fn advance_back(&mut self) -> bool {
        unsafe {
            let head_node: &mut NonNull<Node<T>> = self.head.as_mut().unwrap();
            let prev: Option<NonNull<Node<T>>> = (*head_node.as_ptr()).prev;
            if let Some(prev) = prev {
                decrement_and_possibly_deallocate(*head_node);

                // Increment the count, because a new Lrc has this node as the head
                prev.as_ref().inc_count();
                self.head = Some(prev);

                true
            } else {
                false
            }
        }
    }

    /// Compares head pointers for equality.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let lrc1 = Lrc::new(10);
    /// let lrc2 = Lrc::new(10);
    ///
    /// assert!(lrc1 == lrc2, "Values are the same");
    /// assert!(!Lrc::ptr_eq(&lrc1, &lrc2), "But they are behind different pointers");
    /// ```
    pub fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
        lhs.head.unwrap().eq(&rhs.head.unwrap())
    }

    /// Push a new node to the head of the `Lrc`.
    ///
    /// This guarantees that the new node will be pushed to head,
    /// and that the old **most previous (newest)** node will be attached to the new
    /// head's next ptr.
    ///
    /// This means that for any `Lrc`, navigating to prev nodes
    /// (`Lrc::update`, `Lrc::advance_back`, `<Lrc as DoubleEndedIterator>::next_back`)
    /// will eventually terminate at the new head of _this_ `Lrc`.
    fn push_head(&mut self, mut node: Node<T>) {
        // Make the head point to the absolutely newest node in the list.
        self.update();

        // Assign make the new node's next ptr point to the current head.
        node.next = self.head;
        let node = Some(node.into_not_null());

        let head = self.head.unwrap();
        unsafe {
            // Connect the head to the new node.
            (*head.as_ptr()).prev = node;
            decrement_and_possibly_deallocate(head)
        }

        // Make the new node the head for this Lrc.
        self.head = node;
        // Other nodes will still point to the same heads as they did before,
        // but now calling `update` will cause them to navigate to the same head as this Lrc
    }

    /// Gets the reference count of the head node.
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let lrc = Lrc::new(1);
    /// let count = (&lrc).get_count();
    /// assert_eq!(count, 1);
    ///
    /// let _lrc_clone_1 = lrc.clone();
    /// let count = (&lrc).get_count();
    /// assert_eq!(count, 2);
    ///
    /// let _lrc_clone_2 = lrc.clone();
    /// let count = (&lrc).get_count();
    /// assert_eq!(count, 3);
    /// ```
    pub fn get_count(&self) -> usize {
        self.get_ref_head_node().get_count()
    }

    /// Returns `true` if no other `Lrc`s point to the head node.
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let lrc = Lrc::new(1);
    /// assert!(lrc.is_exclusive());
    /// let _lrc_clone = lrc.clone();
    /// assert!(!lrc.is_exclusive());
    /// ```
    pub fn is_exclusive(&self) -> bool {
        self.get_count() == 1
    }

    /// Returns how many nodes are contained within the list this `Lrc`'s head is a part of.
    ///
    /// It is a `O(n)` operation, dependent on how many nodes are connected to this `Lrc`'s head.
    /// Or put another way, how many `Lrc`s exist in the program that were cloned from this `Lrc`
    /// (or this `Lrc` was cloned from one of them), that have differing head nodes.
    pub fn len(&self) -> usize {
        // This node, plus the length of its next nodes and its prev nodes
        1 + self.next_len() + self.prev_len()
    }

    /// Gets the number of nodes that are older than the head.
    pub fn next_len(&self) -> usize {
        let mut count = 0;
        unsafe {
            let mut node = self.get_ref_head_node();
            while let Some(next_node) = node.next.as_ref() {
                count += 1;
                node = next_node.as_ref()
            }
        }
        count
    }

    /// Gets the number of nodes that are newer than the head.
    pub fn prev_len(&self) -> usize {
        let mut count = 0;
        unsafe {
            let mut node = self.get_ref_head_node();
            while let Some(prev_node) = node.prev.as_ref() {
                count += 1;
                node = prev_node.as_ref()
            }
        }
        count
    }

    /// Gets a mutable reference to the head node.
    fn get_mut_head_node(&mut self) -> &mut Node<T> {
        unsafe { self.head.as_mut().unwrap().as_mut() }
    }

    /// Gets a reference to the head node.
    fn get_ref_head_node(&self) -> &Node<T> {
        unsafe { self.head.as_ref().unwrap().as_ref() }
    }
}

impl<T: Clone> Lrc<T> {
    /// Provides a mutable reference to the head's value.
    /// If the head is shared with another LRC, this will clone the head to ensure exclusive access.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Lrc;
    /// let mut lrc = Lrc::new(1);
    /// let _lrc_clone = lrc.clone();
    ///
    /// assert_eq!((&lrc).get_count(), 2, "There are two Lrcs pointing to the same data.");
    /// assert_eq!(lrc.len(), 1, "The Lrc has a single node.");
    ///
    /// *lrc.make_mut() = 2;
    /// assert_eq!((&lrc).get_count(), 1, "This Lrc has exclusive ownership of this data.");
    /// assert_eq!(lrc.len(), 2, "The other lrc is pointing to the node that holds the value '1'.");
    ///
    /// *lrc.make_mut() = 3;
    /// assert_eq!(lrc.len(), 2, "This Lrc is still exclusive, so no more allocations are needed.");
    /// ```
    pub fn make_mut(&mut self) -> &mut T {
        if !self.is_exclusive() {
            // Clone to ensure unique ownership
            let cloned_value: T = self.clone_inner();
            self.push_head(Node::new(cloned_value))
        }
        self.get_mut_head_node().value.as_mut()
    }

    /// Consumes this Lrc, returning its wrapped value.
    ///
    /// If this Lrc doesn't have exclusive access, it will clone the value.
    pub fn clone_unwrap(self) -> T {
        if self.is_exclusive() {
            let head: NonNull<Node<T>> = self.head.unwrap();
            unsafe {
                let value = (*head.as_ptr()).value.take();

                if let Some(prev) = (*head.as_ptr()).prev.as_mut() {
                    prev.as_mut().next = (*head.as_ptr()).next.take();
                }

                if let Some(next) = (*head.as_ptr()).next.as_mut() {
                    next.as_mut().prev = (*head.as_ptr()).prev.take();
                }

                // No need to decrement the count, it already is 1
                std::ptr::drop_in_place(head.as_ptr());

                value
            }
        } else {
            self.clone_inner()
        }
    }

    /// Clones the wrapped value at the `Lrc`'s head.
    pub fn clone_inner(&self) -> T {
        self.get_ref_head_node().value.as_ref().clone()
    }
}

impl<T: PartialEq> Lrc<T> {
    /// Only sets if the new value is different than the current value.
    ///
    /// It will return true if they were not equal, indicating that an assignment has occurred.
    ///
    /// This is better than `lrc.make_mut().neq_assign(value)` because this will
    /// not allocate a copy if the current value if the current value and the new value don't match,
    /// while `make_mut()` will do that up front, before the equality check.
    pub fn neq_set(&mut self, value: T) -> bool {
        if self.get_ref_head_node().value.as_ref() != &value {
            self.set(value);
            true
        } else {
            false
        }
    }
}

impl<T> Drop for Lrc<T> {
    fn drop(&mut self) {
        let head = self.head.expect("Head should always be present.");
        unsafe {
            decrement_and_possibly_deallocate(head);
        }
    }
}
impl<T> Clone for Lrc<T> {
    fn clone(&self) -> Self {
        if let Some(head) = self.head {
            unsafe {
                head.as_ref().inc_count();
            }
        }
        Lrc { head: self.head }
    }
}

impl<T: PartialEq> PartialEq for Lrc<T> {
    fn eq(&self, other: &Self) -> bool {
        // TODO refactor this to remove the unsafe block.
        unsafe {
            match (self.head, other.head) {
                (Some(lhs), Some(rhs)) => lhs.as_ref().value.eq(&rhs.as_ref().value),
                _ => false,
            }
        }
    }
}

impl<T: Eq> Eq for Lrc<T> {}

impl<T: PartialOrd> PartialOrd for Lrc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_ref_head_node()
            .value
            .partial_cmp(&other.get_ref_head_node().value)
    }
}
impl<T: Ord> Ord for Lrc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_ref_head_node()
            .value
            .cmp(&other.get_ref_head_node().value)
    }
}

impl<T: Hash> Hash for Lrc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ref_head_node().value.hash(state)
    }
}

impl<T> AsRef<T> for Lrc<T> {
    fn as_ref(&self) -> &T {
        &self.get_ref_head_node().value.as_ref()
    }
}

impl<T> Deref for Lrc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.get_ref_head_node().value.as_ref()
    }
}

impl<T> Borrow<T> for Lrc<T> {
    fn borrow(&self) -> &T {
        &self.get_ref_head_node().value.as_ref()
    }
}

impl<T: fmt::Debug> fmt::Debug for Lrc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lrc").field(&self.head).finish()
    }
}

impl<T> Iterator for Lrc<T> {
    type Item = Lrc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_ref_head_node().next.map(|ptr| {
            unsafe {
                ptr.as_ref().inc_count();
            }
            Lrc { head: Some(ptr) }
        })
    }
}

impl<T> DoubleEndedIterator for Lrc<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.get_ref_head_node().prev.map(|ptr| {
            unsafe {
                ptr.as_ref().inc_count();
            }
            Lrc { head: Some(ptr) }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lrc_new() {
        let lrc = Lrc::new(25);
        assert_eq!(
            lrc,
            Lrc {
                head: Some(Node::new(25).into_not_null())
            }
        );
        assert_eq!(lrc.as_ref(), &25)
    }

    #[allow(clippy::redundant_clone)]
    #[test]
    fn clone_lrc() {
        let lrc = Lrc::new(25);
        let copy = lrc.clone();
        assert_eq!(copy.as_ref(), &25)
    }

    #[test]
    fn set_lrc() {
        let mut lrc = Lrc::new(25);
        lrc.set(30);

        assert_eq!(lrc.as_ref(), &30)
    }

    #[test]
    fn len_not_changed_by_setting_exclusive_lrc() {
        let mut lrc = Lrc::new(25);
        lrc.set(24);
        assert_eq!(lrc.len(), 1);
    }

    #[test]
    fn make_mut_will_clone_if_shared() {
        let mut lrc = Lrc::new(0);
        let _shared = lrc.clone();
        lrc.make_mut();

        assert_eq!(lrc.len(), 2);
    }

    #[test]
    fn exclusive_set_equivalent_to_exclusive_make_mut() {
        let mut lrc = Lrc::new(0);
        lrc.set(1);
        assert_eq!(lrc.as_ref(), &1);
        assert_eq!(lrc.len(), 1);
        assert_eq!(lrc.get_count(), 1);

        let mut lrc = Lrc::new(0);
        *lrc.make_mut() = 1;
        assert_eq!(lrc.as_ref(), &1);
        assert_eq!(lrc.len(), 1);
        assert_eq!(lrc.get_count(), 1);
    }

    #[test]
    fn droping_middle_connects_prev_and_next() {
        let mut lrc = Lrc::new(0);
        assert_eq!(
            lrc.get_ref_head_node().count,
            Cell::new(1),
            "exclusive ownership"
        );

        // Clone the initial value so it will stick around towards the end of this test
        let _og_clone = lrc.clone();
        assert_eq!(
            lrc.get_ref_head_node().count,
            Cell::new(2),
            "shared ownership"
        );

        lrc.set(1);

        assert_eq!(lrc.get_ref_head_node().prev, None);
        assert_eq!(lrc.get_ref_head_node().value.as_ref(), &1);
        assert_eq!(lrc.get_ref_head_node().count, Cell::new(1));
        assert!(
            lrc.get_ref_head_node().next.is_some(),
            "Should have pointer to previous head"
        );

        unsafe {
            let lrcs_next = lrc
                .get_ref_head_node()
                .next
                .as_ref()
                .expect("Should have next node")
                .as_ref();
            let lrcs_nexts_prev = lrcs_next
                .prev
                .as_ref()
                .expect("next.prev should be some")
                .as_ref();

            assert_eq!(lrcs_next.value.as_ref(), &0);
            assert_eq!(
                lrcs_next.count,
                Cell::new(1),
                "Should still be owned by the Og Clone"
            );
            assert!(lrcs_next.prev.is_some(), "Should point to head");

            assert_eq!(
                lrcs_nexts_prev,
                lrc.get_ref_head_node(),
                "the head's next ptr's prev ptr should point back to the head"
            );
        }

        // Clone the head.
        let cloned_lrc = lrc.clone();
        assert_eq!(lrc.len(), 2);

        assert_eq!(cloned_lrc.get_ref_head_node().prev, None);
        assert_eq!(cloned_lrc.get_ref_head_node().value.as_ref(), &1);
        assert_eq!(cloned_lrc.get_ref_head_node().count, Cell::new(2));
        assert!(
            cloned_lrc.get_ref_head_node().next.is_some(),
            "Should have pointer to previous head"
        );

        // Replace the head again
        lrc.set(2);

        assert_eq!(lrc.get_ref_head_node().prev, None);
        assert_eq!(
            lrc.get_ref_head_node().value.as_ref(),
            &2,
            "value should now be updated to 2"
        );
        assert_eq!(
            lrc.get_ref_head_node().count,
            Cell::new(1),
            "there should only be one owner of this node"
        );
        assert!(
            lrc.get_ref_head_node().next.is_some(),
            "Should have pointer to previous head"
        );

        unsafe {
            // This should have modified the cloned_lrc's head's prev ptr to point to the head of the lrc
            let cloned_lrcs_heads_prev_value = cloned_lrc
                .get_ref_head_node()
                .prev
                .as_ref()
                .expect("Should point to head")
                .as_ref();
            assert_eq!(cloned_lrcs_heads_prev_value, lrc.get_ref_head_node());
        }

        assert_eq!(lrc.len(), 3);

        // Drop the cloned_lrc, which in cleanup,
        // should attach the head node of lrc (currently of value 2),
        // with the lail node of lrc (value of 0)
        std::mem::drop(cloned_lrc);

        assert_eq!(lrc.len(), 2);

        unsafe {
            let lrcs_next = lrc
                .get_ref_head_node()
                .next
                .as_ref()
                .expect("Should have next node")
                .as_ref();
            assert_eq!(lrcs_next.value.as_ref(), &0);
        }
    }

    #[test]
    fn single_node_older_yeilds_none() {
        let mut lrc = Lrc::new(25);
        let older = lrc.next();
        assert_eq!(older, None)
    }

    #[test]
    fn single_node_newer_yeilds_none() {
        let mut lrc = Lrc::new(25);
        let newer = lrc.next_back();
        assert_eq!(newer, None)
    }

    #[test]
    fn older_traverses_to_previous_lrc() {
        let mut lrc = Lrc::new(25);
        let _clone = lrc.clone();
        lrc.set(26);
        let older = lrc.next();
        assert_eq!(older, Some(Lrc::new(25)))
    }

    #[test]
    fn newer_traverses_back_to_original_head_lrc() {
        let mut lrc = Lrc::new(25);
        let _clone = lrc.clone();
        lrc.set(26);
        let older = lrc.next();
        assert_eq!(older, Some(Lrc::new(25)));
        let newer = older.unwrap().next_back();
        assert_eq!(newer, Some(lrc));
    }

    #[test]
    fn attempt_to_dangle_ref() {
        let lrc = Lrc::new(vec![25]);
        let mut cloned_lrc = lrc.clone();
        let first_item_ref = &lrc.as_ref()[0];
        cloned_lrc.set(vec![22, 23]);
        assert_eq!(first_item_ref, &25)
    }

    #[test]
    fn ptr_eq_positive() {
        let lrc = Lrc::new(24);
        let cloned_lrc = lrc.clone();

        assert!(Lrc::ptr_eq(&lrc, &cloned_lrc));
    }

    #[test]
    fn ptr_eq_negative() {
        let lrc = Lrc::new(24);
        let other_lrc = Lrc::new(24);

        assert!(!Lrc::ptr_eq(&lrc, &other_lrc));
    }

    #[test]
    fn update_sets_lrc_to_have_newest_value() {
        let mut lrc = Lrc::new(0);
        let mut cloned_lrc = lrc.clone();

        cloned_lrc.set(1);
        assert_eq!(cloned_lrc.as_ref(), &1);
        assert_eq!(lrc.as_ref(), &0);

        let did_update = lrc.update();
        assert!(did_update);
        assert_eq!(lrc.as_ref(), &1);
    }

    #[test]
    fn advance_back() {
        let mut lrc = Lrc::new(0);
        let mut clone = lrc.clone();
        lrc.set(1);
        // Move to a newer value
        let did_advance = clone.advance_back();

        assert!(did_advance);
        assert_eq!(clone.as_ref(), &1);
        assert_eq!(clone.len(), 1);
        assert_eq!(clone.get_count(), 2);

        let did_advance = clone.advance_back();
        assert!(!did_advance, "No newer values to advance to.");

        let did_advance = clone.advance_next();
        assert!(
            !did_advance,
            "can't restore old value, as it has be dropped."
        );
    }

    #[test]
    fn advance_next() {
        let mut lrc = Lrc::new(0);
        let mut clone = lrc.clone();
        lrc.set(1);
        // Move to the older value.
        let did_advance = lrc.advance_next();

        assert!(did_advance);
        assert_eq!(lrc.as_ref(), &0);
        assert_eq!(lrc.len(), 1);
        assert_eq!(lrc.get_count(), 2);

        let did_advance = clone.advance_next();
        assert!(!did_advance, "No older values to advance to.");

        let did_advance = clone.advance_back();
        assert!(
            !did_advance,
            "Can't restore old value, as it has be dropped."
        );
    }

    #[test]
    fn size_of_node_overhead() {
        let lrc = Lrc::new(());
        let node_size_overhead_bytes = std::mem::size_of_val(lrc.get_ref_head_node());
        let usize_size = std::mem::size_of::<usize>();
        assert_eq!(node_size_overhead_bytes, usize_size * 4);
    }

    #[test]
    fn size_of_node_for_not_null() {
        let lrc = Lrc::new(Box::new(0));
        let node_size_overhead_bytes = std::mem::size_of_val(lrc.get_ref_head_node());
        let usize_size = std::mem::size_of::<usize>();
        assert_eq!(node_size_overhead_bytes, usize_size * 4);
    }

    #[test]
    fn size_of_node_for_usize() {
        let lrc = Lrc::new(0usize);
        let node_size_overhead_bytes = std::mem::size_of_val(lrc.get_ref_head_node());
        let usize_size = std::mem::size_of::<usize>();
        assert_eq!(node_size_overhead_bytes, usize_size * 5);
    }
}
