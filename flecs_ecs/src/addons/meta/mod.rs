#![doc(hidden)]
mod builtin;
mod component_id_fetcher;
mod cursor;
mod declarations;
mod impl_bindings;
mod impl_primitives;
pub mod macros;
mod meta_functions;
mod meta_traits;
mod opaque;

use core::ffi::{CStr, c_void};

pub use builtin::*;
pub use component_id_fetcher::*;
pub use cursor::*;
pub use declarations::*;
pub use macros::*;
pub use meta_traits::MetaMember;
pub use opaque::*;

use crate::core::*;

use crate::core::ecs_assert;
use crate::sys;

//used for `.member` functions
pub struct Count(pub i32);

impl World {
    /// Find or register component.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The component type.
    ///
    /// # Returns
    ///
    /// The found or registered component.
    pub fn component_ext<T>(&self, id: FetchedId<T>) -> Component<T> {
        Component::<T>::new_id(self, id)
    }

    /// Find or register component and set the name if not already set.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The component type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the component.
    ///
    /// # Returns
    ///
    /// The found or registered component.
    pub fn component_named_ext<'a, T>(&'a self, id: FetchedId<T>, name: &str) -> Component<'a, T> {
        Component::<T>::new_named_id(self, id, name)
    }

    /// Return meta cursor to value
    pub fn cursor<T: ComponentId>(&self, data: &mut T) -> Cursor {
        let type_id = T::get_id(self.world());
        Cursor::new(self, type_id, data as *mut T as *mut c_void)
    }

    /// Return meta cursor to value
    pub fn cursor_id(&self, type_id: impl IntoEntity, ptr: *mut c_void) -> Cursor {
        if ptr.is_null() {
            panic!("ptr is null");
        }

        Cursor::new(self, type_id, ptr)
    }

    /// Create primitive type
    pub fn primitive(&self, kind: EcsPrimitiveKind) -> EntityView {
        let desc = sys::ecs_primitive_desc_t {
            kind: kind as sys::ecs_primitive_kind_t,
            entity: 0u64,
        };

        let eid = unsafe { sys::ecs_primitive_init(self.ptr_mut(), &desc) };
        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InvalidOperation,
            "failed to create primitive type"
        );
        EntityView::new_from(self, eid)
    }

    /// Create array type
    pub fn array(&self, elem_id: impl IntoEntity, array_count: i32) -> EntityView {
        let desc = sys::ecs_array_desc_t {
            type_: *elem_id.into_entity(self),
            count: array_count,
            entity: 0u64,
        };

        let eid = unsafe { sys::ecs_array_init(self.ptr_mut(), &desc) };
        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InvalidOperation,
            "failed to create array type"
        );
        EntityView::new_from(self, eid)
    }

    /// Create vector type
    pub fn vector_id(&self, elem_id: impl Into<Entity>) -> EntityView {
        let elem_id: u64 = *elem_id.into();
        let name_elem = unsafe { sys::ecs_get_name(self.world_ptr(), elem_id) };
        let cstr_name = unsafe { CStr::from_ptr(name_elem) };
        let name =
            compact_str::format_compact!("flecs::meta::vector::{}\0", cstr_name.to_string_lossy());
        let desc = sys::ecs_entity_desc_t {
            name: name.as_ptr() as *const _,
            sep: SEPARATOR.as_ptr(),
            root_sep: SEPARATOR.as_ptr(),
            _canary: 0,
            id: 0,
            parent: 0,
            symbol: core::ptr::null(),
            use_low_id: false,
            add: core::ptr::null(),
            add_expr: core::ptr::null(),
            set: core::ptr::null(),
        };
        let id = unsafe { sys::ecs_entity_init(self.world_ptr_mut(), &desc) };

        let desc = sys::ecs_vector_desc_t {
            entity: id,
            type_: elem_id,
        };

        let eid = unsafe { sys::ecs_vector_init(self.ptr_mut(), &desc) };

        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InvalidOperation,
            "failed to create vector type"
        );

        EntityView::new_from(self, eid)
    }

    /// Create vector type
    pub fn vector<T: 'static>(&self) -> EntityView {
        let id = self.component_id_map::<T>();
        self.vector_id(id)
    }
}

pub trait EcsSerializer {
    fn value_id(&self, type_id: impl Into<Entity>, value: *const c_void) -> i32;
    fn value<T: ComponentId>(&self, value: &T) -> i32;
    fn member(&self, name: &str) -> i32;
}

