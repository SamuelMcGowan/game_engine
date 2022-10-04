mod sparse_set;

pub mod components;
pub mod storage;
pub mod world;

#[derive(Debug)]
pub enum EcsError {
    StorageAlreadyAdded,
}

#[cfg(test)]
mod tests {
    use super::components::Component;
    use super::world::World;

    #[derive(Debug, PartialEq, Eq)]
    struct Foo(usize);
    impl Component for Foo {}

    #[test]
    fn a_test() {
        let mut world = World::default();
        world.register_components::<Foo>().unwrap();

        let a = world.spawn().with(Foo(10)).id();
        let b = world.spawn().with(Foo(20)).id();

        assert_eq!(world.component::<Foo>(a), Some(&Foo(10)));
        assert_eq!(world.component::<Foo>(b), Some(&Foo(20)));
    }
}
