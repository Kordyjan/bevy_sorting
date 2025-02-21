use std::collections::HashMap;

use assert_unordered::assert_eq_unordered_sort;
use bevy::{
    ecs::schedule::{NodeId, ScheduleGraph},
    prelude::*,
};

use super::InferFlow;

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
    fun: &T,
) -> NodeId {
    graph
        .systems()
        .find(|s| s.1.type_id() == fun.system_type_id())
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

#[derive(Event)]
struct SomeEvent;

#[derive(Component)]
struct SomeData;

#[derive(Component)]
struct OtherData;

fn some_reader_only(_reader: EventReader<SomeEvent>) {}

fn some_writer_only(_writer: EventWriter<SomeEvent>) {}