impl EcsSerializer for sys::ecs_serializer_t {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn value_id(&self, type_id: impl IntoEntity, value: *const c_void) -> i32 {
        if let Some(value_func) = self.value {
            unsafe {
                value_func(
                    self,
                    *type_id.into_entity(WorldRef::from_ptr(self.world as *mut _)),
                    value,
                )
            }
        } else {
            0
        }
    }

    fn value<T: ComponentId>(&self, value: &T) -> i32 {
        self.value_id(
            T::get_id(unsafe { WorldRef::from_ptr(self.world as *mut _) }),
            value as *const T as *const c_void,
        )
    }

    fn member(&self, name: &str) -> i32 {
        let name = compact_str::format_compact!("{}\0", name);
        if let Some(member_func) = self.member {
            unsafe { member_func(self, name.as_ptr() as *const _) }
        } else {
            0
        }
    }
}

/// Register opaque type interface
impl<'a, T: 'static> Component<'a, T> {
    pub fn opaque_func<Func>(&self, func: Func) -> &Self
    where
        Func: FnOnce(WorldRef<'a>) -> Opaque<'a, T>,
    {
        let mut opaque = func(self.world());
        opaque.desc.entity = self.world().component_id_map::<T>();
        unsafe { sys::ecs_opaque_init(self.world_ptr_mut(), &opaque.desc) };
        self
    }

    pub fn opaque_func_id<Func, Elem>(&self, id: impl Into<Entity>, func: Func) -> &Self
    where
        Func: FnOnce(WorldRef<'a>) -> Opaque<'a, T, Elem>,
    {
        let mut opaque = func(self.world());
        opaque.desc.entity = *id.into();
        unsafe { sys::ecs_opaque_init(self.world_ptr_mut(), &opaque.desc) };
        self
    }

    pub fn opaque<Type: 'static>(&self) -> Opaque<'a, T> {
        let id = self.world().component_id_map::<Type>();
        let mut opaque = Opaque::<T>::new(self.world());
        opaque.as_type(id);
        opaque
    }

    pub fn opaque_id(&self, id: impl IntoEntity) -> Opaque<'a, T> {
        let id = id.into_entity(self.world());
        let mut opaque = Opaque::<T>::new(self.world());
        opaque.as_type(id);
        opaque
    }

    pub fn opaque_dyn_id<E>(&self, id_type: E, id_field: E) -> Opaque<'a, T>
    where
        E: Into<Entity> + Copy,
    {
        let mut opaque = Opaque::<T>::new_id(self.world(), id_type);
        opaque.as_type(id_field);
        opaque
    }

    /// Return opaque type builder for collection type
    ///
    /// ```
    /// use flecs_ecs::prelude::*;
    /// let world = World::new();
    ///
    /// #[derive(Component)]
    /// struct SerVec {
    ///     pub value: Vec<i32>,
    /// }
    ///
    /// world
    ///     .component::<SerVec>()
    ///     .opaque_collection_vector::<i32>();
    /// ```
    pub fn opaque_collection_vector<ElemType: 'static>(&self) -> Opaque<'a, T, ElemType> {
        let world = self.world();
        let mut opaque = Opaque::<T, ElemType>::new(self.world());
        let id = world.vector::<ElemType>();
        opaque.as_type(id);
        opaque
    }

    /// Return opaque type builder for collection type
    ///
    /// ```
    /// use flecs_ecs::prelude::*;
    /// let world = World::new();
    ///
    /// #[derive(Component)]
    /// struct SerVec {
    ///     pub value: Vec<i32>,
    /// }
    ///
    /// world
    ///     .component::<SerVec>()
    ///     .opaque_collection_dyn::<i32>(world.vector::<i32>());
    /// ```
    pub fn opaque_collection_dyn<ElemType>(
        &self,
        id: impl Into<Entity>,
    ) -> Opaque<'a, T, ElemType> {
        let id: Entity = id.into();
        let copy_id = id;
        let mut opaque = Opaque::<T, ElemType>::new_id(self.world(), self.id);
        opaque.as_type(copy_id);
        opaque
    }
}

impl<T: EnumComponentInfo + 'static> Component<'_, T> {
    /// Add constant.
    pub fn constant(&self, name: &str, value: T) -> &Self {
        unsafe { sys::ecs_add_id(self.world_ptr_mut(), *self.id, flecs::meta::EcsEnum::ID) };

        let name = compact_str::format_compact!("{}\0", name);

        let desc = sys::ecs_entity_desc_t {
            name: name.as_ptr() as *const _,
            parent: *self.id,
            ..Default::default()
        };

        let eid = unsafe { sys::ecs_entity_init(self.world_ptr_mut(), &desc) };

        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InternalError,
            "failed to create entity"
        );

