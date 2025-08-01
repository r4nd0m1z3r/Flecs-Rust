//! The doc module allows for documenting entities (and thus components, systems)
//! by adding brief and/or detailed descriptions as components. Documentation
//! added with the doc module can be retrieved at runtime, and can be used by
//! tooling such as UIs or documentation frameworks.

use core::ffi::CStr;
use core::ptr::NonNull;

use crate::core::*;
use crate::sys;

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
use alloc::string::{String, ToString};

//MARK: trait::Doc
///
///
/// ```
/// use flecs_ecs::{addons::doc::Doc, core::World, macros::Component};
///
/// #[derive(Component)]
/// struct Tag;
///
/// let world = World::default();
/// world.component::<Tag>().set_doc_name("A tag");
///
/// world
///     .entity()
///     .set_doc_brief("A vast expanse of nothingness.");
/// ```
pub trait Doc<'a>: WorldProvider<'a> + Into<Entity> + Clone {
    //MARK: _getters

    /// Get human readable name for an entity.
    ///
    /// # Returns
    ///
    /// The human readable name of the entity.
    ///
    /// # See also
    ///
    /// * [`World::doc_name()`]
    fn doc_name(&self) -> Option<String> {
        self.world().doc_name(self.clone())
    }

    /// Get brief description for an entity.
    ///
    /// # Returns
    ///
    /// The brief description of the entity.
    ///
    /// # See also
    ///
    /// * [`World::doc_brief()`]
    fn doc_brief(&self) -> Option<String> {
        self.world().doc_brief(self.clone())
    }

    /// Get detailed description for an entity.
    ///
    /// # Returns
    ///
    /// The detailed description of the entity.
    ///
    /// # See also
    ///
    /// * [`World::doc_detail()`]
    fn doc_detail(&self) -> Option<String> {
        self.world().doc_detail(self.clone())
    }

    /// Get link to external documentation for an entity.
    ///
    /// # Returns
    ///
    /// The link to external documentation of the entity.
    ///
    /// # See also
    ///
    /// * [`World::doc_link()`]
    fn doc_link(&self) -> Option<String> {
        self.world().doc_link(self.clone())
    }

    /// Get color for an entity.
    ///
    /// # Returns
    ///
    /// The color of the entity.
    ///
    /// # See also
    ///
    /// * [`World::doc_color()`]
    fn doc_color(&self) -> Option<String> {
        self.world().doc_color(self.clone())
    }

    /// Get UUID for entity
    ///
    /// # Returns
    ///
    /// The UUID of the entity.
    ///
    /// # See also
    ///
    /// * [`World::doc_uuid()`]
    /// * [`Doc::set_doc_uuid()`]
    /// * [`World::set_doc_uuid()`]
    fn doc_uuid(&self) -> Option<String> {
        self.world().doc_uuid(self.clone())
    }

    //MARK: _setters

    /// Add human-readable name to entity.
    ///
    /// Contrary to entity names, human readable names do not have to be unique and
    /// can contain special characters used in the query language like '*'.
    ///
    /// # Arguments
    ///
    /// * `name` - The name to add.
    ///
    /// # See also
    ///
    /// * [`World::set_doc_name()`]
    fn set_doc_name(&self, name: &str) -> &Self {
        self.world().set_doc_name(self.clone(), name);
        self
    }

    /// Add brief description to entity.
    ///
    /// # Arguments
    ///
    /// * `brief` - The brief description to add.
    ///
    /// # See also
    ///
    /// * [`World::set_doc_brief()`]
    fn set_doc_brief(&self, brief: &str) -> &Self {
        self.world().set_doc_brief(self.clone(), brief);
        self
    }

    /// Add detailed description to entity.
    ///
    /// # Arguments
    ///
    /// * `detail` - The detailed description to add.
    ///
    /// # See also
    ///
    /// * [`World::set_doc_detail()`]
    fn set_doc_detail(&self, detail: &str) -> &Self {
        self.world().set_doc_detail(self.clone(), detail);
        self
    }

    /// Add link to external documentation to entity.
    ///
    /// # Arguments
    ///
    /// * `link` - The link to add.
    ///
    /// # See also
    ///
    /// * [`World::set_doc_link()`]
    fn set_doc_link(&self, link: &str) -> &Self {
        self.world().set_doc_link(self.clone(), link);
        self
    }

    /// Add color to entity.
    ///
    /// UIs can use color as hint to improve visualizing entities.
    ///
    /// # Arguments
    ///
    /// * `world` - The world.
    /// * `color` - The color to add.
    ///
    /// # See also
    ///
    /// * [`World::set_doc_color()`]
    fn set_doc_color(&self, color: &str) -> &Self {
        self.world().set_doc_color(self.clone(), color);
        self
    }

