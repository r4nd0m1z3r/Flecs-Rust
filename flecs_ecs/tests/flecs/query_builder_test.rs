#![allow(dead_code)]
use core::cell::Cell;
use core::ffi::c_void;

use crate::common_test::*;
use flecs_ecs::sys;

#[test]
fn query_builder_builder_assign_same_type() {
    let world = World::new();

    let q = world
        .query::<(&Position, &Velocity)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, (_p, _v)| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_builder_assign_to_empty() {
    let world = World::new();

    let q = world
        .query::<(&Position, &Velocity)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_builder_assign_from_empty() {
    let world = World::new();

    let q = world
        .query::<()>()
        .set_cache_kind(QueryCacheKind::Auto)
        .with(&Position::id())
        .with(&Velocity::id())
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_builder_build() {
    let world = World::new();

    let q = world
        .query::<(&Position, &Velocity)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, (_p, _v)| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_builder_build_to_let() {
    let world = World::new();

    let q = world
        .query::<(&Position, &Velocity)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_builder_build_n_statements() {
    let world = World::new();

    let mut q = world.query::<()>();
    q.with(&Position::id());
    q.with(&Velocity::id());
    q.set_cache_kind(QueryCacheKind::Auto);
    let q = q.build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_1_type() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _p| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_2_types() {
    let world = World::new();

    let e1 = world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world.entity().set(Velocity { x: 10, y: 20 });

    let r = world
        .query::<(&mut Position, &Velocity)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_entity(|e, (p, v)| {
        count += 1;
        assert_eq!(e, e1);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);

        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_id_term() {
    let world = World::new();

    let tag = world.entity();

    let e1 = world.entity().add(tag);

    world.entity().set(Velocity { x: 10, y: 20 });

    let r = world
        .query::<()>()
        .with(tag)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_type_term() {
    let world = World::new();

    let e1 = world.entity().set(Position { x: 10, y: 20 });

    world.entity().set(Velocity { x: 10, y: 20 });

    let r = world
        .query::<()>()
        .with(&Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_id_pair_term() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();
    let pears = world.entity();

    let e1 = world.entity().add((likes, apples));

    world.entity().add((likes, pears));

    let r = world
        .query::<()>()
        .with((likes, apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_id_pair_wildcard_term() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();
    let pears = world.entity();

    let e1 = world.entity().add((likes, apples));

    let e2 = world.entity().add((likes, pears));

    let r = world
        .query::<()>()
        .with((likes, *flecs::Wildcard))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_iter(|it, index, ()| {
        if it.entity(index).unwrap() == e1 {
            assert_eq!(it.id(0), world.id_view_from((likes, apples)));
            count += 1;
        }
        if it.entity(index).unwrap() == e2 {
            assert_eq!(it.id(0), world.id_view_from((likes, pears)));
            count += 1;
        }
    });
    assert_eq!(count, 2);
}

#[test]
fn query_builder_type_pair_term() {
    let world = World::new();

    let e1 = world.entity().add((Likes::id(), Apples::id()));

    let e2 = world.entity().add((Likes::id(), Pears::id()));

    let r = world
        .query::<()>()
        .with((Likes::id(), *flecs::Wildcard))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_iter(|it, index, ()| {
        if it.entity(index).unwrap() == e1 {
            assert_eq!(it.id(0), world.id_view_from((Likes::id(), Apples::id())));
            count += 1;
        }
        if it.entity(index).unwrap() == e2 {
            assert_eq!(it.id(0), world.id_view_from((Likes::id(), Pears::id())));
            count += 1;
        }
    });
    assert_eq!(count, 2);
}

#[test]
fn query_builder_pair_term_w_var() {
    let world = World::new();

    let e1 = world.entity().add((Likes::id(), Apples::id()));

    let e2 = world.entity().add((Likes::id(), Pears::id()));

    let r = world
        .query::<()>()
        .with(&Likes::id())
        .second()
        .set_var("Food")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let foo_d_var = r.find_var("Food").unwrap();

    let mut count = 0;
    r.each_iter(|it, index, ()| {
        if it.entity(index).unwrap() == e1 {
            assert_eq!(it.id(0), world.id_view_from((Likes::id(), Apples::id())));
            assert_eq!(it.get_var_by_name("Food"), world.id_view_from(Apples::id()));
            assert_eq!(it.get_var(foo_d_var), world.id_view_from(Apples::id()));
            count += 1;
        }
        if it.entity(index).unwrap() == e2 {
            assert_eq!(it.id(0), world.id_view_from((Likes::id(), Pears::id())));
            assert_eq!(it.get_var_by_name("Food"), world.id_view_from(Pears::id()));
            assert_eq!(it.get_var(foo_d_var), world.id_view_from(Pears::id()));
            count += 1;
        }
    });
    assert_eq!(count, 2);
}

#[test]
fn query_builder_2_pair_terms_w_var() {
    let world = World::new();

    let bob = world.entity().add((Eats::id(), Apples::id()));

    let alice = world
        .entity()
        .add((Eats::id(), Pears::id()))
        .add((Likes::id(), bob));

    bob.add((Likes::id(), alice));

    let r = world
        .query::<()>()
        .with(&Eats::id())
        .second()
        .set_var("Food")
        .with(&Likes::id())
        .second()
        .set_var("Person")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let foo_d_var = r.find_var("Food").unwrap();
    let person_var = r.find_var("Person").unwrap();

    let mut count = 0;
    r.each_iter(|it, index, ()| {
        if it.entity(index).unwrap() == bob {
            assert_eq!(it.id(0), world.id_view_from((Eats::id(), Apples::id())));
            assert_eq!(it.get_var_by_name("Food"), world.id_view_from(Apples::id()));
            assert_eq!(it.get_var(foo_d_var), world.id_view_from(Apples::id()));

            assert_eq!(it.id(1), world.id_view_from((Likes::id(), alice)));
            assert_eq!(it.get_var_by_name("Person"), alice);
            assert_eq!(it.get_var(person_var), alice);
            count += 1;
        }
        if it.entity(index).unwrap() == alice {
            assert_eq!(it.id(0), world.id_view_from((Eats::id(), Pears::id())));
            assert_eq!(it.get_var_by_name("Food"), world.id_view_from(Pears::id()));
            assert_eq!(it.get_var(foo_d_var), world.id_view_from(Pears::id()));

            assert_eq!(it.id(1), world.id_view_from((Likes::id(), bob)));
            assert_eq!(it.get_var_by_name("Person"), bob);
            assert_eq!(it.get_var(person_var), bob);
            count += 1;
        }
    });
    assert_eq!(count, 2);
}

#[test]
fn query_builder_set_var() {
    let world = World::new();

    let apples = world.entity();
    let pears = world.entity();

    world.entity().add((Likes::id(), apples));

    let e2 = world.entity().add((Likes::id(), pears));

    let r = world
        .query::<()>()
        .with(&Likes::id())
        .second()
        .set_var("Food")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let foo_d_var = r.find_var("Food").unwrap();

    let mut count = 0;
    r.iterable()
        .set_var(foo_d_var, pears)
        .each_iter(|it, index, ()| {
            assert_eq!(it.entity(index).unwrap(), e2);
            assert_eq!(it.id(0), world.id_view_from((Likes::id(), pears)));
            assert_eq!(it.get_var_by_name("Food"), pears);
            assert_eq!(it.get_var(foo_d_var), pears);
            count += 1;
        });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_set_2_vars() {
    let world = World::new();

    let apples = world.entity();
    let pears = world.entity();

    let bob = world.entity().add((Eats::id(), apples));

    let alice = world
        .entity()
        .add((Eats::id(), pears))
        .add((Likes::id(), bob));

    bob.add((Likes::id(), alice));

    let r = world
        .query::<()>()
        .with(&Eats::id())
        .second()
        .set_var("Food")
        .with(&Likes::id())
        .second()
        .set_var("Person")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let foo_d_var = r.find_var("Food").unwrap();
    let person_var = r.find_var("Person").unwrap();

    let mut count = 0;
    r.iterable()
        .set_var(foo_d_var, pears)
        .set_var(person_var, bob)
        .each_iter(|it, index, ()| {
            assert_eq!(it.entity(index).unwrap(), alice);
            assert_eq!(it.id(0), world.id_view_from((Eats::id(), pears)));
            assert_eq!(it.id(1), world.id_view_from((Likes::id(), bob)));
            assert_eq!(it.get_var_by_name("Food"), pears);
            assert_eq!(it.get_var(foo_d_var), pears);
            assert_eq!(it.get_var_by_name("Person"), bob);
            assert_eq!(it.get_var(person_var), bob);
            count += 1;
        });
    assert_eq!(count, 1);
}

#[test]
fn query_builder_set_var_by_name() {
    let world = World::new();

    let apples = world.entity();
    let pears = world.entity();

    world.entity().add((Likes::id(), apples));

    let e2 = world.entity().add((Likes::id(), pears));

    let r = world
        .query::<()>()
        .with(&Likes::id())
        .second()
        .set_var("Food")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.iterable()
        .set_var_expr("Food", pears)
        .each_iter(|it, index, ()| {
            assert_eq!(it.entity(index).unwrap(), e2);
            assert_eq!(it.id(0), world.id_view_from((Likes::id(), pears)));
            count += 1;
        });
    assert_eq!(count, 1);
}

#[test]
fn query_builder_set_2_vars_by_name() {
    let world = World::new();

    let apples = world.entity();
    let pears = world.entity();

    let bob = world.entity().add((Eats::id(), apples));

    let alice = world
        .entity()
        .add((Eats::id(), pears))
        .add((Likes::id(), bob));

    bob.add((Likes::id(), alice));

    let r = world
        .query::<()>()
        .with(&Eats::id())
        .second()
        .set_var("Food")
        .with(&Likes::id())
        .second()
        .set_var("Person")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let foo_d_var = r.find_var("Food").unwrap();
    let person_var = r.find_var("Person").unwrap();

    let mut count = 0;
    r.iterable()
        .set_var_expr("Food", pears)
        .set_var_expr("Person", bob)
        .each_iter(|it, index, ()| {
            assert_eq!(it.entity(index).unwrap(), alice);
            assert_eq!(it.id(0), world.id_view_from((Eats::id(), pears)));
            assert_eq!(it.id(1), world.id_view_from((Likes::id(), bob)));
            assert_eq!(it.get_var_by_name("Food"), pears);
            assert_eq!(it.get_var(foo_d_var), pears);
            assert_eq!(it.get_var_by_name("Person"), bob);
            assert_eq!(it.get_var(person_var), bob);
            count += 1;
        });
    assert_eq!(count, 1);
}

#[test]
fn query_builder_expr_w_var() {
    let world = World::new();

    let rel = world.entity_named("Rel");
    let obj = world.entity();
    let e = world.entity().add((rel, obj));

    let r = world
        .query::<()>()
        .expr("(Rel, $X)")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let x_var = r.find_var("X").unwrap();
    assert_ne!(x_var, -1);

    let mut count = 0;
    r.each_iter(|it, index, ()| {
        assert_eq!(it.entity(index).unwrap(), e);
        assert_eq!(it.pair(0).unwrap().second_id(), obj);
        count += 1;
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_1_type() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_2_types() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with(&Velocity::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_1_type_w_1_type() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .with(&Velocity::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_2_types_w_1_type() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .with(&Velocity::id())
        .with(&Mass::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 })
        .set(Mass { value: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_pair() {
    let world = World::new();

    let likes = world.entity();
    let bob = world.entity();
    let alice = world.entity();

    let q = world
        .query::<()>()
        .with((likes, bob))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().add((likes, bob));
    world.entity().add((likes, alice));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_not() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .with(&Velocity::id())
        .set_oper(OperKind::Not)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _p| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_add_or() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .set_oper(OperKind::Or)
        .with(&Velocity::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    let e2 = world.entity().set(Velocity { x: 0, y: 0 });
    world.entity().set(Mass { value: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert!((e == e1 || e == e2));
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_add_optional() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with(&Velocity::id())
        .set_oper(OperKind::Optional)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world
        .entity()
        .set(Velocity { x: 0, y: 0 })
        .set(Mass { value: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert!((e == e1 || e == e2));
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_option_type() {
    let world = World::new();

    let q = world
        .query::<(&Position, Option<&Velocity>)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world
        .entity()
        .set(Velocity { x: 0, y: 0 })
        .set(Mass { value: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert!((e == e1 || e == e2));
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_const_type() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _p| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_string_term() {
    let world = World::new();

    world.component::<Position>();

    let q = world
        .query::<()>()
        .expr("flecs.common_test.Position")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_singleton_term() {
    let world = World::new();

    world.set(Other { value: 10 });

    let q = world
        .query::<&SelfRef>()
        .with(&Other::id())
        .singleton()
        .set_inout()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });
    let e = world.entity();
    e.set(SelfRef { value: e.id() });
    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            let o = &it.field::<Other>(1).unwrap()[0];
            assert!(!it.is_self(1));
            assert_eq!(o.value, 10);

            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), it.entity(i).unwrap().id());
                count += 1;
            }
        }
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_isa_superset_term() {
    let world = World::new();

    world
        .component::<Other>()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let q = world
        .query::<&SelfRef>()
        .with(&Other::id())
        .src()
        .up_id(*flecs::IsA)
        .set_in()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            let o = &it.field::<Other>(1).unwrap()[0];
            assert!(!it.is_self(1));
            assert_eq!(o.value, 10);

            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), it.entity(i).unwrap().id());
                count += 1;
            }
        }
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_isa_self_superset_term() {
    let world = World::new();

    world
        .component::<Other>()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let q = world
        .query::<&SelfRef>()
        .with(&Other::id())
        .src()
        .self_()
        .up_id(*flecs::IsA)
        .set_in()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 20 });
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 20 });
    e.set(SelfRef { value: e.id() });

    let mut count = 0;
    let mut owned_count = 0;

    q.run(|mut it| {
        while it.next() {
            let o = &it.field::<Other>(1).unwrap();

            if !it.is_self(1) {
                assert_eq!(o[0].value, 10);
            } else {
                for i in it.iter() {
                    assert_eq!(o[i].value, 20);
                    owned_count += 1;
                }
            }

            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), it.entity(i).unwrap().id());
                count += 1;
            }
        }
    });

    assert_eq!(count, 5);
    assert_eq!(owned_count, 2);
}

#[test]
fn query_builder_childof_superset_term() {
    let world = World::new();

    let q = world
        .query::<&SelfRef>()
        .with(&Other::id())
        .src()
        .up()
        .set_in()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            let o = &it.field::<Other>(1).unwrap()[0];
            assert!(!it.is_self(1));
            assert_eq!(o.value, 10);

            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), it.entity(i).unwrap().id());
                count += 1;
            }
        }
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_childof_self_superset_term() {
    let world = World::new();

    let q = world
        .query::<&SelfRef>()
        .with(&Other::id())
        .src()
        .self_()
        .up()
        .set_in()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 20 });
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 20 });
    e.set(SelfRef { value: e.id() });

    let mut count = 0;
    let mut owned_count = 0;

    q.run(|mut it| {
        while it.next() {
            let o = &it.field::<Other>(1).unwrap();

            if !it.is_self(1) {
                assert_eq!(o[0].value, 10);
            } else {
                for i in it.iter() {
                    assert_eq!(o[i].value, 20);
                    owned_count += 1;
                }
            }

            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), it.entity(i).unwrap().id());
                count += 1;
            }
        }
    });

    assert_eq!(count, 5);
    assert_eq!(owned_count, 2);
}

#[test]
fn query_builder_isa_superset_term_w_each() {
    let world = World::new();

    world
        .component::<Other>()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .src()
        .up_id(*flecs::IsA)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_isa_self_superset_term_w_each() {
    let world = World::new();

    world
        .component::<Other>()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .src()
        .self_()
        .up_id(*flecs::IsA)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((*flecs::IsA, base));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 5);
}

#[test]
fn query_builder_childof_superset_term_w_each() {
    let world = World::new();

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .src()
        .up()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_childof_self_superset_term_w_each() {
    let world = World::new();

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .src()
        .self_()
        .up()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 5);
}

#[test]
fn query_builder_isa_superset_shortcut() {
    let world = World::new();

    world
        .component::<Other>()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .up_id(*flecs::IsA)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().is_a(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().is_a(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().is_a(base);
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_isa_superset_shortcut_w_self() {
    let world = World::new();

    world
        .component::<Other>()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .self_()
        .up_id(*flecs::IsA)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().is_a(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().is_a(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().is_a(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 5);
}

#[test]
fn query_builder_childof_superset_shortcut() {
    let world = World::new();

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .up()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_childof_superset_shortcut_w_self() {
    let world = World::new();

    let q = world
        .query::<(&SelfRef, &Other)>()
        .term_at(1)
        .self_()
        .up()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().child_of(base);
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });
    let e = world.entity().set(Other { value: 10 });
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    q.each_entity(|e, (s, o)| {
        assert_eq!(e, s.value);
        assert_eq!(o.value, 10);
        count += 1;
    });

    assert_eq!(count, 5);
}

#[test]
fn query_builder_relation() {
    let world = World::new();

    let likes = world.entity();
    let bob = world.entity();
    let alice = world.entity();

    let q = world
        .query::<&SelfRef>()
        .with((likes, bob))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity().add((likes, bob));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((likes, bob));
    e.set(SelfRef { value: e.id() });

    let e = world.entity().add((likes, alice));
    e.set(SelfRef { value: Entity(0) });
    let e = world.entity().add((likes, alice));
    e.set(SelfRef { value: Entity(0) });

    let mut count = 0;

    q.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_relation_w_object_wildcard() {
    let world = World::new();

    let likes = world.entity();
    let bob = world.entity();
    let alice = world.entity();

    let q = world
        .query::<&SelfRef>()
        .with((likes, *flecs::Wildcard))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity().add((likes, bob));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((likes, bob));
    e.set(SelfRef { value: e.id() });

    let e = world.entity().add((likes, alice));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((likes, alice));
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: Entity(0) });
    let e = world.entity();
    e.set(SelfRef { value: Entity(0) });

    let mut count = 0;

    q.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 4);
}

#[test]
fn query_builder_relation_w_predicate_wildcard() {
    let world = World::new();

    let likes = world.entity();
    let dislikes = world.entity();
    let bob = world.entity();
    let alice = world.entity();

    let q = world
        .query::<&SelfRef>()
        .with((*flecs::Wildcard, alice))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity().add((likes, alice));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((dislikes, alice));
    e.set(SelfRef { value: e.id() });

    let e = world.entity().add((likes, bob));
    e.set(SelfRef { value: Entity(0) });
    let e = world.entity().add((dislikes, bob));
    e.set(SelfRef { value: Entity(0) });

    let mut count = 0;

    q.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_add_pair_w_rel_type() {
    let world = World::new();

    let dislikes = world.entity();
    let bob = world.entity();
    let alice = world.entity();

    let q = world
        .query::<&SelfRef>()
        .with((Likes::id(), *flecs::Wildcard))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity().add((Likes::id(), alice));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((dislikes, alice));
    e.set(SelfRef { value: Entity(0) });

    let e = world.entity().add((Likes::id(), bob));
    e.set(SelfRef { value: e.id() });
    let e = world.entity().add((dislikes, bob));
    e.set(SelfRef { value: Entity(0) });

    let mut count = 0;

    q.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_template_term() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .with(id::<&Template<u32>>())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Template::<u32> { value: 10 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _p| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_typed_term_at() {
    let world = World::new();

    world.set(Count(0));

    let s = world
        .system::<&Velocity>()
        .with(&mut RelFoo::id())
        .term_at_type::<Velocity>()
        .singleton()
        .term_at_type::<RelFoo>()
        .set_second(flecs::Wildcard::ID)
        .run(|mut it| {
            let world = it.world();
            while it.next() {
                world.get::<&mut Count>(|count| {
                    count.0 += it.count() as i32;
                });
            }
        });

    world.entity().add((RelFoo::id(), Tag));
    world.set(Velocity { x: 0, y: 0 });

    s.run();

    world.get::<&Count>(|count| {
        assert_eq!(count.0, 1);
    });
}

#[test]
fn query_builder_typed_term_at_indexed() {
    let world = World::new();

    world.set(Count(0));

    let s = world
        .system::<&Velocity>()
        .with(&mut RelFoo::id())
        .term_at_checked::<Velocity>(0)
        .singleton()
        .term_at_checked::<RelFoo>(1)
        .set_second(flecs::Wildcard::ID)
        .run(|mut it| {
            let world = it.world();
            while it.next() {
                world.get::<&mut Count>(|count| {
                    count.0 += it.count() as i32;
                });
            }
        });

    world.entity().add((RelFoo::id(), Tag));
    world.set(Velocity { x: 0, y: 0 });

    s.run();

    world.get::<&Count>(|count| {
        assert_eq!(count.0, 1);
    });
}

#[test]
fn query_builder_explicit_subject_w_id() {
    let world = World::new();

    let q = world
        .query::<&Position>()
        .with(&Position::id())
        .set_id(*flecs::This_)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Velocity { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _p| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_explicit_subject_w_type() {
    let world = World::new();

    world.set(Position { x: 10, y: 20 });

    let q = world
        .query::<&Position>()
        .with(&Position::id())
        .set_src(Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    q.each_entity(|e, p| {
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        count += 1;
        assert_eq!(e, world.singleton::<Position>());
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_explicit_object_w_id() {
    let world = World::new();

    let likes = world.entity();
    let alice = world.entity();
    let bob = world.entity();

    let q = world
        .query::<()>()
        .with(likes)
        .set_second(alice)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().add((likes, alice));
    world.entity().add((likes, bob));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_explicit_object_w_type() {
    let world = World::new();

    let likes = world.entity();
    let bob = world.entity();

    let q = world
        .query::<()>()
        .with(likes)
        .set_second(Alice::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .add((likes, *world.id_view_from(Alice::id()).id()));
    world.entity().add((likes, bob));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
#[ignore = "decided to not support explicit term setting"]
fn query_builder_explicit_term() {
    // let world = create_world();

    // let q = world
    //     .query::<()>()
    //     .with(world.term(world.id_view_from(Position::id())))
    //     .set_cache_kind(QueryCacheKind::Auto)
    //     .build();

    // let e1 = world.entity().set(Position { x: 0, y: 0 });
    // world.entity().set(Velocity { x: 0, y: 0 });

    // let mut count = 0;
    // q.each_entity(|e, _| {
    //     count += 1;
    //     assert_eq!(e, e1);
    // });

    // assert_eq!(count, 1);
}

#[test]
#[ignore = "decided to not support explicit term setting"]
fn query_builder_explicit_term_w_type() {
    //     let world = create_world();

    //     let q = world.query::<()>()
    //         .with(world.term<Position>())
    //         .set_cache_kind(QueryCacheKind::Auto)
    //         .build();

    //     let e1 = world.entity().set(Position { x: 0, y: 0 });
    //     world.entity().set(Velocity { x: 0, y: 0 });

    //    let mut count = 0;
    //     q.each_entity(|e, _| {
    //         count += 1;
    //         assert_eq!(e, e1);
    //     });

    //     assert_eq!(count, 1);
}

#[test]
#[ignore = "decided to not support explicit term setting"]
fn query_builder_explicit_term_w_pair_type() {
    //     let world = create_world();

    //     let q = world.query::<()>()
    //         .with((world.term<Likes, alice>()))
    //         .set_cache_kind(QueryCacheKind::Auto)
    //         .build();

    //     let e1 = world.entity().add((Likes::id(), alice::id()));
    //     world.entity().add((Likes::id(), bob::id()));

    //    let mut count = 0;
    //     q.each_entity(|e, _| {
    //         count += 1;
    //         assert_eq!(e, e1);
    //     });

    //     assert_eq!(count, 1);
}

#[test]
#[ignore = "decided to not support explicit term setting"]
fn query_builder_explicit_term_w_id() {
    //     let world = create_world();

    //     let apples = world.entity();
    //    let pears = world.entity();

    //     let q = world.query::<()>()
    //         .with(world.term(apples))
    //         .set_cache_kind(QueryCacheKind::Auto)
    //         .build();

    //     let e1 = world.entity().add(apples);
    //     world.entity().add(pears);

    //    let mut count = 0;
    //     q.each_entity(|e, _| {
    //         count += 1;
    //         assert_eq!(e, e1);
    //     });

    //     assert_eq!(count, 1);
}

#[test]
fn query_builder_explicit_term_w_pair_id() {
    //     let world = create_world();

    //     let likes = world.entity();
    //     let apples = world.entity();
    //    let pears = world.entity();

    //     let q = world.query::<()>()
    //         .with(world.term((likes,apples)))
    //         .set_cache_kind(QueryCacheKind::Auto)
    //         .build();

    //     let e1 = world.entity().add((likes,apples));
    //     world.entity().add((likes,pears));

    //    let mut count = 0;
    //     q.each_entity(|e, _| {
    //         count += 1;
    //         assert_eq!(e, e1);
    //     });

    //     assert_eq!(count, 1);
}

#[test]
fn query_builder_1_term_to_empty() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();

    let mut q = world.query::<()>();
    q.with(&Position::id()).set_cache_kind(QueryCacheKind::Auto);
    q.with((likes, apples));

    let q = q.build();

    assert_eq!(q.field_count(), 2);
    assert_eq!(q.term(0).id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).id(), world.id_view_from((likes, apples)));
}

#[test]
fn query_builder_2_subsequent_args() {
    #[derive(Component, Default)]
    struct Flags {
        count: usize,
    }

    let world = create_world_with_flags::<Flags>();

    let s = world
        .system::<(&mut (RelFoo, flecs::Wildcard), &Velocity)>()
        .term_at(1)
        .singleton()
        .run(|mut it| {
            while it.next() {
                it.real_world().get::<&mut Flags>(|f| f.count += it.count());
            }
        });

    world.entity().set_pair::<RelFoo, Tag>(RelFoo { foo: 10 });
    world.set(Velocity { x: 0, y: 0 });

    s.run();

    world.get::<&Flags>(|f| assert_eq!(f.count, 1));
}

#[test]
fn query_builder_optional_tag_is_set() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&TagA::id())
        .with(&TagB::id())
        .set_oper(OperKind::Optional)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e_1 = world.entity().add(TagA::id()).add(TagB::id());
    let e_2 = world.entity().add(TagA::id());

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);

            count += it.count();

            if it.entity(0).unwrap() == e_1 {
                assert!(it.is_set(0));
                assert!(it.is_set(1));
            } else {
                assert_eq!(it.entity(0).unwrap(), e_2);
                assert!(it.is_set(0));
                assert!(!it.is_set(1));
            }
        }
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_10_terms() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with(&TagA::id())
        .with(&TagB::id())
        .with(&TagC::id())
        .with(&TagD::id())
        .with(&TagE::id())
        .with(&TagF::id())
        .with(&TagG::id())
        .with(&TagH::id())
        .with(&TagI::id())
        .with(&TagJ::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.field_count(), 10);

    let e = world
        .entity()
        .add(TagA::id())
        .add(TagB::id())
        .add(TagC::id())
        .add(TagD::id())
        .add(TagE::id())
        .add(TagF::id())
        .add(TagG::id())
        .add(TagH::id())
        .add(TagI::id())
        .add(TagJ::id());

    let mut count = 0;
    f.run(|mut it| {
        while it.next() {
            assert_eq!(it.field_count(), 10);
            assert_eq!(it.entity(0).unwrap(), e);
            assert_eq!(it.count(), 1);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_16_terms() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with(&TagA::id())
        .with(&TagB::id())
        .with(&TagC::id())
        .with(&TagD::id())
        .with(&TagE::id())
        .with(&TagF::id())
        .with(&TagG::id())
        .with(&TagH::id())
        .with(&TagI::id())
        .with(&TagJ::id())
        .with(&TagK::id())
        .with(&TagL::id())
        .with(&TagM::id())
        .with(&TagN::id())
        .with(&TagO::id())
        .with(&TagP::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.field_count(), 16);

    let e = world
        .entity()
        .add(TagA::id())
        .add(TagB::id())
        .add(TagC::id())
        .add(TagD::id())
        .add(TagE::id())
        .add(TagF::id())
        .add(TagG::id())
        .add(TagH::id())
        .add(TagI::id())
        .add(TagJ::id())
        .add(TagK::id())
        .add(TagL::id())
        .add(TagM::id())
        .add(TagN::id())
        .add(TagO::id())
        .add(TagP::id())
        .add(TagQ::id())
        .add(TagR::id())
        .add(TagS::id())
        .add(TagT::id());

    let mut count = 0;
    f.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            assert_eq!(it.entity(0).unwrap(), e);
            assert_eq!(it.field_count(), 16);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

unsafe extern "C" fn group_by_first_id(
    _world: *mut sys::ecs_world_t,
    table: *mut sys::ecs_table_t,
    _id: u64,
    _ctx: *mut c_void,
) -> u64 {
    unsafe {
        let table_type: *const sys::ecs_type_t = sys::ecs_table_get_type(table);
        *(*table_type).array.add(0)
    }
}

unsafe extern "C" fn group_by_first_id_negated(
    world: *mut sys::ecs_world_t,
    table: *mut sys::ecs_table_t,
    id: u64,
    ctx: *mut c_void,
) -> u64 {
    unsafe { !group_by_first_id(world, table, id, ctx) }
}

#[test]
fn query_builder_group_by_raw() {
    let world = World::new();

    world.component::<TagA>();
    world.component::<TagB>();
    world.component::<TagC>();
    world.component::<TagX>();

    let q = world
        .query::<()>()
        .with(&TagX::id())
        .group_by_fn(world.entity_from::<TagX>(), Some(group_by_first_id))
        .build();

    let q_reverse = world
        .query::<()>()
        .with(&TagX::id())
        .group_by_fn(world.entity_from::<TagX>(), Some(group_by_first_id_negated))
        .build();

    let e3 = world.entity().add(TagX::id()).add(TagC::id());
    let e2 = world.entity().add(TagX::id()).add(TagB::id());
    let e1 = world.entity().add(TagX::id()).add(TagA::id());

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            if count == 0 {
                assert!(it.entity(0).unwrap() == e1);
            } else if count == 1 {
                assert!(it.entity(0).unwrap() == e2);
            } else if count == 2 {
                assert!(it.entity(0).unwrap() == e3);
            } else {
                panic!();
            }
            count += 1;
        }
    });
    assert_eq!(count, 3);

    count = 0;
    q_reverse.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            if count == 0 {
                assert!(it.entity(0).unwrap() == e3);
            } else if count == 1 {
                assert!(it.entity(0).unwrap() == e2);
            } else if count == 2 {
                assert!(it.entity(0).unwrap() == e1);
            } else {
                panic!();
            }
            count += 1;
        }
    });
    assert_eq!(count, 3);
}

#[test]
fn query_builder_group_by_template() {
    let world = World::new();

    world.component::<TagA>();
    world.component::<TagB>();
    world.component::<TagC>();
    world.component::<TagX>();

    let q = world
        .query::<()>()
        .with(&TagX::id())
        .group_by_fn(TagX::id(), Some(group_by_first_id))
        .build();

    let q_reverse = world
        .query::<()>()
        .with(&TagX::id())
        .group_by_fn(TagX::id(), Some(group_by_first_id_negated))
        .build();

    let e3 = world.entity().add(TagX::id()).add(TagC::id());
    let e2 = world.entity().add(TagX::id()).add(TagB::id());
    let e1 = world.entity().add(TagX::id()).add(TagA::id());

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            if count == 0 {
                assert!(it.entity(0).unwrap() == e1);
            } else if count == 1 {
                assert!(it.entity(0).unwrap() == e2);
            } else if count == 2 {
                assert!(it.entity(0).unwrap() == e3);
            } else {
                panic!();
            }
            count += 1;
        }
    });
    assert_eq!(count, 3);

    count = 0;
    q_reverse.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            if count == 0 {
                assert!(it.entity(0).unwrap() == e3);
            } else if count == 1 {
                assert!(it.entity(0).unwrap() == e2);
            } else if count == 2 {
                assert!(it.entity(0).unwrap() == e1);
            } else {
                panic!();
            }
            count += 1;
        }
    });
    assert_eq!(count, 3);
}

unsafe extern "C" fn group_by_rel(
    world: *mut sys::ecs_world_t,
    table: *mut sys::ecs_table_t,
    id: u64,
    _ctx: *mut c_void,
) -> u64 {
    unsafe {
        let mut id_matched: u64 = 0;
        let ref_id = &mut id_matched;
        if sys::ecs_search(world, table, ecs_pair(id, *flecs::Wildcard), ref_id) != -1 {
            return *ecs_second(id_matched, world);
        }
        0
    }
}

#[test]
fn query_builder_group_by_iter_one() {
    let world = World::new();

    let rel = world.entity();
    let tgt_a = world.entity();
    let tgt_b = world.entity();
    let tgt_c = world.entity();
    let tag = world.entity();

    world.entity().add((rel, tgt_a));
    let e2 = world.entity().add((rel, tgt_b));
    world.entity().add((rel, tgt_c));

    world.entity().add((rel, tgt_a)).add(tag);
    let e5 = world.entity().add((rel, tgt_b)).add(tag);
    world.entity().add((rel, tgt_c)).add(tag);

    let q = world
        .query::<()>()
        .with((rel, *flecs::Wildcard))
        .group_by_fn(rel, Some(group_by_rel))
        .build();

    let mut e2_found = false;
    let mut e5_found = false;
    let mut count = 0;

    q.iterable().set_group(tgt_b).each_iter(|mut it, size, ()| {
        let e = it.entity(size).unwrap();
        assert_eq!(it.group_id(), tgt_b);

        if e == e2 {
            e2_found = true;
        }
        if e == e5 {
            e5_found = true;
        }
        count += 1;
    });

    assert_eq!(2, count);
    assert!(e2_found);
    assert!(e5_found);
}

#[test]
fn query_builder_group_by_iter_one_template() {
    let world = World::new();

    world.entity().add((Rel::id(), TagA::id()));
    let e2 = world.entity().add((Rel::id(), TagB::id()));
    world.entity().add((Rel::id(), TagC::id()));

    world.entity().add((Rel::id(), TagA::id())).add(Tag);
    let e5 = world.entity().add((Rel::id(), TagB::id())).add(Tag);
    world.entity().add((Rel::id(), TagC::id())).add(Tag);

    let q = world
        .query::<()>()
        .with((Rel::id(), *flecs::Wildcard))
        .group_by_fn(Rel::id(), Some(group_by_rel))
        .build();

    let mut e2_found = false;
    let mut e5_found = false;
    let mut count = 0;

    q.iterable()
        .set_group(TagB::id())
        .each_iter(|mut it, size, ()| {
            let e = it.entity(size).unwrap();
            assert_eq!(it.group_id(), world.id_view_from(TagB::id()));

            if e == e2 {
                e2_found = true;
            }
            if e == e5 {
                e5_found = true;
            }
            count += 1;
        });

    assert_eq!(2, count);
    assert!(e2_found);
    assert!(e5_found);
}

#[test]
fn query_builder_group_by_iter_one_all_groups() {
    let world = World::new();

    let rel = world.entity();
    let tgt_a = world.entity();
    let tgt_b = world.entity();
    let tgt_c = world.entity();
    let tag = world.entity();

    let e1 = world.entity().add((rel, tgt_a));
    let e2 = world.entity().add((rel, tgt_b));
    let e3 = world.entity().add((rel, tgt_c));

    let e4 = world.entity().add((rel, tgt_a)).add(tag);
    let e5 = world.entity().add((rel, tgt_b)).add(tag);
    let e6 = world.entity().add((rel, tgt_c)).add(tag);

    let q = world
        .query::<()>()
        .with((rel, *flecs::Wildcard))
        .group_by_fn(rel, Some(group_by_rel))
        .build();

    let group_id = Cell::new(0u64);
    let count = Cell::new(0usize);
    let e1_found = Cell::new(false);
    let e2_found = Cell::new(false);
    let e3_found = Cell::new(false);
    let e4_found = Cell::new(false);
    let e5_found = Cell::new(false);
    let e6_found = Cell::new(false);

    let func = |mut it: TableIter<false>, size: usize, ()| {
        let e = it.entity(size).unwrap();
        if it.group_id() == group_id.get() {
            if e == e1 {
                e1_found.set(true);
            }
            if e == e2 {
                e2_found.set(true);
            }
            if e == e3 {
                e3_found.set(true);
            }
            if e == e4 {
                e4_found.set(true);
            }
            if e == e5 {
                e5_found.set(true);
            }
            if e == e6 {
                e6_found.set(true);
            }
            count.set(count.get() + 1);
        }
    };

    group_id.set(*tgt_b.id());
    q.iterable().set_group(tgt_b).each_iter(func);
    assert_eq!(2, count.get());
    assert!(e2_found.get());
    assert!(e5_found.get());

    group_id.set(*tgt_a.id());
    q.iterable().set_group(tgt_a).each_iter(func);
    assert_eq!(4, count.get());
    assert!(e1_found.get());
    assert!(e4_found.get());

    group_id.set(*tgt_c.id());
    q.iterable().set_group(tgt_c).each_iter(func);
    assert_eq!(6, count.get());
    assert!(e3_found.get());
    assert!(e6_found.get());
}

#[test]
fn query_builder_group_by_default_func_w_id() {
    let world = World::new();

    let rel = world.entity();
    let tgt_a = world.entity();
    let tgt_b = world.entity();
    let tgt_c = world.entity();

    let e1 = world.entity().add((rel, tgt_c));
    let e2 = world.entity().add((rel, tgt_b));
    let e3 = world.entity().add((rel, tgt_a));

    let q = world
        .query::<()>()
        .with((rel, *flecs::Wildcard))
        .group_by(rel)
        .build();

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;
    let mut count = 0;

    q.each_iter(|mut it: TableIter<false>, size: usize, ()| {
        let e = it.entity(size).unwrap();
        if e == e1 {
            assert_eq!(it.group_id(), tgt_c);
            assert!(!e1_found);
            assert!(e2_found);
            assert!(e3_found);
            e1_found = true;
        }
        if e == e2 {
            assert_eq!(it.group_id(), tgt_b);
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(e3_found);
            e2_found = true;
        }
        if e == e3 {
            assert_eq!(it.group_id(), tgt_a);
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e3_found = true;
        }
        count += 1;
    });

    assert_eq!(3, count);
    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
}

#[test]
fn query_builder_group_by_default_func_w_type() {
    let world = World::new();

    let tgt_a = world.entity();
    let tgt_b = world.entity();
    let tgt_c = world.entity();

    let e1 = world.entity().add((Rel::id(), tgt_c));
    let e2 = world.entity().add((Rel::id(), tgt_b));
    let e3 = world.entity().add((Rel::id(), tgt_a));

    let q = world
        .query::<()>()
        .with((Rel::id(), id::<flecs::Wildcard>()))
        .group_by(Rel::id())
        .build();

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;
    let mut count = 0;

    q.each_iter(|mut it: TableIter<false>, size: usize, ()| {
        let e = it.entity(size).unwrap();
        if e == e1 {
            assert_eq!(it.group_id(), tgt_c);
            assert!(!e1_found);
            assert!(e2_found);
            assert!(e3_found);
            e1_found = true;
        }
        if e == e2 {
            assert_eq!(it.group_id(), tgt_b);
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(e3_found);
            e2_found = true;
        }
        if e == e3 {
            assert_eq!(it.group_id(), tgt_a);
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e3_found = true;
        }
        count += 1;
    });

    assert_eq!(3, count);
    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
}

extern "C" fn callback_group_create(
    world: *mut sys::ecs_world_t,
    group_id: u64,
    group_by_ctx: *mut c_void,
) -> *mut c_void {
    assert_ne!(world, core::ptr::null_mut());
    assert_ne!(group_id, 0);
    assert_ne!(group_by_ctx, core::ptr::null_mut());
    let cell_count = unsafe { &mut *(group_by_ctx as *mut Cell<u64>) };
    assert_eq!(cell_count.get(), 5);
    let group_id = Box::new(group_id);
    Box::into_raw(group_id) as *mut c_void
}
extern "C" fn callback_group_delete(
    world: *mut sys::ecs_world_t,
    group_id: u64,
    ctx: *mut c_void,
    group_by_ctx: *mut c_void,
) {
    assert_ne!(world, core::ptr::null_mut());
    assert_ne!(group_id, 0);
    assert_ne!(group_by_ctx, core::ptr::null_mut());
    let cell_count = unsafe { &mut *(group_by_ctx as *mut Cell<u64>) };
    assert_eq!(cell_count.get(), 5);
    assert_ne!(ctx, core::ptr::null_mut());
    let group_id_ctx = unsafe { *(ctx as *mut u64) };
    assert_eq!(group_id_ctx, group_id);
    let _box = unsafe { Box::from_raw(ctx as *mut u64) };
}

#[test]
fn query_builder_group_by_callbacks() {
    let cell_count_group_ctx = Cell::new(5u64);
    let world = World::new();

    let tgt_a = world.entity();
    let tgt_b = world.entity();
    let tgt_c = world.entity();

    let e1 = world.entity().add((Rel::id(), tgt_c));
    let e2 = world.entity().add((Rel::id(), tgt_b));
    let e3 = world.entity().add((Rel::id(), tgt_a));

    let q = world
        .query::<()>()
        .with((Rel::id(), *flecs::Wildcard))
        .group_by(Rel::id())
        .group_by_ctx(cell_count_group_ctx.as_ptr() as *mut c_void, None)
        .on_group_create(Some(callback_group_create))
        .on_group_delete(Some(callback_group_delete))
        .build();

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;
    let mut count = 0;

    q.each_iter(|mut it: TableIter<false>, size: usize, ()| {
        let e = it.entity(size).unwrap();
        if e == e1 {
            assert_eq!(it.group_id(), tgt_c);
            assert!(!e1_found);
            assert!(e2_found);
            assert!(e3_found);
            e1_found = true;
            let ctx: *mut u64 = q.group_context(it.group_id()) as *mut u64;
            assert_eq!(unsafe { *ctx }, it.group_id());
        }
        if e == e2 {
            assert_eq!(it.group_id(), tgt_b);
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(e3_found);
            e2_found = true;
            let ctx: *mut u64 = q.group_context(it.group_id()) as *mut u64;
            assert_eq!(unsafe { *ctx }, it.group_id());
        }
        if e == e3 {
            assert_eq!(it.group_id(), tgt_a);
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e3_found = true;
            let ctx: *mut u64 = q.group_context(it.group_id()) as *mut u64;
            assert_eq!(unsafe { *ctx }, it.group_id());
        }
        count += 1;
    });

    assert_eq!(3, count);
    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
}

#[test]
fn query_builder_create_w_no_template_args() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_any_wildcard() {
    let world = World::new();

    let likes = world.entity();
    let apple = world.entity();
    let mango = world.entity();

    let e1 = world.entity().add((likes, apple)).add((likes, mango));

    let q = world
        .query::<()>()
        .with((likes, *flecs::Any))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_cascade() {
    let world = World::new();

    let tag = world
        .entity()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let foo_ = world.entity();
    let bar = world.entity();

    let e0 = world.entity().add(tag);
    let e1 = world.entity().is_a(e0);
    let e2 = world.entity().is_a(e1);
    let e3 = world.entity().is_a(e2);

    let q = world
        .query::<()>()
        .with(tag)
        .cascade_id(*flecs::IsA)
        .set_cached()
        .build();

    e1.add(bar);
    e2.add(foo_);

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;

        if e == e1 {
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e1_found = true;
        }
        if e == e2 {
            assert!(e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e2_found = true;
        }
        if e == e3 {
            assert!(e1_found);
            assert!(e2_found);
            assert!(!e3_found);
            e3_found = true;
        }
    });

    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
    assert_eq!(count, 3);
}

#[test]
fn query_builder_cascade_desc() {
    let world = World::new();

    let tag = world
        .entity()
        .add((flecs::OnInstantiate::ID, flecs::Inherit::ID));

    let foo_ = world.entity();
    let bar = world.entity();

    let e0 = world.entity().add(tag);
    let e1 = world.entity().is_a(e0);
    let e2 = world.entity().is_a(e1);
    let e3 = world.entity().is_a(e2);

    let q = world
        .query::<()>()
        .with(tag)
        .cascade_id(*flecs::IsA)
        .desc()
        .set_cached()
        .build();

    e1.add(bar);
    e2.add(foo_);

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;

        if e == e1 {
            assert!(!e1_found);
            assert!(e2_found);
            assert!(e3_found);
            e1_found = true;
        }
        if e == e2 {
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(e3_found);
            e2_found = true;
        }
        if e == e3 {
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e3_found = true;
        }
    });

    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
    assert_eq!(count, 3);
}

#[test]
fn query_builder_cascade_w_relationship() {
    let world = World::new();

    let tag = world.entity();
    let foo_ = world.entity();
    let bar = world.entity();

    let e0 = world.entity().add(tag);
    let e1 = world.entity().child_of(e0);
    let e2 = world.entity().child_of(e1);
    let e3 = world.entity().child_of(e2);

    let q = world
        .query::<()>()
        .with(tag)
        .cascade_id(*flecs::ChildOf)
        .set_cached()
        .build();

    e1.add(bar);
    e2.add(foo_);

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;

        if e == e1 {
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e1_found = true;
        }
        if e == e2 {
            assert!(e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e2_found = true;
        }
        if e == e3 {
            assert!(e1_found);
            assert!(e2_found);
            assert!(!e3_found);
            e3_found = true;
        }
    });

    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
    assert_eq!(count, 3);
}

#[test]
fn query_builder_up_w_type() {
    let world = World::new();

    world.component::<Rel>().add(*flecs::Traversable);

    let q = world
        .query::<&SelfRef2>()
        .with(&Other::id())
        .src()
        .up_id(Rel::id())
        .set_in()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.entity().set(Other { value: 10 });

    let e = world.entity().add((Rel::id(), base));
    e.set(SelfRef2 { value: e.id() });
    let e = world.entity().add((Rel::id(), base));
    e.set(SelfRef2 { value: e.id() });
    let e = world.entity().add((Rel::id(), base));
    e.set(SelfRef2 { value: e.id() });

    let mut count = 0;

    q.run(|mut it| {
        while it.next() {
            let o = &it.field::<Other>(1).unwrap()[0];
            let s = it.field::<SelfRef2>(0).unwrap();
            assert!(!it.is_self(1));
            assert_eq!(o.value, 10);

            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), s[i].value);
                count += 1;
            }
        }
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_cascade_w_type() {
    let world = World::new();

    world.component::<Rel>().add(*flecs::Traversable);

    let tag = world.entity();
    let foo_ = world.entity();
    let bar = world.entity();

    let e0 = world.entity().add(tag);
    let e1 = world.entity().add((Rel::id(), e0));
    let e2 = world.entity().add((Rel::id(), e1));
    let e3 = world.entity().add((Rel::id(), e2));

    let q = world
        .query::<()>()
        .with(tag)
        .cascade_id(Rel::id())
        .set_cached()
        .build();

    e1.add(bar);
    e2.add(foo_);

    let mut e1_found = false;
    let mut e2_found = false;
    let mut e3_found = false;

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;

        if e == e1 {
            assert!(!e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e1_found = true;
        }
        if e == e2 {
            assert!(e1_found);
            assert!(!e2_found);
            assert!(!e3_found);
            e2_found = true;
        }
        if e == e3 {
            assert!(e1_found);
            assert!(e2_found);
            assert!(!e3_found);
            e3_found = true;
        }
    });

    assert!(e1_found);
    assert!(e2_found);
    assert!(e3_found);
    assert_eq!(count, 3);
}