        let id = self.world().component_id::<T::UnderlyingTypeOfEnum>();

        let pair = ecs_pair(flecs::Constant::ID, *id);

        unsafe {
            let ptr =
                sys::ecs_ensure_id(self.world_ptr_mut(), eid, pair) as *mut T::UnderlyingTypeOfEnum;
            *ptr = *(&value as *const T as *const <T as ComponentId>::UnderlyingTypeOfEnum);
            sys::ecs_modified_id(self.world_ptr_mut(), eid, pair);
        }
        self
    }
}

impl UntypedComponent<'_> {
    /// Add constant.
    pub fn constant<T: ComponentId>(&self, name: &str, value: T) -> &Self {
        let name = compact_str::format_compact!("{}\0", name);
        let world = self.world_ptr_mut();
        let id = *self.id;

        unsafe { sys::ecs_add_id(world, id, flecs::meta::EcsEnum::ID) };

        let desc = sys::ecs_entity_desc_t {
            name: name.as_ptr() as *const _,
            parent: id,
            ..Default::default()
        };
        let eid = unsafe { sys::ecs_entity_init(world, &desc) };
        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InternalError,
            "failed to create entity"
        );

        unsafe {
            sys::ecs_set_id(
                world,
                eid,
                ecs_pair(flecs::Constant::ID, *self.world.component_id::<T>()),
                core::mem::size_of::<T>(),
                &value as *const T as *const c_void,
            );
        };
        self
    }

    /// Add member with unit.
    ///
    /// [`MetaMember`] is a trait that accepts the following options:
    /// (name : &'static str,),
    /// (name: &'static str, count: i32),
    /// (name: &'static str, count: i32, offset: i32)
    pub fn member_unit<Meta: MetaMember>(
        self,
        type_id: impl IntoEntity,
        unit: impl IntoEntity,
        data: Meta,
    ) -> Self {
        let name = compact_str::format_compact!("{}\0", data.name());
        let world = self.world_ptr_mut();
        let id = *self.id;
        let type_id = *type_id.into_entity(world);
        let unit = *unit.into_entity(world);

        let desc = sys::ecs_entity_desc_t {
            name: name.as_ptr() as *const _,
            parent: id,
            ..Default::default()
        };
        let eid = unsafe { sys::ecs_entity_init(world, &desc) };
        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InternalError,
            "failed to create entity"
        );

        let entity = EntityView::new_from(self.world(), eid);

        let member: sys::EcsMember = sys::EcsMember {
            type_: type_id,
            unit,
            count: data.count(),
            offset: data.offset(),
            use_offset: Meta::USE_OFFSET,
        };

        entity.set(member);
        self
    }

    /// Add member.
    ///
    /// [`MetaMember`] is a trait that accepts the following options:
    /// (name : &'static str,),
    /// (name: &'static str, count: i32),
    /// (name: &'static str, count: i32, offset: i32)
    pub fn member(self, type_id: impl IntoEntity, data: impl MetaMember) -> Self {
        self.member_unit(type_id, 0, data)
    }

    /// Add member with unit typed.
    ///
    /// [`MetaMember`] is a trait that accepts the following options:
    /// (name : &'static str,),
    /// (name: &'static str, count: i32),
    /// (name: &'static str, count: i32, offset: i32)
    pub fn member_unit_type<T: ComponentId, U: ComponentId>(self, data: impl MetaMember) -> Self {
        self.member_unit(T::get_id(self.world()), U::get_id(self.world()), data)
    }

    //TODO

    /*
    /** Add member using pointer-to-member. */
    template <typename MemberType, typename ComponentType, typename RealType = typename core::remove_extent<MemberType>::type>
    untyped_component& member(const char* name, const MemberType ComponentType::* ptr) {
        flecs::entity_t type_id = _::type<RealType>::id(world_);
        size_t offset = reinterpret_cast<size_t>(&(static_cast<ComponentType*>(nullptr)->*ptr));
        return member(type_id, name, core::extent<MemberType>::value, offset);
    }

    /** Add member with unit using pointer-to-member. */
    template <typename MemberType, typename ComponentType, typename RealType = typename core::remove_extent<MemberType>::type>
    untyped_component& member(flecs::entity_t unit, const char* name, const MemberType ComponentType::* ptr) {
        flecs::entity_t type_id = _::type<RealType>::id(world_);
        size_t offset = reinterpret_cast<size_t>(&(static_cast<ComponentType*>(nullptr)->*ptr));
        return member(type_id, unit, name, core::extent<MemberType>::value, offset);
    }

    /** Add member with unit using pointer-to-member. */
    template <typename UnitType, typename MemberType, typename ComponentType, typename RealType = typename core::remove_extent<MemberType>::type>
    untyped_component& member(const char* name, const MemberType ComponentType::* ptr) {
        flecs::entity_t type_id = _::type<RealType>::id(world_);
        flecs::entity_t unit_id = _::type<UnitType>::id(world_);
        size_t offset = reinterpret_cast<size_t>(&(static_cast<ComponentType*>(nullptr)->*ptr));
        return member(type_id, unit_id, name, core::extent<MemberType>::value, offset);
             */

    /// Add bitmask constant
    pub fn bit<T: ComponentId>(self, name: &str, value: T) -> Self {
        let name = compact_str::format_compact!("{}\0", name);
        let world = self.world_ptr_mut();
        let id = *self.id;

        unsafe { sys::ecs_add_id(world, id, flecs::meta::Bitmask::ID) };

        let desc = sys::ecs_entity_desc_t {
            name: name.as_ptr() as *const _,
            parent: id,
            ..Default::default()
        };

        let eid = unsafe { sys::ecs_entity_init(world, &desc) };

        ecs_assert!(
            eid != 0,
            FlecsErrorCode::InternalError,
            "failed to create entity"
        );

        unsafe {
            sys::ecs_set_id(
                world,
                eid,
                ecs_pair(flecs::Constant::ID, *self.world.component_id::<T>()),
                core::mem::size_of::<T>(),
                &value as *const T as *const c_void,
            );
        };
        self
    }

    /// register array metadata for component
    pub fn array<ElemType: ComponentId>(self, elem_count: i32) -> Self {
        let desc = sys::ecs_array_desc_t {
            entity: *self.id,
            type_: ElemType::get_id(self.world()),
            count: elem_count,
        };

        unsafe { sys::ecs_array_init(self.world_ptr_mut(), &desc) };
        self
    }

    /// add member value range
    pub fn range(self, min: f64, max: f64) -> Self {
        let m = unsafe { sys::ecs_cpp_last_member(self.world_ptr(), *self.id) };
        if m.is_null() {
            return self;
        }

        let world_ptr = self.world_ptr_mut();
        let w = unsafe { WorldRef::from_ptr(world_ptr) };
        let me = w.entity_from_id(unsafe { (*m).member });

        let mr = unsafe {
            &mut *(sys::ecs_ensure_id(world_ptr, *me.id, flecs::meta::MemberRanges::ID)
                as *mut flecs::meta::MemberRanges)
        };

        mr.value.min = min;
        mr.value.max = max;
        me.modified(flecs::meta::MemberRanges::ID);
        self
    }

    /// add member warning range
    pub fn warning_range(self, min: f64, max: f64) -> Self {
        let m = unsafe { sys::ecs_cpp_last_member(self.world_ptr(), *self.id) };
        if m.is_null() {
            return self;
        }

        let world_ptr = self.world_ptr_mut();
        let w = unsafe { WorldRef::from_ptr(world_ptr) };
        let me = w.entity_from_id(unsafe { (*m).member });

        let mr = unsafe {
            &mut *(sys::ecs_ensure_id(world_ptr, *me.id, flecs::meta::MemberRanges::ID)
                as *mut flecs::meta::MemberRanges)
        };

        mr.warning.min = min;
        mr.warning.max = max;
        me.modified(flecs::meta::MemberRanges::ID);
        self
    }

    /// add member error range
    pub fn error_range(self, min: f64, max: f64) -> Self {
        let m = unsafe { sys::ecs_cpp_last_member(self.world_ptr(), *self.id) };
        if m.is_null() {
            return self;
        }

        let world_ptr = self.world_ptr_mut();
        let w = unsafe { WorldRef::from_ptr(world_ptr) };
        let me = w.entity_from_id(unsafe { (*m).member });

        let mr = unsafe {
            &mut *(sys::ecs_ensure_id(world_ptr, *me.id, flecs::meta::MemberRanges::ID)
                as *mut flecs::meta::MemberRanges)
        };

        mr.error.min = min;
        mr.error.max = max;
        me.modified(flecs::meta::MemberRanges::ID);
        self
    }
}