    /// Set doc UUID.
    /// This adds `(flecs.doc.Description, flecs.doc.Uuid)` to the entity.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The UUID to add.
    ///
    /// # See also
    /// * [`World::set_doc_uuid()`]
    /// * [`World::doc_uuid()`]
    /// * [`Doc::doc_uuid()`]
    fn set_doc_uuid(&self, uuid: &str) -> &Self {
        self.world().set_doc_uuid(self.clone(), uuid);
        self
    }
}

impl<'a, T> Doc<'a> for T where T: Into<Entity> + WorldProvider<'a> + Clone {}

//MARK: impl World
/// ```
/// use flecs_ecs::prelude::*;
///
/// #[derive(Component)]
/// struct Tag;
///
/// let world = World::default();
/// world.component::<Tag>();
/// world.set_doc_name(Tag, "A tag");
/// ```
impl World {
    //MARK: _World::getters

    /// Get human readable name for an entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to get the human readable name.
    ///
    /// # Returns
    ///
    /// The human readable name of the entity.
    ///
    /// # See also
    ///
    /// * [`Doc::doc_name()`]
    /// * [`World::doc_name()`]
    #[inline(always)]
    pub fn doc_name(&self, entity: impl IntoEntity) -> Option<String> {
        let cstr = NonNull::new(unsafe {
            sys::ecs_doc_get_name(self.world_ptr(), *entity.into_entity(self))
        } as *mut _)
        .map(|s| unsafe { CStr::from_ptr(s.as_ptr()) });
        cstr.and_then(|s| s.to_str().ok().map(ToString::to_string))
    }

    /// Get brief description for an entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to get the brief description
    ///
    /// # Returns
    ///
    /// The brief description of the entity.
    ///
    /// # See also
    ///
    /// * [`Doc::doc_brief()`]
    /// * [`World::doc_brief()`]
    #[inline(always)]
    pub fn doc_brief(&self, entity: impl IntoEntity) -> Option<String> {
        let cstr = NonNull::new(unsafe {
            sys::ecs_doc_get_brief(self.world_ptr(), *entity.into_entity(self))
        } as *mut _)
        .map(|s| unsafe { CStr::from_ptr(s.as_ptr()) });
        cstr.and_then(|s| s.to_str().ok().map(ToString::to_string))
    }

    /// Get detailed description for an entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity for which to get the detailed description.
    ///
    /// # Returns
    ///
    /// The detailed description of the entity.
    ///
    /// # See also
    ///
    /// * [`Doc::doc_detail()`]
    /// * [`World::doc_detail()`]
    #[inline(always)]
    pub fn doc_detail(&self, entity: impl IntoEntity) -> Option<String> {
        let cstr = NonNull::new(unsafe {
            sys::ecs_doc_get_detail(self.world_ptr(), *entity.into_entity(self))
        } as *mut _)
        .map(|s| unsafe { CStr::from_ptr(s.as_ptr()) });
        cstr.and_then(|s| s.to_str().ok().map(ToString::to_string))
    }

    /// Get link to external documentation for an entity.
    /// # Arguments
    ///
    /// * `entity` - The entity for which to get the link to external documentation.
    ///
    /// # Returns
    ///
    /// The link to external documentation of the entity.
    ///
    /// # See also
    ///
    /// * [`Doc::doc_link()`]
    /// * [`World::doc_link()`]
    #[inline(always)]
    pub fn doc_link(&self, entity: impl IntoEntity) -> Option<String> {
        let cstr = NonNull::new(unsafe {
            sys::ecs_doc_get_link(self.world_ptr(), *entity.into_entity(self))
        } as *mut _)
        .map(|s| unsafe { CStr::from_ptr(s.as_ptr()) });
        cstr.and_then(|s| s.to_str().ok().map(ToString::to_string))
    }

    /// Get color for an entity.
    /// # Arguments
    ///
    /// * `entity` - The entity for which to get the color.
    ///
    /// # Returns
    ///
    /// The color of the entity.
    ///
    /// # See also
    ///
    /// * [`Doc::doc_color()`]
    /// * [`World::doc_color()`]
    #[inline(always)]
    pub fn doc_color(&self, entity: impl IntoEntity) -> Option<String> {
        let cstr = NonNull::new(unsafe {
            sys::ecs_doc_get_color(self.world_ptr(), *entity.into_entity(self))
        } as *mut _)
        .map(|s| unsafe { CStr::from_ptr(s.as_ptr()) });
        cstr.and_then(|s| s.to_str().ok().map(ToString::to_string))
    }

    /// Get UUID for entity
    ///
    /// # See Also
    ///
    /// * [`World::doc_uuid()`]
    /// * [`Doc::doc_uuid()`]
    /// * [`Doc::set_doc_uuid()`]
    pub fn doc_uuid(&self, entity: impl IntoEntity) -> Option<String> {
        let cstr = NonNull::new(unsafe {
            sys::ecs_doc_get_uuid(self.world_ptr(), *entity.into_entity(self))
        } as *mut _)
        .map(|s| unsafe { CStr::from_ptr(s.as_ptr()) });
        cstr.and_then(|s| s.to_str().ok().map(ToString::to_string))
    }

