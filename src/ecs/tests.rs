use super::components::Component;
use super::world::World;

#[derive(Debug, PartialEq, Eq)]
struct Foo(usize);
impl Component for Foo {}

#[test]
fn adding_components() {
    let mut world = World::default();
    world.register_components::<Foo>().unwrap();

    let a = world.spawn().with(Foo(10)).id();
    let b = world.spawn().with(Foo(20)).id();

    assert_eq!(world.component::<Foo>(a).as_deref(), Ok(&Foo(10)));
    assert_eq!(world.component::<Foo>(b).as_deref(), Ok(&Foo(20)));
}

#[test]
fn remove_a() {
    let mut world = World::default();
    world.register_components::<Foo>().unwrap();

    let a = world.spawn().with(Foo(10)).id();
    let b = world.spawn().with(Foo(20)).id();

    let a_foo = world.component_storage_mut::<Foo>().unwrap().remove(a);
    assert_eq!(a_foo, Some(Foo(10)));

    assert!(world.component::<Foo>(a).as_deref().is_err());
    assert_eq!(world.component::<Foo>(b).as_deref(), Ok(&Foo(20)));
}

#[test]
fn remove_b() {
    let mut world = World::default();
    world.register_components::<Foo>().unwrap();

    let a = world.spawn().with(Foo(10)).id();
    let b = world.spawn().with(Foo(20)).id();

    let b_foo = world.component_storage_mut::<Foo>().unwrap().remove(b);
    assert_eq!(b_foo, Some(Foo(20)));

    assert_eq!(world.component::<Foo>(a).as_deref(), Ok(&Foo(10)));
    assert!(world.component::<Foo>(b).as_deref().is_err());
}

#[test]
fn remove_both() {
    let mut world = World::default();
    world.register_components::<Foo>().unwrap();

    let a = world.spawn().with(Foo(10)).id();
    let b = world.spawn().with(Foo(20)).id();

    let a_foo = world.component_storage_mut::<Foo>().unwrap().remove(a);
    let b_foo = world.component_storage_mut::<Foo>().unwrap().remove(b);

    assert_eq!(a_foo, Some(Foo(10)));
    assert_eq!(b_foo, Some(Foo(20)));

    assert!(world.component::<Foo>(a).as_deref().is_err());
    assert!(world.component::<Foo>(b).as_deref().is_err());
}

#[test]
fn remove_twice() {
    let mut world = World::default();
    world.register_components::<Foo>().unwrap();

    let a = world.spawn().with(Foo(10)).id();

    let a_foo = world.component_storage_mut::<Foo>().unwrap().remove(a);
    let a_foo_again = world.component_storage_mut::<Foo>().unwrap().remove(a);

    assert_eq!(a_foo, Some(Foo(10)));
    assert_eq!(a_foo_again, None);
}

#[test]
fn add_again() {
    let mut world = World::default();
    world.register_components::<Foo>().unwrap();

    let a = world.spawn().with(Foo(10)).id();
    let a_foo = world.component_storage_mut::<Foo>().unwrap().remove(a);

    assert_eq!(a_foo, Some(Foo(10)));
    assert!(world.component::<Foo>(a).as_deref().is_err());

    world.entity(a).with(Foo(20));

    assert_eq!(world.component::<Foo>(a).as_deref(), Ok(&Foo(20)));
}
