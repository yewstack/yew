//! Submodule providing the `Spawnable` and `Registrable` traits.

/// A Worker that can be spawned by a spawner.
pub trait Spawnable {
    /// Spawner Type.
    type Spawner;

    /// Creates a spawner.
    fn spawner() -> Self::Spawner;
}

/// A trait to enable public workers being registered in a web worker.
pub trait Registrable {
    /// Registrar Type.
    type Registrar;

    /// Creates a registrar for the current worker.
    fn registrar() -> Self::Registrar;
}
