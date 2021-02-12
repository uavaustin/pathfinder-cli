use pathfinder::*;
use std::collections::linked_list::LinkedList;

// Copied from pathfinder/tests/util.rs
fn output_result<T>(
    _waypoints: LinkedList<Waypoint<T>>,
    result: LinkedList<Waypoint<T>>,
    plane: Plane,
) {
    let mut iter = result.iter();
    eprintln!(
        "{:.5}, {:.5}",
        plane.location.lat_degree(),
        plane.location.lon_degree(),
        // plane.location.alt()
    );

    while let Some(node) = iter.next() {
        eprintln!(
            "{:.5}, {:.5}",
            node.location.lat_degree(),
            node.location.lon_degree(),
        );
    }

    println!();
}

// Copied from pathfinder/tests/util.rs
fn vec_to_list<T>(waypoints: Vec<Waypoint<T>>) -> LinkedList<Waypoint<T>> {
    let mut list = LinkedList::new();
    for wp in waypoints {
        list.push_back(wp);
    }
    list
}

fn main() {
    // Code snatched from pathfinder/tests/full_test_2.rs
    let flyzone = vec![vec![
        Location::from_degrees(30.276450732764616, -97.74291515350342, 0f32),
        Location::from_degrees(30.276450732764616, -97.7239465713501, 0f32),
        Location::from_degrees(30.29294185380876, -97.7239465713501, 0f32),
        Location::from_degrees(30.29294185380876, -97.74291515350342, 0f32),
    ]];
    let obstacles = vec![Obstacle::from_degrees(
        30.286975723301133,
        -97.7305555343628,
        50f32,
        50f32,
    )];
    let waypoints = vec_to_list::<()>(vec![Waypoint::from_degrees(
        30.287401, -97.726685, 100f32, 10f32,
    )]);

    let mut pathfinder = Pathfinder::new(Tanstar::new(), TConfig::default(), flyzone, obstacles);
    let plane = Plane::from_degrees(30.288105, -97.73533, 10.0);
    let result = pathfinder.get_adjust_path(plane.clone(), waypoints.clone());

    println!("\nresult is not empty: {}", !result.is_empty());
}
