use std::{any, collections::HashMap, vec};

use assert_unordered::assert_eq_unordered_sort;
use bevy::{
    ecs::schedule::{NodeId, ScheduleGraph},
    prelude::*,
};

use super::{InferFlow, InferFlowEach};

#[test]
fn simple_event_sorting() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            some_reader_only.in_auto_sets(),
            some_writer_only.in_auto_sets(),
        ),
    );

    let graph = app.get_schedule(Update).unwrap().graph();
    let (read_event_set, write_event_set) = find_set_pair(graph, "SomeEvent");
    let reader_only_system = find_system(graph, &some_reader_only);
    let writer_only_system = find_system(graph, &some_writer_only);

    assert_eq_unordered_sort!(
        vec![reader_only_system],
        systems_for_set(graph, read_event_set)
    );
    assert_eq_unordered_sort!(
        vec![writer_only_system],
        systems_for_set(graph, write_event_set)
    );
}

#[test]
fn double_event_sorting() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            some_reader_only.in_auto_sets(),
            two_writers.in_auto_sets(),
            two_readers.in_auto_sets(),
            some_writer_only.in_auto_sets(),
        ),
    );

    let graph = app.get_schedule(Update).unwrap().graph();
    let (read_some_set, write_some_set) = find_set_pair(graph, "SomeEvent");
    let (read_other_set, write_other_set) = find_set_pair(graph, "OtherEvent");
    let reader_only_system = find_system(graph, &some_reader_only);
    let writer_only_system = find_system(graph, &some_writer_only);
    let two_readers_system = find_system(graph, &two_readers);
    let two_writers_system = find_system(graph, &two_writers);

    assert_eq_unordered_sort!(
        vec![reader_only_system, two_readers_system],
        systems_for_set(graph, read_some_set)
    );
    assert_eq_unordered_sort!(
        vec![writer_only_system, two_writers_system],
        systems_for_set(graph, write_some_set)
    );
    assert_eq_unordered_sort!(
        vec![two_readers_system],
        systems_for_set(graph, read_other_set)
    );
    assert_eq_unordered_sort!(
        vec![two_writers_system],
        systems_for_set(graph, write_other_set)
    );
}

#[test]
fn miexed_event_sorting() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            some_reader_only.in_auto_sets(),
            other_writer_only.in_auto_sets(),
            mixed_events.in_auto_sets(),
        ),
    );

    let graph = app.get_schedule(Update).unwrap().graph();
    let (read_some_set, write_some_set) = find_set_pair(graph, "SomeEvent");
    let (read_other_set, write_other_set) = find_set_pair(graph, "OtherEvent");
    let reader_only_system = find_system(graph, &some_reader_only);
    let other_writer_system = find_system(graph, &other_writer_only);
    let mixed_system = find_system(graph, &mixed_events);

    assert_eq_unordered_sort!(
        vec![reader_only_system],
        systems_for_set(graph, read_some_set)
    );
    assert_eq_unordered_sort!(vec![mixed_system], systems_for_set(graph, write_some_set));
    assert_eq_unordered_sort!(vec![mixed_system], systems_for_set(graph, read_other_set));
    assert_eq_unordered_sort!(
        vec![other_writer_system],
        systems_for_set(graph, write_other_set)
    );
}

#[test]
fn commands_do_not_create_autoset() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            commands_only.in_auto_sets(),
            commands_and_reader.in_auto_sets(),
        ),
    );

    let graph = app.get_schedule(Update).unwrap().graph();
    assert_eq!(graph.system_sets().count(), 3);
}

#[test]
fn simple_resources_sorting() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            resource_only.in_auto_sets(),
            resource_mut_only.in_auto_sets(),
        ),
    );

    let graph = app.get_schedule(Update).unwrap().graph();
    let (read_something_set, write_something_set) = find_set_pair(graph, "Something");
    let res_system = find_system(graph, &resource_only);
    let res_mut_system = find_system(graph, &resource_mut_only);

    assert_eq_unordered_sort!(vec![res_system], systems_for_set(graph, read_something_set));

    assert_eq_unordered_sort!(
        vec![res_mut_system],
        systems_for_set(graph, write_something_set)
    )
}

#[test]
fn mixed_resource_sorting() {
    let mut app = App::new();
    app.add_systems(
        Update,
        (
            resource_mixed.in_auto_sets(),
            resource_mut_only.in_auto_sets(),
            other_res_only.in_auto_sets(),
        ),
    );

    let graph = app.get_schedule(Update).unwrap().graph();
    let (read_something_set, write_something_set) = find_set_pair(graph, "Something");
    let (read_other_set, write_other_set) = find_set_pair(graph, "SomethingElse");
    let something_mut_system = find_system(graph, &resource_mut_only);
    let mixed_system = find_system(graph, &resource_mixed);
    let other_system = find_system(graph, &other_res_only);

    assert_eq_unordered_sort!(
        vec![mixed_system],
        systems_for_set(graph, read_something_set)
    );

    assert_eq_unordered_sort!(
        vec![something_mut_system],
        systems_for_set(graph, write_something_set)
    );

    assert_eq_unordered_sort!(vec![other_system], systems_for_set(graph, read_other_set));

    assert_eq_unordered_sort!(vec![mixed_system], systems_for_set(graph, write_other_set));
}

#[test]
fn queries_create_autosets() {
    let mut app = App::new();
    app.add_systems(Update, with_query.in_auto_sets());

    let graph = app.get_schedule(Update).unwrap().graph();
    let read_some_data_set = find_set(graph, "Reads(\"SomeData\")");
    let write_other_data_set = find_set(graph, "Writes(\"OtherData\")");
    let system = find_system(graph, &with_query);

    assert_eq!(3, graph.system_sets().count());

    assert_eq_unordered_sort!(vec![system], systems_for_set(graph, read_some_data_set));

    assert_eq_unordered_sort!(vec![system], systems_for_set(graph, write_other_data_set))
}

