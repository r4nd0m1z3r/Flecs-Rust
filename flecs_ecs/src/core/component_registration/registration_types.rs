#![doc(hidden)]
use flecs_ecs_derive::Component;

use super::ComponentId;

pub struct Enum;

pub struct Struct;

#[derive(Component, Clone, Copy)]
#[repr(C)]
pub enum NoneEnum {
    None = 1,
}

#[derive(Default, Clone)]
pub struct FlecsNoneDefaultDummy;

#[derive(Clone)]
pub struct FlecsNoneCloneDummy;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlecsNonePartialOrdDummy;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FlecsNonePartialEqDummy;

pub struct ConditionalTypeSelector<const B: bool, T> {
    phantom: core::marker::PhantomData<T>,
}

pub struct ConditionalTypePairSelector<T, First, Second>
where
    First: ComponentId,
    Second: ComponentId,
{
    phantom: core::marker::PhantomData<(T, First, Second)>,
}
