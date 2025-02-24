use std::marker::PhantomData;

use bevy::{
    core::{Name, NameOrEntity},
    diagnostic::Diagnostics,
    ecs::{
        archetype::{Archetype, Archetypes},
        bundle::Bundles,
        component::{ComponentIdFor, Components},
        entity::{Entities, EntityLocation},
        query::QueryData,
        removal_detection::RemovedComponentEvents,
        schedule::SystemConfigs,
        system::{DynSystemParam, SystemBuffer, SystemChangeTick, SystemName},
        world::{
            DeferredWorld, EntityMutExcept, EntityRefExcept, FilteredEntityMut, FilteredEntityRef,
            WorldId,
        },
    },
    prelude::{
        Bundle, Commands, Component, Deferred, Entity, EntityMut, EntityRef, Event, EventReader,
        EventWriter, FilteredResources, FilteredResourcesMut, FromWorld, IntoSystemConfigs, Local,
        MeshRayCast, NonSend, NonSendMut, ParallelCommands, PickingEventWriters, Query,
        RemovedComponents, Res, ResMut, Resource, SystemParamFunction, TransformHelper, World,
    },
    render::{
        sync_world::{MainEntity, RenderEntity},
        texture::FallbackImageMsaa,
    },
    ui::{
        self,
        experimental::{UiChildren, UiRootNodes},
        picking_backend, DefaultUiCamera,
    },
};

use crate::IntoSystemRW;

use impl_trait_for_tuples::impl_for_tuples;

#[cfg(test)]
mod tests;

trait AutoSetArg {
    fn apply(sys: SystemConfigs) -> SystemConfigs;
}

impl<E: Event> AutoSetArg for EventReader<'_, '_, E> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<E>()
    }
}

impl<E: Event> AutoSetArg for EventWriter<'_, E> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<E>()
    }
}

impl<R: Resource> AutoSetArg for Res<'_, R> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<R>()
    }
}

impl<R: Resource> AutoSetArg for ResMut<'_, R> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<R>()
    }
}

impl<T: 'static> AutoSetArg for NonSend<'_, T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<T>()
    }
}

impl<T: 'static> AutoSetArg for NonSendMut<'_, T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<T>()
    }
}

impl<T: AutoSetArg> AutoSetArg for Option<T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        T::apply(sys)
    }
}

trait AutoSetArgInQuery {
    fn apply(sys: SystemConfigs) -> SystemConfigs;
}

impl<T: Component> AutoSetArgInQuery for &T {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<T>()
    }
}

impl<T: Component> AutoSetArgInQuery for &mut T {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<T>()
    }
}

impl<T: AutoSetArgInQuery> AutoSetArgInQuery for Option<T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        T::apply(sys)
    }
}

impl AutoSetArgInQuery for NameOrEntity {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<Name>()
    }
}

impl<T> AutoSetArgInQuery for PhantomData<T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for Entity {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for MainEntity {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for RenderEntity {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for picking_backend::NodeQuery {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for picking_backend::NodeQueryReadOnly {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for ui::NodeQuery {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for ui::NodeQueryReadOnly {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for EntityLocation {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for EntityMut<'_> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for EntityRef<'_> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for FilteredEntityMut<'_> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl AutoSetArgInQuery for FilteredEntityRef<'_> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl<B: Bundle> AutoSetArgInQuery for EntityMutExcept<'_, B> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

impl<B: Bundle> AutoSetArgInQuery for EntityRefExcept<'_, B> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

#[allow(clippy::let_and_return)]
#[impl_for_tuples(0, 15)]
impl AutoSetArgInQuery for Tuple {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        for_tuples!( #( let sys = <Tuple as AutoSetArgInQuery>::apply(sys); )* );
        sys
    }
}

impl<D: AutoSetArgInQuery + QueryData> AutoSetArg for Query<'_, '_, D> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        D::apply(sys)
    }
}

trait NoInfer {}

impl NoInfer for Commands<'_, '_> {}

impl<T> NoInfer for PhantomData<T> {}

impl<T> NoInfer for Vec<T> {}

impl NoInfer for &World {}

impl NoInfer for Diagnostics<'_, '_> {}

impl NoInfer for DefaultUiCamera<'_, '_> {}

impl NoInfer for FilteredResources<'_, '_> {}

impl NoInfer for FilteredResourcesMut<'_, '_> {}

impl NoInfer for MeshRayCast<'_, '_> {}

impl NoInfer for ParallelCommands<'_, '_> {}

impl NoInfer for PickingEventWriters<'_> {}

impl NoInfer for TransformHelper<'_, '_> {}

impl NoInfer for FallbackImageMsaa<'_> {}

impl NoInfer for UiChildren<'_, '_> {}

impl NoInfer for UiRootNodes<'_, '_> {}

impl NoInfer for WorldId {}

impl NoInfer for DynSystemParam<'_, '_> {}

impl NoInfer for SystemChangeTick {}

impl NoInfer for SystemName<'_> {}

impl NoInfer for &Archetypes {}

impl NoInfer for &Bundles {}

impl NoInfer for &Components {}

impl NoInfer for &Entities {}

impl NoInfer for &RemovedComponentEvents {}

impl<T: FromWorld + Send> NoInfer for Local<'_, T> {}

impl NoInfer for DeferredWorld<'_> {}

impl<T: SystemBuffer> NoInfer for Deferred<'_, T> {}

impl<T: Component> NoInfer for RemovedComponents<'_, '_, T> {}

impl<T: Component> NoInfer for ComponentIdFor<'_, T> {}

impl<T: NoInfer> AutoSetArg for T {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys
    }
}

#[allow(clippy::let_and_return)]
#[impl_for_tuples(0, 15)]
impl AutoSetArg for Tuple {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        for_tuples!( #( let sys = <Tuple as AutoSetArg>::apply(sys); )* );
        sys
    }
}

trait InferFlow<Marker> {
    fn in_auto_sets(self) -> SystemConfigs;
}

impl<T, M> InferFlow<M> for T
where
    T::Param: AutoSetArg,
    T: SystemParamFunction<M, In = (), Out = ()>,
    M: 'static,
{
    fn in_auto_sets(self) -> SystemConfigs {
        <T::Param as AutoSetArg>::apply(self.into_configs())
    }
}