#[test]
fn query_filters_influence_autosets() {
    let mut app = App::new();
    app.add_systems(Update, with_query_filter.in_auto_sets());

    let graph = app.get_schedule(Update).unwrap().graph();
    let read_some_data_set = find_set(graph, "Reads(\"SomeData\")");
    let read_other_data_set = find_set(graph, "Reads(\"OtherData\")");
    let read_third_data_set = find_set(graph, "Reads(\"DataNumberThree\")");
    let read_fourth_data_set = find_set(graph, "Reads(\"DataNumberFour\")");
    let system = find_system(graph, &with_query_filter);

    for set in [
        read_some_data_set,
        read_other_data_set,
        read_third_data_set,
        read_fourth_data_set,
    ] {
        assert_eq_unordered_sort!(vec![system], systems_for_set(graph, set));
    }
}

#[test]
fn big_system_test() {
    let mut app = App::new();
    app.add_systems(Update, big_system.in_auto_sets());

    let graph = app.get_schedule(Update).unwrap().graph();
    let read_some_data_set = find_set(graph, "Reads(\"SomeData\")");
    let write_other_data_set = find_set(graph, "Writes(\"OtherData\")");
    let write_third_data_set = find_set(graph, "Writes(\"DataNumberThree\")");
    let read_fourth_data_set = find_set(graph, "Reads(\"DataNumberFour\")");
    let write_res_set = find_set(graph, "Writes(\"Something\")");
    let system = find_system(graph, &big_system);

    for set in [
        read_some_data_set,
        write_other_data_set,
        write_third_data_set,
        read_fourth_data_set,
        write_res_set,
    ] {
        assert_eq_unordered_sort!(vec![system], systems_for_set(graph, set));
    }
}

#[test]
fn sanity_each() {
    (resource_only,).each_in_auto_sets();
    (resource_only, resource_mut_only).each_in_auto_sets();
}

fn find_set(graph: &ScheduleGraph, name: &str) -> NodeId {
    graph
        .system_sets()
        .find(|s| format!("{:?}", s.1) == name)
        .map(|s| s.0)
        .unwrap()
}

fn find_set_pair(graph: &ScheduleGraph, type_name: &str) -> (NodeId, NodeId) {
    (
        find_set(graph, &format!("Reads(\"{type_name}\")")),
        find_set(graph, &format!("Writes(\"{type_name}\")")),
    )
}

fn find_system<I: SystemInput, O, M, T: IntoSystem<I, O, M>>(
    graph: &ScheduleGraph,
    _: &T,
) -> NodeId {
    let name = any::type_name::<T>();
    graph
        .systems()
        .find(|s| s.1.name() == name)
        .map(|s| s.0)
        .unwrap()
}

fn systems_for_set(graph: &ScheduleGraph, set: NodeId) -> Vec<NodeId> {
    graph.hierarchy().graph().neighbors(set).collect()
}

#[allow(dead_code)]
fn debug_graphs(schedule: &Schedule) {
    panic!(
        "\n\n{:?}\n\n{:?}\n\n{:?}\n\n{:?}\n\n",
        schedule
            .graph()
            .system_sets()
            .map(|s| (s.0, s.1))
            .filter(|s| s.1.system_type().is_none())
            .collect::<HashMap<_, _>>(),
        schedule
            .graph()
            .systems()
            .map(|s| (s.0, s.1))
            .collect::<HashMap<_, _>>(),
        schedule.graph().hierarchy().graph(),
        schedule.graph().dependency().graph()
    );
}

#[derive(Resource)]
struct Something;

#[derive(Resource)]
struct SomethingElse;

#[derive(Event)]
struct SomeEvent;

#[derive(Event)]
struct OtherEvent;

#[derive(Component)]
struct SomeData;

#[derive(Component)]
struct OtherData;

#[derive(Component)]
struct DataNumberThree;

#[derive(Component)]
struct DataNumberFour;

fn some_reader_only(_reader: EventReader<SomeEvent>) {}

fn some_writer_only(_writer: EventWriter<SomeEvent>) {}

fn two_readers(_reader: EventReader<SomeEvent>, _reader2: EventReader<OtherEvent>) {}

fn two_writers(_writer: EventWriter<OtherEvent>, _writer2: EventWriter<SomeEvent>) {}

fn other_writer_only(_writer: EventWriter<OtherEvent>) {}

fn mixed_events(_writer: EventWriter<SomeEvent>, _reader: EventReader<OtherEvent>) {}

fn commands_only(_commands: Commands) {}

fn commands_and_reader(_commands: Commands, _reader: EventReader<SomeEvent>) {}

fn resource_only(_resource: Res<Something>) {}

fn resource_mut_only(_resource: ResMut<Something>) {}

fn other_res_only(_resource: Res<SomethingElse>) {}

fn resource_mixed(_read: Res<Something>, _write: ResMut<SomethingElse>) {}

fn with_query(_q: Query<(&SomeData, &mut OtherData, Entity)>) {}

fn with_query_filter(
    _q: Query<
        &SomeData,
        (
            With<OtherData>,
            Or<(Added<DataNumberThree>, Changed<DataNumberFour>)>,
        ),
    >,
) {
}

fn big_system(
    _ps: ParamSet<(
        ParallelCommands,
        Query<(&mut OtherData, &mut DataNumberThree), With<SomeData>>,
        ResMut<Something>,
        Option<Single<(&mut DataNumberThree, &DataNumberFour), Changed<SomeData>>>,
    )>,
) {
}
