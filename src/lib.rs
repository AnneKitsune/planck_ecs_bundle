//! Adds a simple `Bundle` trait that makes it easier to add
//! a group of systems to a `DispatcherBuilder`.
#![deny(missing_docs)]
use world_dispatcher::*;

/// A trait allowing the creation of bundles.
/// Bundles are groups of `System`s that are added together and
/// in order in a `DispatcherBuilder`.
/// # Example
/// ```rust
/// use world_dispatcher::*;
/// use planck_ecs_bundle::*;
/// struct TestBundle;
/// impl Bundle for TestBundle {
///     fn systems() -> Vec<System> {
///         vec![
///             (|| {Ok(())}).system(),
///             (|| {Ok(())}).system(),
///             (|| {println!("hello!"); Ok(())}).system(),
///         ]
///     }
/// }
/// let mut builder = DispatcherBuilder::default();
/// #[allow(unused_assignments)]
/// {
///     builder = TestBundle::insert(builder);
/// }
/// ```
pub trait Bundle {
    /// Returns all the systems of this bundle.
    /// This should normally be the only function you need to implement.
    fn systems() -> Vec<System>;
    /// Inserts the systems of this bundle into the provided `DispatcherBuilder`.
    /// A default implementation is already provided for you.
    fn insert(builder: DispatcherBuilder) -> DispatcherBuilder {
        let mut builder = builder;
        for sys in Self::systems() {
            builder = builder.add_system(sys);
        }
        builder
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn bundle() {
        struct TestBundle;
        impl Bundle for TestBundle {
            fn systems() -> Vec<System> {
                vec![
                    (|| {Ok(())}).system(),
                    (|| {Ok(())}).system(),
                    (|| {println!("hello!"); Ok(())}).system(),
                ]
            }
        }
        let mut builder = DispatcherBuilder::default();
        #[allow(unused_assignments)]
        {
            builder = TestBundle::insert(builder);
        }
    }
}
