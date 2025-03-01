use std::marker::PhantomData;

use bevy::{
    core::{Name, NameOrEntity},
    diagnostic::Diagnostics,
    ecs::{
        archetype::Archetypes,
        bundle::Bundles,
        component::{ComponentIdFor, Components},
        entity::{Entities, EntityLocation},
        query::{QueryData, QueryFilter},
        removal_detection::RemovedComponentEvents,
        schedule::SystemConfigs,
        system::{DynSystemParam, SystemBuffer, SystemChangeTick, SystemName, SystemParam},
        world::{
            DeferredWorld, EntityMutExcept, EntityRefExcept, FilteredEntityMut, FilteredEntityRef,
            WorldId,
        },
    },
    prelude::{
        Added, AnyOf, Bundle, Changed, Commands, Component, Deferred, Entity, EntityMut, EntityRef,
        Event, EventReader, EventWriter, FilteredResources, FilteredResourcesMut, FromWorld, Has,
        IntoSystemConfigs, Local, MeshRayCast, Mut, NonSend, NonSendMut, Or, ParallelCommands,
        ParamSet, PickingEventWriters, Populated, Query, Ref, RemovedComponents, Res, ResMut,
        Resource, Single, SystemParamFunction, TransformHelper, With, Without, World,
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
use bevy_utils_proc_macros::all_tuples;

use crate::prelude::IntoSystemRW;

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

impl<T: Component> AutoSetArgInQuery for Ref<'_, T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<T>()
    }
}

impl<T: Component> AutoSetArgInQuery for &mut T {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<T>()
    }
}

impl<T: Component> AutoSetArgInQuery for Mut<'_, T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.writes::<T>()
    }
}

impl<T: Component> AutoSetArgInQuery for Has<T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<T>()
    }
}

impl<T: AutoSetArgInQuery> AutoSetArgInQuery for Option<T> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        T::apply(sys)
    }
}

impl<T: AutoSetArgInQuery> AutoSetArgInQuery for AnyOf<T> {
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

trait AutoSetArgInQueryFilter {
    fn apply(sys: SystemConfigs) -> SystemConfigs;
}

impl<F: AutoSetArgInQueryFilter> AutoSetArgInQueryFilter for Or<F> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        F::apply(sys)
    }
}

impl<C: Component> AutoSetArgInQueryFilter for Added<C> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<C>()
    }
}

impl<C: Component> AutoSetArgInQueryFilter for Changed<C> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<C>()
    }
}

impl<C: Component> AutoSetArgInQueryFilter for With<C> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<C>()
    }
}

impl<C: Component> AutoSetArgInQueryFilter for Without<C> {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        sys.reads::<C>()
    }
}

#[allow(clippy::let_and_return)]
#[impl_for_tuples(0, 15)]
impl AutoSetArgInQueryFilter for Tuple {
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        for_tuples!( #( let sys = <Tuple as AutoSetArgInQueryFilter>::apply(sys); )* );
        sys
    }
}

impl<D: AutoSetArgInQuery + QueryData, F: AutoSetArgInQueryFilter + QueryFilter> AutoSetArg
    for Query<'_, '_, D, F>
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = D::apply(sys);
        F::apply(sys)
    }
}

impl<D: AutoSetArgInQuery + QueryData, F: AutoSetArgInQueryFilter + QueryFilter> AutoSetArg
    for Single<'_, D, F>
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = D::apply(sys);
        F::apply(sys)
    }
}

impl<D: AutoSetArgInQuery + QueryData, F: AutoSetArgInQueryFilter + QueryFilter> AutoSetArg
    for Populated<'_, '_, D, F>
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = D::apply(sys);
        F::apply(sys)
    }
}