#[test]
fn query_builder_named_query() {
    let world = World::new();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    let e2 = world.entity().set(Position { x: 0, y: 0 });

    let q = world
        .query_named::<()>("my_query")
        .with(&Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    q.each_entity(|e, _| {
        assert!((e == e1 || e == e2));
        count += 1;
    });
    assert_eq!(count, 2);

    let qe = q.entity();
    assert_ne!(qe, 0);
    assert_eq!(qe.name(), "my_query");
}

#[test]
fn query_builder_term_w_write() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(Position::id())
        .with(Position::id())
        .write_curr()
        .with(&mut Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(0).inout(), InOutKind::Default);
    assert_eq!(q.term(0).src_id(), *flecs::This_);
    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).src_id(), 0);
    assert_eq!(q.term(2).inout(), InOutKind::InOut);
    assert_eq!(q.term(2).src_id(), *flecs::This_);
}

#[test]
fn query_builder_term_w_read() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(Position::id())
        .with(Position::id())
        .read_curr()
        .with(&Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(0).inout(), InOutKind::Default);
    assert_eq!(q.term(0).src_id(), *flecs::This_);
    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).src_id(), 0);
    assert_eq!(q.term(2).inout(), InOutKind::In);
    assert_eq!(q.term(2).src_id(), *flecs::This_);
}

