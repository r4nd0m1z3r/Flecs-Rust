//! builder for [`Observer`].

use core::{default, ffi::c_void};

use crate::core::internals::*;
use crate::core::private::internal_SystemAPI;
use crate::core::*;
use crate::sys;

/// [`ObserverBuilder`] is used to configure and build [`Observer`]s.
///
/// Observers are systems that react to events.
/// Observers let applications register callbacks for ECS events.
///
/// These are typically constructed via [`World::observer()`].
pub struct ObserverBuilder<'a, P = (), T: QueryTuple = ()> {
    desc: sys::ecs_observer_desc_t,
    term_builder: TermBuilder,
    world: WorldRef<'a>,
    event_count: usize,
    _phantom: core::marker::PhantomData<&'a (T, P)>,
}

impl<'a, P: ComponentId, T: QueryTuple> ObserverBuilder<'a, P, T> {
    /// Create a new observer builder
    ///
    /// # Arguments
    ///
    /// * `world` - The world to create the observer in
    pub(crate) fn new(world: impl WorldProvider<'a>) -> Self {
        let desc = Default::default();
        let mut obj = Self {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 1,
            world: world.world(),
            _phantom: core::marker::PhantomData,
        };

        obj.desc.events[0] = P::UnderlyingType::entity_id(world.world());
        obj.desc.entity =
            unsafe { sys::ecs_entity_init(world.world_ptr_mut(), &Default::default()) };

        T::populate(&mut obj);

        obj
    }

    /// Create a new observer builder with a name
    ///
    /// # Arguments
    ///
    /// * `world` - The world to create the observer in
    /// * `name` - The name of the observer
    pub fn new_named(world: impl WorldProvider<'a>, name: &str) -> Self {
        let name = compact_str::format_compact!("{}\0", name);

        let desc = Default::default();
        let mut obj = Self {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 1,
            world: world.world(),
            _phantom: core::marker::PhantomData,
        };
        let entity_desc: sys::ecs_entity_desc_t = sys::ecs_entity_desc_t {
            name: name.as_ptr() as *const _,
            sep: SEPARATOR.as_ptr(),
            root_sep: SEPARATOR.as_ptr(),
            ..default::Default::default()
        };

        obj.desc.events[0] = P::entity_id(world.world());
        obj.desc.entity = unsafe { sys::ecs_entity_init(obj.world_ptr_mut(), &entity_desc) };

        T::populate(&mut obj);
        obj
    }
}

impl<'a, P, T: QueryTuple> ObserverBuilder<'a, P, T> {
    pub(crate) fn new_untyped(world: impl WorldProvider<'a>) -> ObserverBuilder<'a, (), T> {
        let desc = Default::default();
        let mut obj = ObserverBuilder {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 0,
            world: world.world(),
            _phantom: core::marker::PhantomData,
        };

        obj.desc.entity =
            unsafe { sys::ecs_entity_init(world.world_ptr_mut(), &Default::default()) };

        T::populate(&mut obj);
        obj
    }

    /// Create a new observer builder from an existing descriptor
    ///
    /// # Arguments
    ///
    /// * `world` - The world to create the observer in
    /// * `desc` - The descriptor to create the observer from
    pub(crate) fn new_from_desc(
        world: impl WorldProvider<'a>,
        desc: sys::ecs_observer_desc_t,
    ) -> Self {
        let mut obj = Self {
            desc,
            term_builder: TermBuilder::default(),
            event_count: 0,
            world: world.world(),
            _phantom: core::marker::PhantomData,
        };

        if obj.desc.entity == 0 {
            obj.desc.entity =
                unsafe { sys::ecs_entity_init(world.world_ptr_mut(), &Default::default()) };
        }

        T::populate(&mut obj);
        obj
    }
}

impl<P, T: QueryTuple> ObserverBuilder<'_, P, T> {
    /// set observer flags, which are the same as Query flags
    ///
    /// # Arguments
    ///
    /// * `flags` - the flags to set
    fn observer_flags(&mut self, flags: ObserverFlags) -> &mut Self {
        self.desc.flags_ |= flags.bits();
        self
    }

    /// Specify the event(s) for when the observer should run.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to add
    pub fn add_event(&mut self, event: impl IntoEntity) -> &mut ObserverBuilder<(), T> {
        let event = *event.into_entity(self.world);
        self.desc.events[self.event_count] = event;
        self.event_count += 1;
        // SAFETY: Same layout
        unsafe { core::mem::transmute(self) }
    }

    /// Invoke observer for anything that matches its query on creation
    ///
    /// # Arguments
    ///
    /// * `should_yield` - If true, the observer will be invoked for all existing entities that match its query
    pub fn yield_existing(&mut self) -> &mut Self {
        self.desc.yield_existing = true;
        self
    }
}

#[doc(hidden)]
impl<'a, P, T: QueryTuple> internals::QueryConfig<'a> for ObserverBuilder<'a, P, T> {
    #[inline(always)]
    fn term_builder(&self) -> &TermBuilder {
        &self.term_builder
    }

    #[inline(always)]
    fn term_builder_mut(&mut self) -> &mut TermBuilder {
        &mut self.term_builder
    }

    #[inline(always)]
    fn query_desc(&self) -> &sys::ecs_query_desc_t {
        &self.desc.query
    }

    #[inline(always)]
    fn query_desc_mut(&mut self) -> &mut sys::ecs_query_desc_t {
        &mut self.desc.query
    }

    #[inline(always)]
    fn count_generic_terms(&self) -> i32 {
        T::COUNT
    }
}
impl<'a, P, T: QueryTuple> TermBuilderImpl<'a> for ObserverBuilder<'a, P, T> {}

impl<'a, P, T: QueryTuple> QueryBuilderImpl<'a> for ObserverBuilder<'a, P, T> {}

impl<'a, P, T> Builder<'a> for ObserverBuilder<'a, P, T>
where
    T: QueryTuple,
{
    type BuiltType = Observer<'a>;

    /// Build the `observer_builder` into an `observer`
    fn build(&mut self) -> Self::BuiltType {
        // ensure that the observer doesn't fetch components for OnAdd events, where data is not initialized
        if self.desc.events[0] == flecs::OnAdd::ID {
            for term in self.desc.query.terms.iter_mut() {
                if (term.first.id | term.id | term.second.id | term.src.id) == 0 {
                    break;
                }

                if term.inout == sys::ecs_inout_kind_t_EcsInOutDefault as i16 {
                    term.inout = sys::ecs_inout_kind_t_EcsInOutNone as i16;
                }
            }
        }

        let observer = Observer::new(self.world(), self.desc);
        for s in self.term_builder.str_ptrs_to_free.iter_mut() {
            unsafe { core::mem::ManuallyDrop::drop(s) };
        }
        self.term_builder.str_ptrs_to_free.clear();
        observer
    }
}

impl<'a, P, T: QueryTuple> WorldProvider<'a> for ObserverBuilder<'a, P, T> {
    fn world(&self) -> WorldRef<'a> {
        self.world
    }
}

implement_reactor_api!(ObserverBuilder<'a, P, T>);