impl<P0> AutoSetArg for ParamSet<'_, '_, (P0,)>
where
    P0: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        <P0 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1> AutoSetArg for ParamSet<'_, '_, (P0, P1)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        <P1 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1, P2> AutoSetArg for ParamSet<'_, '_, (P0, P1, P2)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
    P2: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        let sys = <P1 as AutoSetArg>::apply(sys);
        <P2 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1, P2, P3> AutoSetArg for ParamSet<'_, '_, (P0, P1, P2, P3)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
    P2: SystemParam + AutoSetArg,
    P3: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        let sys = <P1 as AutoSetArg>::apply(sys);
        let sys = <P2 as AutoSetArg>::apply(sys);
        <P3 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1, P2, P3, P4> AutoSetArg for ParamSet<'_, '_, (P0, P1, P2, P3, P4)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
    P2: SystemParam + AutoSetArg,
    P3: SystemParam + AutoSetArg,
    P4: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        let sys = <P1 as AutoSetArg>::apply(sys);
        let sys = <P2 as AutoSetArg>::apply(sys);
        let sys = <P3 as AutoSetArg>::apply(sys);
        <P4 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1, P2, P3, P4, P5> AutoSetArg for ParamSet<'_, '_, (P0, P1, P2, P3, P4, P5)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
    P2: SystemParam + AutoSetArg,
    P3: SystemParam + AutoSetArg,
    P4: SystemParam + AutoSetArg,
    P5: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        let sys = <P1 as AutoSetArg>::apply(sys);
        let sys = <P2 as AutoSetArg>::apply(sys);
        let sys = <P3 as AutoSetArg>::apply(sys);
        let sys = <P4 as AutoSetArg>::apply(sys);
        <P5 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1, P2, P3, P4, P5, P6> AutoSetArg for ParamSet<'_, '_, (P0, P1, P2, P3, P4, P5, P6)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
    P2: SystemParam + AutoSetArg,
    P3: SystemParam + AutoSetArg,
    P4: SystemParam + AutoSetArg,
    P5: SystemParam + AutoSetArg,
    P6: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        let sys = <P1 as AutoSetArg>::apply(sys);
        let sys = <P2 as AutoSetArg>::apply(sys);
        let sys = <P3 as AutoSetArg>::apply(sys);
        let sys = <P4 as AutoSetArg>::apply(sys);
        let sys = <P5 as AutoSetArg>::apply(sys);
        <P6 as AutoSetArg>::apply(sys)
    }
}

impl<P0, P1, P2, P3, P4, P5, P6, P7> AutoSetArg
    for ParamSet<'_, '_, (P0, P1, P2, P3, P4, P5, P6, P7)>
where
    P0: SystemParam + AutoSetArg,
    P1: SystemParam + AutoSetArg,
    P2: SystemParam + AutoSetArg,
    P3: SystemParam + AutoSetArg,
    P4: SystemParam + AutoSetArg,
    P5: SystemParam + AutoSetArg,
    P6: SystemParam + AutoSetArg,
    P7: SystemParam + AutoSetArg,
{
    fn apply(sys: SystemConfigs) -> SystemConfigs {
        let sys = <P0 as AutoSetArg>::apply(sys);
        let sys = <P1 as AutoSetArg>::apply(sys);
        let sys = <P2 as AutoSetArg>::apply(sys);
        let sys = <P3 as AutoSetArg>::apply(sys);
        let sys = <P4 as AutoSetArg>::apply(sys);
        let sys = <P5 as AutoSetArg>::apply(sys);
        let sys = <P6 as AutoSetArg>::apply(sys);
        <P7 as AutoSetArg>::apply(sys)
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

pub trait InferFlow<Marker> {
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

pub trait InferFlowEach<Marker> {
    type After;
    fn each_in_auto_sets(self) -> Self::After;
}

macro_rules! impl_each {
    ( $(($marker: ident, $sys: ident)),* ) => {
        impl <$($marker, $sys),*> InferFlowEach<( $($marker, )* )> for ( $($sys,)* ) 
            where $( $sys: InferFlow<$marker> ),* 
        {
            type After = ( $( impl_each!(@HELPER $sys), )* );

            #[allow(non_snake_case)]
            fn each_in_auto_sets(self) -> Self::After {
                let ( $( $sys, )* ) = self;
                ( $( $sys.in_auto_sets(), )* )
            }
        }
    };

    (@HELPER $discard: ident ) => { SystemConfigs };
}

all_tuples!(impl_each, 1, 32, M, S);
