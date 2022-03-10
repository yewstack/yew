//! Per-subtree state of apps

/// Data kept per controlled subtree. [Portal] and [AppHandle] serve as
/// host of (pairwise) unrelated subtrees.
///
/// [Portal]: super::bportal::BPortal
/// [AppHandle]: super::app_handle::AppHandle
#[derive(Debug, Clone)]
pub struct BundleRoot;