#[test]
#[ignore = "TableIter with stage not implemented"]
fn query_builder_iter_w_stage() {
    //     let world = create_world();

    //     world.set_stage_count(2);
    //     let stage = world.stage(1);

    //     let e1 = world.entity().set(Position { x: 0, y: 0 });

    //     let q = world.query::<&Position>();

    //    let mut count = 0;
    //     q.each(stage, [&](flecs::iter& it, size_t i, Position&) {
    //         assert_eq!(it.world(), stage);
    //         assert_eq!(it.entity(i).unwrap(), e1);
    //         count += 1;
    //     });

    //     assert_eq!(count, 1);
}

// template<typename ... Components>
// struct QueryWrapper
// {
//     QueryWrapper(flecs::query::<Components...> f) : f_(f) {}
//     flecs::query::<Components...> f_;
// };

#[test]
#[ignore = "transform entity to query functionality possibly missing"]
fn query_builder_builder_force_assign_operator() {
    // let world = create_world();

    // let e1 = world.entity().set(Position { x: 10, y: 20 });

    // let q = world
    //     .query::<()>()
    //     .with(&Position::id())
    //     .set_cache_kind(QueryCacheKind::Auto)
    //     .build();
    // let entity_query = q.entity().id();
    // let f = world.entity().set(QueryWrapper {
    //     query_entity: entity_query,
    // });

    // let mut count = 0;
    // let entity_query = f.get::<QueryWrapper>().query_entity;

    // .each_entity(|e, _| {
    //     assert_eq!(e, e1);
    //     count += 1;
    // });
}