pub fn flecs_entity_support<'a>(world: impl WorldProvider<'a>) -> Opaque<'a, Entity> {
    let mut opaque = Opaque::<Entity>::new(world);
    opaque.as_type(flecs::meta::Entity::ID);
    opaque.serialize(|ser: &Serializer, data: &Entity| {
        let id: Id = <Entity as Into<Id>>::into(*data);
        let id: u64 = *id;
        ser.value_id(flecs::meta::Entity::ID, &id as *const u64 as *const c_void)
    });
    opaque.assign_entity(|dst: &mut Entity, _world: WorldRef<'a>, e: Entity| {
        *dst = e;
    });
    opaque
}

impl EntityView<'_> {
    /// Make entity a unit
    pub fn unit(
        &self,
        symbol: Option<&str>,
        prefix: impl Into<Entity>,
        base: impl Into<Entity>,
        over: impl Into<Entity>,
        factor: i32,
        power: i32,
    ) -> &Self {
        if let Some(symbol) = symbol {
            let symbol = compact_str::format_compact!("{}\0", symbol);
            let desc = sys::ecs_unit_desc_t {
                entity: *self.id,
                symbol: symbol.as_ptr() as *const _,
                base: *base.into(),
                over: *over.into(),
                prefix: *prefix.into(),
                translation: sys::ecs_unit_translation_t { factor, power },
                quantity: 0,
            };

            unsafe { sys::ecs_unit_init(self.world_ptr_mut(), &desc) };
        } else {
            let desc = sys::ecs_unit_desc_t {
                entity: *self.id,
                symbol: core::ptr::null(),
                base: *base.into(),
                over: *over.into(),
                prefix: *prefix.into(),
                translation: sys::ecs_unit_translation_t { factor, power },
                quantity: 0,
            };

            unsafe { sys::ecs_unit_init(self.world_ptr_mut(), &desc) };
        }

        self
    }

    /// Make entity an unit prefix
    pub fn unit_prefix(&self, symbol: &str, factor: i32, power: i32) -> &Self {
        let symbol = compact_str::format_compact!("{}\0", symbol);
        let desc = sys::ecs_unit_prefix_desc_t {
            entity: *self.id,
            symbol: symbol.as_ptr() as *const _,
            translation: sys::ecs_unit_translation_t { factor, power },
        };

        unsafe { sys::ecs_unit_prefix_init(self.world_ptr_mut(), &desc) };

        self
    }

    /// Add quantity to unit
    pub fn quantity_id(&self, quantity: impl Into<Entity>) -> &Self {
        unsafe {
            sys::ecs_add_id(
                self.world_ptr_mut(),
                *self.id,
                ecs_pair(flecs::meta::Quantity::ID, *quantity.into()),
            );
        };
        self
    }

    /// Add quantity to unit
    pub fn quantity<T: ComponentId>(&self) -> &Self {
        self.quantity_id(T::get_id(self.world()))
    }

    /// Make entity a quantity
    pub fn quantity_self(&self) -> &Self {
        unsafe { sys::ecs_add_id(self.world_ptr_mut(), *self.id, flecs::meta::Quantity::ID) };
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // pub type SerializeFn<T> = extern "C" fn(*const Serializer, *const T) -> i32;

    #[derive(Debug, Clone, Component)]
    struct Int {
        value: i32,
    }

    // //#[test]
    // fn test_opaque() {
    //     let world = World::new();
    //     world
    //         .component::<Int>()
    //         .opaque::<flecs::meta::I32>()
    //         .serialize(|s: &meta::Serializer, i: &Int| s.value::<i32>(&i.value));

    //     let int_type = Int { value: 10 };

    //     let json = world.to_json::<Int>(&int_type);

    //     println!("{}", json);
    //     assert_eq!("10", json);
    // }

    // #[derive(Component, Default)]
    // struct Position {
    //     x: f32,
    //     y: f32,
    // }

    // //#[test]
    // fn test_expr() {
    //     let world = World::new();

    //     world
    //         .component::<Position>()
    //         .member::<f32>("x", 1, core::mem::offset_of!(Position, x) as i32)
    //         .member::<f32>("y", 1, core::mem::offset_of!(Position, y) as i32);

    //     let e = world.entity().set(Position { x: 10.0, y: 20.0 });

    //     let pos_id = <Position as ComponentId>::id(&world);

    //     // e.get::<&Position>(|pos| {
    //     //     let expr = world.to_expr(pos);
    //     //     println!("{}", expr);
    //     // });
    // }
}
