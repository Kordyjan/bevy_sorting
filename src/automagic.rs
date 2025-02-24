use std::marker::PhantomData;

use bevy::{
    diagnostic::Diagnostics,
    ecs::{
        archetype::Archetypes,
        bundle::Bundles,
        component::{ComponentIdFor, Components},
        entity::Entities,
        removal_detection::RemovedComponentEvents,
        schedule::SystemConfigs,
        system::{DynSystemParam, SystemBuffer, SystemChangeTick, SystemName},
        world::{DeferredWorld, WorldId},
    },
    prelude::{
        Commands, Component, Deferred, Event, EventReader, EventWriter, FilteredResources,
        FilteredResourcesMut, FromWorld, IntoSystemConfigs, Local, MeshRayCast, NonSend,
        NonSendMut, ParallelCommands, PickingEventWriters, RemovedComponents, Res, ResMut,
        Resource, SystemParamFunction, TransformHelper, World,
    },
    render::texture::FallbackImageMsaa,
    ui::{
        experimental::{UiChildren, UiRootNodes},
        DefaultUiCamera,
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
