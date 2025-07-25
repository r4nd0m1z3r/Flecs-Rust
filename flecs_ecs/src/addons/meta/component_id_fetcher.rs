use flecs_ecs::prelude::*;

pub struct ComponentIdFetcher<T> {
    pub phantom: core::marker::PhantomData<T>,
}

#[derive(Debug)]
pub struct FetchedId<T> {
    pub id: u64,
    phantom: core::marker::PhantomData<T>,
}

impl<T> Clone for FetchedId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for FetchedId<T> {}

impl<T> From<FetchedId<T>> for u64 {
    fn from(id: FetchedId<T>) -> u64 {
        id.id
    }
}

impl<T> From<FetchedId<T>> for Entity {
    fn from(id: FetchedId<T>) -> Entity {
        Entity::new(id.id())
    }
}

impl<T> FetchedId<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            phantom: core::marker::PhantomData,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

pub trait FlecsComponent<T> {
    fn deref_id<'a>(&self, world: impl WorldProvider<'a>) -> FetchedId<T>;
}

pub trait ExternalComponent<T> {
    fn deref_id<'a>(&self, world: impl WorldProvider<'a>) -> FetchedId<T>;
}

impl<T: ComponentId> FlecsComponent<T> for &&ComponentIdFetcher<T> {
    fn deref_id<'a>(&self, world: impl WorldProvider<'a>) -> FetchedId<T> {
        FetchedId::new(T::entity_id(world))
    }
}

impl<T: 'static> ExternalComponent<T> for &ComponentIdFetcher<T> {
    fn deref_id<'a>(&self, world: impl WorldProvider<'a>) -> FetchedId<T> {
        let world = world.world();
        let map = world.components_map();
        let id = *(map.entry(core::any::TypeId::of::<T>()).or_insert_with(|| {
            let type_name = get_only_type_name::<T>();
            let name = compact_str::format_compact!("external_components::{}\0", type_name);
            external_register_component::<true, T>(world, name.as_ptr() as *const _)
        }));
        FetchedId::new(id)
    }
}

// The reason this macro exists is while we could use lookup by name, it's not as efficient as using the typeid map for external types.
// a simple benchmark of looking up 100'000 component lookups by name vs typeid map:
//
// typeid map:
// Elapsed: 236.083µs
// Elapsed per id: 2ns
//
// lookup by name through `external_register_component`:
// Elapsed: 28.224417ms
// Elapsed per id: 282ns
#[macro_export]
macro_rules! id {
    ($world:expr, $type:ty) => {
        (&&&flecs_ecs::addons::meta::ComponentIdFetcher::<$type> {
            phantom: core::marker::PhantomData,
        })
            .deref_id($world)
    };
}

pub use id;

#[cfg(test)]
mod tests {
    use flecs_ecs::prelude::*;
    #[test]
    fn meta_id_macro_test() {
        #[derive(Component)]
        struct Position {
            x: f32,
            y: f32,
        }

        struct ExternalPosition {
            x: f32,
            y: f32,
        }

        let world = World::new();

        let id = id!(&world, Position).id();
        assert_eq!(id, world.component_id::<Position>());
        let id_ext = id!(&world, ExternalPosition).id();
        assert_ne!(id_ext, id);

        //compile test
        let world_ref = world.real_world();

        let id_world_ref = id!(world_ref, Position).id();
        assert_eq!(id_world_ref, id);

        let id_ext_world_ref = id!(world_ref, ExternalPosition).id();
        assert_eq!(id_ext_world_ref, id_ext);
    }

    fn get_id_from_fetcher<T: 'static>(fetcher: FetchedId<T>) -> u64 {
        fetcher.id()
    }

    #[test]
    fn meta_id_macro_test_ref() {
        #[derive(Component)]
        struct Position {
            x: f32,
            y: f32,
        }

        struct ExternalPosition {
            x: f32,
            y: f32,
        }

        let world = World::new();

        let fetcher = id!(&world, Position);
        let id = get_id_from_fetcher(fetcher);
        assert_eq!(id, world.component_id::<Position>());

        let fetcher_ext = id!(&world, ExternalPosition);
        let id_ext = get_id_from_fetcher(fetcher_ext);
        assert_ne!(id_ext, id);
    }
}