    //MARK: _World::setters

    /// Add human-readable name to entity.
    ///
    /// Contrary to entity names, human readable names do not have to be unique and
    /// can contain special characters used in the query language like '*'.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which to add the name.
    /// * `name` - The name to add.
    ///
    /// # See also
    ///
    /// * [`Doc::set_doc_name()`]
    /// * [`World::set_doc_name()`]
    #[inline(always)]
    pub fn set_doc_name(&self, entity: impl IntoEntity, name: &str) {
        let name = compact_str::format_compact!("{}\0", name);
        unsafe {
            sys::ecs_doc_set_name(
                self.ptr_mut(),
                *entity.into_entity(self),
                name.as_ptr() as *const _,
            );
        };
    }

    /// Add brief description to entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which to add the brief description.
    /// * `brief` - The brief description to add.
    ///
    /// # See also
    ///
    /// * [`Doc::set_doc_brief()`]
    /// * [`World::set_doc_brief()`]
    #[inline(always)]
    pub fn set_doc_brief(&self, entity: impl IntoEntity, brief: &str) {
        let brief = compact_str::format_compact!("{}\0", brief);
        unsafe {
            sys::ecs_doc_set_brief(
                self.ptr_mut(),
                *entity.into_entity(self),
                brief.as_ptr() as *const _,
            );
        };
    }

    /// Add detailed description to entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which to add the detailed description.
    /// * `detail` - The detailed description to add.
    ///
    /// # See also
    ///
    /// * [`Doc::set_doc_detail()`]
    /// * [`World::set_doc_detail()`]
    #[inline(always)]
    pub fn set_doc_detail(&self, entity: impl IntoEntity, detail: &str) {
        let detail = compact_str::format_compact!("{}\0", detail);
        unsafe {
            sys::ecs_doc_set_detail(
                self.ptr_mut(),
                *entity.into_entity(self),
                detail.as_ptr() as *const _,
            );
        };
    }

    /// Add link to external documentation to entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which to add the link.
    /// * `link` - The link to add.
    ///
    /// # See also
    ///
    /// * [`Doc::set_doc_link()`]
    /// * [`World::set_doc_link()`]
    #[inline(always)]
    pub fn set_doc_link(&self, entity: impl IntoEntity, link: &str) {
        let link = compact_str::format_compact!("{}\0", link);
        unsafe {
            sys::ecs_doc_set_link(
                self.ptr_mut(),
                *entity.into_entity(self),
                link.as_ptr() as *const _,
            );
        };
    }

    /// Add color to entity.
    ///
    /// UIs can use color as hint to improve visualizing entities.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which to add the color.
    /// * `color` - The color to add.
    ///
    /// # See also
    ///
    /// * [`Doc::set_doc_color()`]
    /// * [`World::set_doc_color()`]
    #[inline(always)]
    pub fn set_doc_color(&self, entity: impl IntoEntity, color: &str) {
        let color = compact_str::format_compact!("{}\0", color);
        unsafe {
            sys::ecs_doc_set_color(
                self.ptr_mut(),
                *entity.into_entity(self),
                color.as_ptr() as *const _,
            );
        };
    }

    /// Add UUID to entity.
    /// This adds `(flecs.doc.Description, flecs.doc.Uuid)` to the entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to which to add the UUID.
    /// * `uuid` - The UUID to add.
    ///
    /// # See also
    ///
    /// * [`Doc::set_doc_uuid()`]
    /// * [`World::set_doc_uuid()`]
    /// * [`World::doc_uuid()`]
    /// * [`Doc::doc_uuid()`]
    pub fn set_doc_uuid(&self, entity: impl IntoEntity, uuid: &str) {
        let uuid = compact_str::format_compact!("{}\0", uuid);
        unsafe {
            sys::ecs_doc_set_uuid(
                self.ptr_mut(),
                *entity.into_entity(self),
                uuid.as_ptr() as *const _,
            );
        };
    }
}

#[test]
fn test_compile_doc() {
    #[derive(flecs_ecs_derive::Component)]
    struct Tag;

    let world = World::new();

    let entity = world.entity();
    entity.set_doc_name("name");

    let query = world.query::<()>().set_cached().build();
    query.set_doc_name("name");

    let system = world.system::<()>().build();
    system.set_doc_name("name");

    let observer = world.observer::<flecs::OnAdd, ()>().with(Tag).run(|_| {});
    observer.set_doc_name("name");

    let comp = world.component::<()>();
    comp.set_doc_name("name").set_doc_brief("Unit");
}
