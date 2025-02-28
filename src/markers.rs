use std::{
    any::{type_name, TypeId},
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::{ecs::label, prelude::SystemSet};

/// System set marking all systems that reads value of T
pub struct Reads<T: 'static>(PhantomData<fn() -> T>);

impl<T: 'static> Default for Reads<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Debug for Reads<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Reads").field(&type_name::<T>()).finish()
    }
}

impl<T> Hash for Reads<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl<T> Clone for Reads<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Reads<T> {}

impl<T> PartialEq for Reads<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Reads<T> {}

impl<T> SystemSet for Reads<T> {
    fn dyn_clone(&self) -> Box<dyn SystemSet> {
        Box::new(*self)
    }

    fn as_dyn_eq(&self) -> &dyn label::DynEq {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        TypeId::of::<Self>().hash(&mut state);
    }
}

/// System set for all systems that writes to T
pub struct Writes<T: 'static>(PhantomData<fn() -> T>);

impl<T: 'static> Default for Writes<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Debug for Writes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Writes").field(&type_name::<T>()).finish()
    }
}

impl<T> Hash for Writes<T> {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl<T> Clone for Writes<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Writes<T> {}

impl<T> PartialEq for Writes<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Writes<T> {}

impl<T> SystemSet for Writes<T> {
    fn dyn_clone(&self) -> Box<dyn SystemSet> {
        Box::new(*self)
    }

    fn as_dyn_eq(&self) -> &dyn label::DynEq {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        TypeId::of::<Self>().hash(&mut state);
    }
}

/// Extension trait for systems allowing to clearly specify read and write constraint
pub trait IntoSystemRW<M>: IntoSystemConfigs<M> {
    /// Specifies that system reads from T
    fn reads<T: 'static>(self) -> NodeConfigs<Box<dyn System<In = (), Out = ()>>> {
        self.in_set(Reads::<T>::default())
    }

    /// Specifies that system writes to T
    fn writes<T: 'static>(self) -> NodeConfigs<Box<dyn System<In = (), Out = ()>>> {
        self.in_set(Writes::<T>::default())
    }
}

impl<M, S: IntoSystemConfigs<M>> IntoSystemRW<M> for S {}
