#![allow(clippy::disallowed_names)]

use ecs::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Foo(usize);
impl Component for Foo {}

#[test]
fn adding_components() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let b = world.spawn().insert(Foo(20)).id();

    assert_eq!(world.get::<Comp<Foo>>().get(a), Some(&Foo(10)));
    assert_eq!(world.get::<Comp<Foo>>().get(b), Some(&Foo(20)));
}

#[test]
fn remove_a() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let b = world.spawn().insert(Foo(20)).id();

    let a_foo = world.get::<CompMut<Foo>>().remove(a);
    assert_eq!(a_foo, Some(Foo(10)));

    assert!(world.get::<Comp<Foo>>().get(a).is_none());
    assert_eq!(world.get::<Comp<Foo>>().get(b), Some(&Foo(20)));
}

#[test]
fn remove_b() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let b = world.spawn().insert(Foo(20)).id();

    let b_foo = world.get::<CompMut<Foo>>().remove(b);
    assert_eq!(b_foo, Some(Foo(20)));

    assert_eq!(world.get::<Comp<Foo>>().get(a), Some(&Foo(10)));
    assert!(world.get::<Comp<Foo>>().get(b).is_none());
}

#[test]
fn remove_both() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let b = world.spawn().insert(Foo(20)).id();

    let a_foo = world.get::<CompMut<Foo>>().remove(a);
    let b_foo = world.get::<CompMut<Foo>>().remove(b);

    assert_eq!(a_foo, Some(Foo(10)));
    assert_eq!(b_foo, Some(Foo(20)));

    assert!(world.get::<Comp<Foo>>().get(a).is_none());
    assert!(world.get::<Comp<Foo>>().get(b).is_none());
}

#[test]
fn remove_twice() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();

    let a_foo = world.get::<CompMut<Foo>>().remove(a);
    let a_foo_again = world.get::<CompMut<Foo>>().remove(a);

    assert_eq!(a_foo, Some(Foo(10)));
    assert_eq!(a_foo_again, None);
}

#[test]
fn add_again() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let a_foo = world.get::<CompMut<Foo>>().remove(a);

    assert_eq!(a_foo, Some(Foo(10)));
    assert!(world.get::<Comp<Foo>>().get(a).is_none());

    world.entity(a).insert(Foo(20));

    assert_eq!(world.get::<Comp<Foo>>().get(a), Some(&Foo(20)));
}

#[test]
fn add_unique() {
    let mut world = World::default();

    world.insert_unique(100usize).unwrap();
    assert_eq!(world.get::<Unique<usize>>().get(), &100usize);

    world
        .run(|mut num: UniqueMut<usize>| {
            *num = 200;
        })
        .unwrap();

    world
        .run(|num: Unique<usize>| {
            assert_eq!(*num, 200);
        })
        .unwrap()
}

#[test]
fn iter() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let b = world.spawn().insert(Foo(20)).id();

    world
        .run(|query: Comp<Foo>| {
            let sum: usize = query.iter().map(|foo| foo.0).sum();
            assert_eq!(sum, 30);

            assert_eq!(query.get(a), Some(&Foo(10)));
            assert_eq!(query.get(b), Some(&Foo(20)));
        })
        .unwrap();
}

#[test]
fn system_mut() {
    let mut world = World::default();

    let a = world.spawn().insert(Foo(10)).id();
    let b = world.spawn().insert(Foo(20)).id();

    world
        .run(|mut query: CompMut<Foo>| {
            *query.get_mut(a).unwrap() = Foo(30);

            assert_eq!(query.get(a), Some(&Foo(30)));
            assert_eq!(query.get(b), Some(&Foo(20)));
        })
        .unwrap();
}

#[test]
fn system_failure() {
    let mut world = World::default();

    let result = world.run(|| -> Result<(), &'static str> { Err("hello, world!") });

    assert_eq!(result, Err(SystemError::ExecutionError("hello, world!")))
}

#[test]
#[should_panic]
fn system_borrow_conflict() {
    let mut world = World::default();

    world.spawn().insert(Foo(10));
    world.spawn().insert(Foo(20));

    world
        .run(|_q1: CompMut<Foo>, _q2: CompMut<Foo>| {})
        .unwrap();
}

#[test]
#[should_panic]
fn remove_entity() {
    let mut world = World::default();

    let entity = world.spawn().insert(Foo(10));
    let a = entity.id();

    entity.despawn();
    world.entity(a);
}

#[test]
#[should_panic]
fn remove_entity_components() {
    let mut world = World::default();

    let entity = world.spawn().insert(Foo(10));
    let a = entity.id();

    entity.despawn();
    world.get::<Comp<Foo>>().get(a);
}

#[test]
fn replace_entity() {
    let mut world = World::default();

    let entity = world.spawn().insert(Foo(10));
    let a = entity.id();

    entity.despawn();

    let b = world.spawn().id();
    assert_eq!(a.entity(), b.entity());
    assert!(world.get::<Comp<Foo>>().get(b).is_none());
}