#[test]
fn query_builder_query_as_arg() {
    fn query_arg(f: &Query<&SelfRef>) -> i32 {
        let mut count = 0;

        f.each_entity(|e, s| {
            assert_eq!(e, s.value);
            count += 1;
        });

        count
    }

    let world = World::new();

    let f = world
        .query::<&SelfRef>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    assert_eq!(query_arg(&f), 3);
}

#[test]
fn query_builder_query_default_as_move_arg() {
    fn query_move(f: Query<&SelfRef>) -> i32 {
        let mut count = 0;

        f.each_entity(|e, s| {
            assert_eq!(e, s.value);
            count += 1;
        });

        count
    }
    let world = World::new();

    let _f = world.query::<&SelfRef>();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    assert_eq!(query_move(world.new_query::<&SelfRef>()), 3);
}

#[test]
fn query_builder_query_as_return() {
    fn query_return(world: &World) -> Query<&SelfRef> {
        world.new_query::<&SelfRef>()
    }

    let world = World::new();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let f = query_return(&world);

    let mut count = 0;

    f.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_query_copy() {
    let world = World::new();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let f = world
        .query::<&SelfRef>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let f_2 = f;

    let mut count = 0;

    f_2.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_query_1_component() {
    let world = World::new();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    world.each_entity::<&SelfRef>(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_query_2_components() {
    let world = World::new();

    let e = world.entity();
    e.set(SelfRef2 { value: e.id() })
        .set(Position { x: 10, y: 20 });

    let e = world.entity();
    e.set(SelfRef2 { value: e.id() })
        .set(Position { x: 10, y: 20 });

    let e = world.entity();
    e.set(SelfRef2 { value: e.id() })
        .set(Position { x: 10, y: 20 });

    let mut count = 0;

    world.each_entity::<(&SelfRef2, &Position)>(|e, (s, p)| {
        assert_eq!(e, s.value);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_query_1_component_no_entity() {
    let world = World::new();

    world.entity().set(Position { x: 10, y: 20 });

    world.entity().set(Position { x: 10, y: 20 });

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    let mut count = 0;

    world.each::<&Position>(|p| {
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_query_2_components_no_entity() {
    let world = World::new();

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world.entity().set(Position { x: 3, y: 5 });

    world.entity().set(Velocity { x: 20, y: 40 });

    let mut count = 0;

    world.each::<(&Position, &Velocity)>(|(p, v)| {
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_term_after_arg() {
    let world = World::new();

    let e_1 = world
        .entity()
        .add(TagA::id())
        .add(TagB::id())
        .add(TagC::id());

    world.entity().add(TagA::id()).add(TagB::id());

    let f = world
        .query::<(&TagA, &TagB)>()
        .term_at(0)
        .set_src(*flecs::This_) // dummy
        .with(&TagC::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.field_count(), 3);

    let mut count = 0;
    f.run(|mut it| {
        while it.next() {
            for i in it.iter() {
                assert_eq!(it.entity(i).unwrap(), e_1);
                count += 1;
            }
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_name_arg() {
    let world = World::new();

    let e = world.entity_named("Foo").set(Position { x: 10, y: 20 });

    let f = world
        .query::<&Position>()
        .term_at(0)
        .src()
        .name("Foo")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;

    f.run(|mut it| {
        while it.next() {
            let p = it.field::<Position>(0).unwrap();
            assert_eq!(p[0].x, 10);
            assert_eq!(p[0].y, 20);
            assert_eq!(it.src(0), e);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_const_in_term() {
    let world = World::new();

    world.entity().set(Position { x: 10, y: 20 });

    let f = world
        .query::<()>()
        .with(&Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    f.run(|mut it| {
        while it.next() {
            let p = it.field::<Position>(0).unwrap();
            assert!(it.is_readonly(0));
            for i in it.iter() {
                count += 1;
                assert_eq!(p[i].x, 10);
                assert_eq!(p[i].y, 20);
            }
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_const_optional() {
    let world = World::new();

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .add(TagD::id());
    world.entity().add(TagD::id());

    let f = world
        .query::<(&TagD, Option<&Position>)>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    let mut set_count = 0;
    f.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            if it.is_set(1) {
                let p = &it.field::<Position>(1).unwrap()[0];
                assert!(it.is_readonly(1));
                assert_eq!(p.x, 10);
                assert_eq!(p.y, 20);
                set_count += 1;
            }
            count += 1;
        }
    });

    assert_eq!(count, 2);
    assert_eq!(set_count, 1);
}

#[test]
fn query_builder_2_terms_w_expr() {
    let world = World::new();

    let a = world.entity_named("A");
    let b = world.entity_named("B");

    let e1 = world.entity().add(a).add(b);

    let f = world
        .query::<()>()
        .expr("A, B")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.field_count(), 2);

    let mut count = 0;
    f.each_iter(|it, index, ()| {
        if it.entity(index).unwrap() == e1 {
            assert_eq!(it.id(0), a);
            assert_eq!(it.id(1), b);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
#[should_panic]
#[ignore = "panics in C, not captured by Rust"]
fn query_builder_assert_on_uninitialized_term() {
    let world = World::new();

    world.entity_named("A");
    world.entity_named("B");

    let _f = world
        .query::<()>()
        .term()
        .term()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();
}

#[test]
fn query_builder_operator_shortcuts() {
    let world = World::new();

    let a = world.entity();
    let b = world.entity();
    let c = world.entity();
    let d = world.entity();
    let e = world.entity();
    let f = world.entity();
    let g = world.entity();
    let h = world.entity();

    let query = world
        .query::<()>()
        .with(a)
        .and()
        .with(b)
        .or()
        .with(c)
        .with(d)
        .not()
        .with(e)
        .optional()
        .with(f)
        .and_from()
        .with(g)
        .or_from()
        .with(h)
        .not_from()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut t = query.term(0);
    assert_eq!(t.id(), a);
    assert_eq!(t.oper(), OperKind::And);

    t = query.term(1);
    assert_eq!(t.id(), b);
    assert_eq!(t.oper(), OperKind::Or);

    t = query.term(2);
    assert_eq!(t.id(), c);
    assert_eq!(t.oper(), OperKind::And);

    t = query.term(3);
    assert_eq!(t.id(), d);
    assert_eq!(t.oper(), OperKind::Not);

    t = query.term(4);
    assert_eq!(t.id(), e);
    assert_eq!(t.oper(), OperKind::Optional);

    t = query.term(5);
    assert_eq!(t.id(), f);
    assert_eq!(t.oper(), OperKind::AndFrom);

    t = query.term(6);
    assert_eq!(t.id(), g);
    assert_eq!(t.oper(), OperKind::OrFrom);

    t = query.term(7);
    assert_eq!(t.id(), h);
    assert_eq!(t.oper(), OperKind::NotFrom);
}

#[test]
fn query_builder_inout_shortcuts() {
    let world = World::new();

    let a = world.entity();
    let b = world.entity();
    let c = world.entity();
    let d = world.entity();

    let query = world
        .query::<()>()
        .with(a)
        .set_in()
        .with(b)
        .set_out()
        .with(c)
        .set_inout()
        .with(d)
        .set_inout_none()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut t = query.term(0);
    assert_eq!(t.id(), a);
    assert_eq!(t.inout(), InOutKind::In);

    t = query.term(1);
    assert_eq!(t.id(), b);
    assert_eq!(t.inout(), InOutKind::Out);

    t = query.term(2);
    assert_eq!(t.id(), c);
    assert_eq!(t.inout(), InOutKind::InOut);

    t = query.term(3);
    assert_eq!(t.id(), d);
    assert_eq!(t.inout(), InOutKind::None);
}

#[test]
fn query_builder_iter_column_w_const_as_array() {
    let world = World::new();

    let f = world
        .query::<&Position>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world.entity().set(Position { x: 10, y: 20 });
    let e2 = world.entity().set(Position { x: 20, y: 30 });

    let mut count = 0;
    f.run(|mut it| {
        while it.next() {
            let mut p = it.field::<Position>(0).unwrap();
            for i in it.iter() {
                p[i].x += 1;
                p[i].y += 2;

                count += 1;
            }
        }
    });

    assert_eq!(count, 2);

    e1.get::<&Position>(|p| {
        assert_eq!(p.x, 11);
        assert_eq!(p.y, 22);
    });

    e2.get::<&Position>(|p| {
        assert_eq!(p.x, 21);
        assert_eq!(p.y, 32);
    });
}

#[test]
fn query_builder_iter_column_w_const_as_ptr() {
    let world = World::new();

    let f = world
        .query::<&Position>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let base = world.prefab().set(Position { x: 10, y: 20 });
    world.entity().is_a(base);
    world.entity().is_a(base);

    let mut count = 0;
    f.run(|mut it| {
        while it.next() {
            let p = &it.field::<Position>(0).unwrap()[0];
            for _i in it.iter() {
                assert_eq!(p.x, 10);
                assert_eq!(p.y, 20);
                count += 1;
            }
        }
    });

    assert_eq!(count, 2);
}

#[test]
fn query_builder_with() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(world.id_view_from(Position::id()))
        .with(world.id_view_from(Velocity::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_name() {
    let world = World::new();

    world.component::<Velocity>();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with("flecs.common_test.Velocity")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_component() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with(&Velocity::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_pair_id() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();
    let pears = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with((likes, apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, apples));
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_pair_name() {
    let world = World::new();

    let likes = world.entity_named("likes");
    let apples = world.entity_named("Apples");
    let pears = world.entity_named("Pears");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with(("likes", "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, apples));
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_pair_components() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with((Likes::id(), Apples::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), Apples::id()));
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), Pears::id()));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_pair_component_id() {
    let world = World::new();

    let apples = world.entity();
    let pears = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with((Likes::id(), apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), apples));
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_pair_name_component_id() {
    let world = World::new();

    let likes = world.entity_named("Likes");
    let apples = world.entity_named("Apples");
    let pears = world.entity_named("Pears");

    let q = world
        .query::<()>()
        .with(Position::id())
        .with(("Likes", apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, apples));
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_pair_component_name() {
    let world = World::new();

    let apples = world.entity_named("Apples");
    let pears = world.entity_named("Pears");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with((Likes::id(), "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), apples));
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_enum() {
    #[repr(C)]
    #[derive(Component)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .with_enum(Color::Green)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e1 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add_enum(Color::Green);
    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add_enum(Color::Red);

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e1);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(world.id_view_from(Position::id()))
        .without(world.id_view_from(Velocity::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    let e2 = world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_name() {
    let world = World::new();

    world.component::<Velocity>();

    let q = world
        .query::<()>()
        .with(world.id_view_from(Position::id()))
        .without("flecs.common_test.Velocity")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    let e2 = world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_component() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without(Velocity::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .set(Velocity { x: 0, y: 0 });
    let e2 = world.entity().set(Position { x: 0, y: 0 });

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_pair_id() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();
    let pears = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without((likes, apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, apples));
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_pair_name() {
    let world = World::new();

    let likes = world.entity_named("likes");
    let apples = world.entity_named("Apples");
    let pears = world.entity_named("Pears");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without(("likes", "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, apples));
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_pair_components() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without((Likes::id(), Apples::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), Apples::id()));
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), Pears::id()));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_pair_component_id() {
    let world = World::new();

    let apples = world.entity();
    let pears = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without((Likes::id(), apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), apples));
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_pair_component_name() {
    let world = World::new();

    let apples = world.entity_named("Apples");
    let pears = world.entity_named("Pears");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without((Likes::id(), "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), apples));
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((Likes::id(), pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_pair_name_component_id() {
    let world = World::new();

    let likes = world.entity_named("Likes");
    let apples = world.entity_named("Apples");
    let pears = world.entity_named("Pears");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without(("Likes", apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, apples));
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add((likes, pears));

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_without_enum() {
    let world = World::new();

    #[repr(C)]
    #[derive(Component)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    let q = world
        .query::<()>()
        .with(&Position::id())
        .without_enum(Color::Green)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add_enum(Color::Green);
    let e2 = world
        .entity()
        .set(Position { x: 0, y: 0 })
        .add_enum(Color::Red);

    let mut count = 0;
    q.each_entity(|e, _| {
        count += 1;
        assert_eq!(e, e2);
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_write() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write(world.id_view_from(Position::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_name() {
    let world = World::new();

    world.component::<Position>();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write("flecs.common_test.Position")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_component() {
    let world = World::new();

    world.component::<Position>();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write(Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_pair_id() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write((likes, apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), likes);
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_pair_name() {
    let world = World::new();

    let likes = world.entity_named("likes");
    let apples = world.entity_named("Apples");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write(("likes", "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), likes);
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_pair_components() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write((Likes::id(), Apples::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Likes::id()));
    assert_eq!(q.term(1).second_id(), world.id_view_from(Apples::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_pair_component_id() {
    let world = World::new();

    let apples = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write((Likes::id(), apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Likes::id()));
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_pair_component_name() {
    let world = World::new();

    let apples = world.entity_named("Apples");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write((Likes::id(), "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Likes::id()));
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_write_enum() {
    #[repr(C)]
    #[derive(Component)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .write_enum(Color::Green)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::Out);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Color::id()));
    assert_eq!(q.term(1).second_id(), world.entity_from_enum(Color::Green));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read(world.id_view_from(Position::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_name() {
    let world = World::new();

    world.component::<Position>();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read("flecs.common_test.Position")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_component() {
    let world = World::new();

    world.component::<Position>();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read(Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Position::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_pair_id() {
    let world = World::new();

    let likes = world.entity();
    let apples = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read((likes, apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), likes);
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_pair_name() {
    let world = World::new();

    let likes = world.entity_named("likes");
    let apples = world.entity_named("Apples");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read(("likes", "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), likes);
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_pair_components() {
    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read((Likes::id(), Apples::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Likes::id()));
    assert_eq!(q.term(1).second_id(), world.id_view_from(Apples::id()));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_pair_component_id() {
    let world = World::new();

    let apples = world.entity();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read((Likes::id(), apples))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Likes::id()));
    assert_eq!(q.term(1).second_id(), apples);
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_pair_component_name() {
    let world = World::new();

    let apples = world.entity_named("Apples");

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read((Likes::id(), "Apples"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Likes::id()));
    assert_eq!(q.term(1).second_id(), apples);

    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_read_enum() {
    #[repr(C)]
    #[derive(Component)]
    pub enum Color {
        Red,
        Green,
        Blue,
    }

    let world = World::new();

    let q = world
        .query::<()>()
        .with(&Position::id())
        .read_enum(Color::Green)
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(q.term(1).inout(), InOutKind::In);
    assert_eq!(q.term(1).first_id(), world.id_view_from(Color::id()));
    assert_eq!(q.term(1).second_id(), world.entity_from_enum(Color::Green));
    assert_eq!(q.term(1).src_id(), 0);
}

#[test]
fn query_builder_assign_after_init() {
    let world = World::new();

    #[allow(unused_assignments)]
    let mut f: Query<()> = world.new_query::<()>();
    let mut fb = world.query::<()>();
    fb.with(&Position::id());
    fb.set_cache_kind(QueryCacheKind::Auto);
    f = fb.build();

    let e1 = world.entity().set(Position { x: 10, y: 20 });

    let mut count = 0;
    f.each_entity(|e, _| {
        assert_eq!(e, e1);
        count += 1;
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_with_t_inout() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with(world.id_view_from(Position::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.term(0).inout(), InOutKind::Default);
}

#[test]
fn query_builder_with_t_inout_1() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with(Position::id())
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.term(0).inout(), InOutKind::Default);
}

#[test]
fn query_builder_with_r_t_inout_2() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with((Position::id(), Velocity::id()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.term(0).inout(), InOutKind::Default);
}

#[test]
fn query_builder_with_r_t_inout_3() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with((Position::id(), world.entity_from::<Velocity>()))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.term(0).inout(), InOutKind::Default);
}

#[test]
fn query_builder_with_r_t_inout() {
    let world = World::new();

    let f = world
        .query::<()>()
        .with((
            world.entity_from::<Position>(),
            world.entity_from::<Velocity>(),
        ))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    assert_eq!(f.term(0).inout(), InOutKind::Default);
}

#[test]
fn query_builder_query_as_move_arg() {
    fn query_move(f: Query<&SelfRef>) -> i32 {
        let mut count = 0;

        f.each_entity(|e, s| {
            assert_eq!(e, s.value);
            count += 1;
        });

        count
    }
    let world = World::new();

    let f = world
        .query::<&SelfRef>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    assert_eq!(query_move(f), 3);
}

#[test]
fn query_builder_filter_as_return() {
    fn query_auto_return(world: &World) -> Query<&SelfRef> {
        world
            .query::<&SelfRef>()
            .set_cache_kind(QueryCacheKind::Auto)
            .build()
    }

    let world = World::new();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let f = query_auto_return(&world);

    let mut count = 0;

    f.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_filter_copy() {
    let world = World::new();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let f = world
        .query::<&SelfRef>()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let f_2 = f;

    let mut count = 0;

    f_2.each_entity(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_filter_1_component() {
    let world = World::new();

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let e = world.entity();
    e.set(SelfRef { value: e.id() });

    let mut count = 0;

    world.each_entity::<&SelfRef>(|e, s| {
        assert_eq!(e, s.value);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_filter_2_components() {
    let world = World::new();

    let e = world.entity();
    e.set(SelfRef2 { value: e.id() })
        .set(Position { x: 10, y: 20 });

    let e = world.entity();
    e.set(SelfRef2 { value: e.id() })
        .set(Position { x: 10, y: 20 });

    let e = world.entity();
    e.set(SelfRef2 { value: e.id() })
        .set(Position { x: 10, y: 20 });

    let mut count = 0;

    world.each_entity::<(&SelfRef2, &Position)>(|e, (s, p)| {
        assert_eq!(e, s.value);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_filter_1_component_no_entity() {
    let world = World::new();

    world.entity().set(Position { x: 10, y: 20 });

    world.entity().set(Position { x: 10, y: 20 });

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    let mut count = 0;

    world.each::<&Position>(|p| {
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_world_each_filter_2_components_no_entity() {
    let world = World::new();

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    world.entity().set(Position { x: 3, y: 5 });

    world.entity().set(Velocity { x: 20, y: 40 });

    let mut count = 0;

    world.each::<(&Position, &Velocity)>(|(p, v)| {
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 20);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 2);
        count += 1;
    });

    assert_eq!(count, 3);
}

#[test]
fn query_builder_var_src_w_prefixed_name() {
    let world = World::new();

    let r = world
        .query::<()>()
        .with(&Foo::id())
        .set_src("$Var")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity().add(Foo::id());

    let mut count = 0;
    r.run(|mut it| {
        while it.next() {
            assert_eq!(it.get_var_by_name("Var"), e);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_var_first_w_prefixed_name() {
    let world = World::new();

    let r = world
        .query::<()>()
        .with(&Foo::id())
        .term()
        .set_first("$Var")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let e = world.entity().add(Foo::id());

    let mut count = 0;
    r.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            assert_eq!(it.entity(0).unwrap(), e);
            assert_eq!(it.get_var_by_name("Var"), world.id_view_from(Foo::id()));
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_var_second_w_prefixed_name() {
    let world = World::new();

    let r = world
        .query::<()>()
        .with(&Foo::id())
        .set_second("$Var")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let t = world.entity();
    let e = world.entity().add((Foo::id(), t));

    let mut count = 0;
    r.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            assert_eq!(it.entity(0).unwrap(), e);
            assert_eq!(it.get_var_by_name("Var"), t);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_term_w_second_var_string() {
    let world = World::new();

    let foo_ = world.entity();

    let r = world
        .query::<()>()
        .with((foo_, "$Var"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let t = world.entity();
    let e = world.entity().add((foo_, t));

    let mut count = 0;
    r.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            assert_eq!(it.entity(0).unwrap(), e);
            assert_eq!(it.get_var_by_name("Var"), t);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_term_type_w_second_var_string() {
    let world = World::new();

    let r = world
        .query::<()>()
        .with((Foo::id(), "$Var"))
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let t = world.entity();
    let e = world.entity().add((Foo::id(), t));

    let mut count = 0;
    r.run(|mut it| {
        while it.next() {
            assert_eq!(it.count(), 1);
            assert_eq!(it.entity(0).unwrap(), e);
            assert_eq!(it.get_var_by_name("Var"), t);
            count += 1;
        }
    });

    assert_eq!(count, 1);
}

#[test]
fn query_builder_named_rule() {
    let world = World::new();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    let e2 = world.entity().set(Position { x: 0, y: 0 });

    let q = world
        .query_named::<&Position>("my_query")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    q.each_entity(|e, _p| {
        assert!((e == e1 || e == e2));
        count += 1;
    });
    assert_eq!(count, 2);

    let qe = q.entity();
    assert_ne!(qe, 0);
    assert_eq!(qe.name(), "my_query");
}

#[test]
fn query_builder_named_scoped_rule() {
    let world = World::new();

    let e1 = world.entity().set(Position { x: 0, y: 0 });
    let e2 = world.entity().set(Position { x: 0, y: 0 });

    let q = world
        .query_named::<&Position>("my::query")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    q.each_entity(|e, _p| {
        assert!((e == e1 || e == e2));
        count += 1;
    });
    assert_eq!(count, 2);

    let qe = q.entity();
    assert_ne!(qe, 0);
    assert_eq!(qe.name(), "query");
    assert_eq!(qe.path().unwrap(), "::my::query");
}

#[test]
#[should_panic]
fn query_builder_is_valid() {
    let world = World::new();

    let _q_1 = world.query::<&Position>();

    let _q_2 = world
        .query::<()>()
        .expr("foo")
        .set_cache_kind(QueryCacheKind::Auto)
        .build();
}

#[test]
#[ignore = "We don't support unresolved queries. TODO introduce a try_build command which allows fails."]
fn query_builder_unresolved_by_name() {
    // let world = create_world();

    // let q = world.query::<()>()
    //     .flags(EcsQueryAllowUnresolvedByName)
    //     .expr("$this == Foo")
    //     .set_cache_kind(QueryCacheKind::Auto)
    //     .build();

    // assert!(q);

    // test_false(q.iterable().is_true());

    // world.entity_named("Foo");

    // test_true(q.iterable().is_true());
}

#[test]
fn query_builder_scope() {
    let world = World::new();

    let root = world.entity();
    let tag_a = world.entity();
    let tag_b = world.entity();

    world.entity().add(root).add(tag_a).add(tag_b);

    let e2 = world.entity().add(root).add(tag_a);

    world.entity().add(root).add(tag_b);

    world.entity().add(root);

    let r = world
        .query::<()>()
        .with(root)
        .scope_open()
        .not()
        .with(tag_a)
        .without(tag_b)
        .scope_close()
        .set_cache_kind(QueryCacheKind::Auto)
        .build();

    let mut count = 0;
    r.each_entity(|e, _| {
        assert_ne!(e, e2);
        count += 1;
    });

    assert_eq!(count, 3);
}

/*

void QueryBuilder_each_w_field_w_fixed_src(void) {
    flecs::world ecs;

    flecs::entity e1 = ecs.entity()
        .set(Position{10, 20})
        .set(Velocity{1, 2});

    flecs::entity e2 = ecs.entity()
        .set(Position{20, 30});

    auto q = ecs.query_builder()
        .with<Position>()
        .with<Velocity>().src(e1)
        .cache_kind(cache_kind)
        .build();

    int32_t count = 0;
    q.each([&](flecs::iter& it, size_t row) {
        auto e = it.entity(row);
        auto p = it.field_at<Position>(0, row);
        auto v = it.field<const Velocity>(1);

        if (e == e1) {
            test_int(p.x, 10);
            test_int(p.y, 20);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }
        if (e == e2) {
            test_int(p.x, 20);
            test_int(p.y, 30);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }

        count ++;
    });

    test_int(count, 2);
}

void QueryBuilder_each_w_field_at_w_fixed_src(void) {
    flecs::world ecs;

    flecs::entity e1 = ecs.entity()
        .set(Position{10, 20})
        .set(Velocity{1, 2});

    flecs::entity e2 = ecs.entity()
        .set(Position{20, 30});

    auto q = ecs.query_builder()
        .with<Position>()
        .with<Velocity>().src(e1)
        .cache_kind(cache_kind)
        .build();

    int32_t count = 0;
    q.each([&](flecs::iter& it, size_t row) {
        auto e = it.entity(row);
        auto p = it.field_at<Position>(0, row);
        auto v = it.field_at<const Velocity>(1, 0);

        if (e == e1) {
            test_int(p.x, 10);
            test_int(p.y, 20);
            test_int(v.x, 1);
            test_int(v.y, 2);
        }
        if (e == e2) {
            test_int(p.x, 20);
            test_int(p.y, 30);
            test_int(v.x, 1);
            test_int(v.y, 2);
        }

        count ++;
    });

    test_int(count, 2);
}

void QueryBuilder_each_w_const_field_w_fixed_src(void) {
    flecs::world ecs;

    flecs::entity e1 = ecs.entity()
        .set(Position{10, 20})
        .set(Velocity{1, 2});

    flecs::entity e2 = ecs.entity()
        .set(Position{20, 30});

    auto q = ecs.query_builder()
        .with<Position>()
        .with<Velocity>().src(e1)
        .cache_kind(cache_kind)
        .build();

    int32_t count = 0;
    q.each([&](flecs::iter& it, size_t row) {
        auto e = it.entity(row);
        auto p = it.field_at<Position>(0, row);
        auto v = it.field<const Velocity>(1);

        if (e == e1) {
            test_int(p.x, 10);
            test_int(p.y, 20);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }
        if (e == e2) {
            test_int(p.x, 20);
            test_int(p.y, 30);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }

        count ++;
    });

    test_int(count, 2);
}

void QueryBuilder_each_w_const_field_at_w_fixed_src(void) {
    flecs::world ecs;

    flecs::entity e1 = ecs.entity()
        .set(Position{10, 20})
        .set(Velocity{1, 2});

    flecs::entity e2 = ecs.entity()
        .set(Position{20, 30});

    auto q = ecs.query_builder()
        .with<Position>()
        .with<Velocity>().src(e1)
        .cache_kind(cache_kind)
        .build();

    int32_t count = 0;
    q.each([&](flecs::iter& it, size_t row) {
        auto e = it.entity(row);
        auto p = it.field_at<Position>(0, row);
        auto v = it.field_at<const Velocity>(1, 0);

        if (e == e1) {
            test_int(p.x, 10);
            test_int(p.y, 20);
            test_int(v.x, 1);
            test_int(v.y, 2);
        }
        if (e == e2) {
            test_int(p.x, 20);
            test_int(p.y, 30);
            test_int(v.x, 1);
            test_int(v.y, 2);
        }

        count ++;
    });

    test_int(count, 2);
}

void QueryBuilder_each_w_untyped_field_w_fixed_src(void) {
    flecs::world ecs;

    flecs::entity e1 = ecs.entity()
        .set(Position{10, 20})
        .set(Velocity{1, 2});

    flecs::entity e2 = ecs.entity()
        .set(Position{20, 30});

    auto q = ecs.query_builder()
        .with<Position>()
        .with<Velocity>().src(e1)
        .cache_kind(cache_kind)
        .build();

    int32_t count = 0;
    q.each([&](flecs::iter& it, size_t row) {
        auto e = it.entity(row);
        auto p = it.field_at<Position>(0, row);
        flecs::untyped_field vf = it.field(1);
        Velocity *v = static_cast<Velocity*>(vf[0]);

        if (e == e1) {
            test_int(p.x, 10);
            test_int(p.y, 20);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }
        if (e == e2) {
            test_int(p.x, 20);
            test_int(p.y, 30);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }

        count ++;
    });

    test_int(count, 2);
}

void QueryBuilder_each_w_untyped_field_at_w_fixed_src(void) {
    flecs::world ecs;

    flecs::entity e1 = ecs.entity()
        .set(Position{10, 20})
        .set(Velocity{1, 2});

    flecs::entity e2 = ecs.entity()
        .set(Position{20, 30});

    auto q = ecs.query_builder()
        .with<Position>()
        .with<Velocity>().src(e1)
        .cache_kind(cache_kind)
        .build();

    int32_t count = 0;
    q.each([&](flecs::iter& it, size_t row) {
        auto e = it.entity(row);
        auto p = it.field_at<Position>(0, row);
        void *vptr = it.field_at(1, 0);
        Velocity *v = static_cast<Velocity*>(vptr);

        if (e == e1) {
            test_int(p.x, 10);
            test_int(p.y, 20);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }
        if (e == e2) {
            test_int(p.x, 20);
            test_int(p.y, 30);
            test_int(v->x, 1);
            test_int(v->y, 2);
        }

        count ++;
    });

    test_int(count, 2);
}

void QueryBuilder_singleton_pair(void) {
    flecs::world ecs;

    flecs::entity rel = ecs.component<Position>();
    flecs::entity tgt = ecs.entity();

    ecs.set<Position>(tgt, {10, 20});

    int32_t count = 0;

    auto q = ecs.query_builder<const Position>()
        .term_at(0).second(tgt).singleton()
        .cache_kind(cache_kind)
        .build();

    q.each([&](flecs::iter& it, size_t, const Position& p) {
        test_assert(it.src(0) == rel);
        test_assert(it.pair(0) == ecs.pair<Position>(tgt));
        test_int(p.x, 10);
        test_int(p.y, 20);
        count ++;
    });

    test_int(count, 1);
}

void QueryBuilder_query_w_this_second(void) {
    flecs::world ecs;

    flecs::entity rel = ecs.entity();

    auto q = ecs.query_builder()
        .with(rel, flecs::This)
        .build();

    flecs::entity e1 = ecs.entity();
    e1.add(rel, e1);

    int32_t count = 0;
    q.each([&](flecs::entity e) {
        test_assert(e == e1);
        count ++;
    });

    test_int(count, 1);
}

*/
