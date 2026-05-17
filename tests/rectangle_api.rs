use iron_canvas::{
    CENTER, Vertex, cube_face_vertex_count, rectangle_vertices, rotate_rectangle_y,
    rotate_rectangle_z, rotate_y, rotate_z,
};

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.000_01,
        "expected {expected}, got {actual}",
    );
}

fn assert_vertex_close(actual: Vertex, expected: [f32; 3]) {
    assert_close(actual.position[0], expected[0]);
    assert_close(actual.position[1], expected[1]);
    assert_close(actual.position[2], expected[2]);
}

#[test]
fn compatibility_vertices_are_cube_edge_pairs_with_expected_count() {
    let vertices = rectangle_vertices();

    assert_eq!(vertices.len(), 24);
    assert_eq!(vertices.first().unwrap().position, [0.0, 0.0, 0.0]);
    assert_eq!(vertices.last().unwrap().position, [2.0, 0.0, 2.0]);
}

#[test]
fn cube_edge_vertices_use_expected_bounds() {
    let vertices = rectangle_vertices();
    let xs = vertices.iter().map(|vertex| vertex.position[0]);
    let ys = vertices.iter().map(|vertex| vertex.position[1]);
    let zs = vertices.iter().map(|vertex| vertex.position[2]);

    assert_close(xs.clone().fold(f32::INFINITY, f32::min), 0.0);
    assert_close(xs.fold(f32::NEG_INFINITY, f32::max), 2.0);
    assert_close(ys.clone().fold(f32::INFINITY, f32::min), 0.0);
    assert_close(ys.fold(f32::NEG_INFINITY, f32::max), 2.0);
    assert_close(zs.clone().fold(f32::INFINITY, f32::min), 0.0);
    assert_close(zs.fold(f32::NEG_INFINITY, f32::max), 2.0);
}

#[test]
fn cube_faces_are_exported_as_six_quads() {
    assert_eq!(cube_face_vertex_count(), 24);
}

#[test]
fn rotate_z_mutates_cube_edge_vertices_at_runtime() {
    let mut vertices = rectangle_vertices().to_vec();

    rotate_rectangle_z(vertices.as_mut_ptr(), vertices.len(), 90.0);

    assert_vertex_close(vertices[0], [0.0, 2.0, 0.0]);
    assert_vertex_close(vertices[5], [2.0, 0.0, 0.0]);
}

#[test]
fn rotate_y_mutates_depth_for_whole_cube_turns() {
    let mut vertices = rectangle_vertices().to_vec();

    rotate_rectangle_y(vertices.as_mut_ptr(), vertices.len(), 90.0);

    assert_vertex_close(vertices[0], [2.0, 0.0, 0.0]);
    assert_vertex_close(vertices[3], [2.0, 2.0, 2.0]);
}

#[test]
fn four_quarter_turns_return_to_original_vertices() {
    let mut vertices = rectangle_vertices().to_vec();

    for _ in 0..4 {
        rotate_rectangle_z(vertices.as_mut_ptr(), vertices.len(), 90.0);
    }

    for (actual, expected) in vertices.iter().zip(rectangle_vertices()) {
        assert_vertex_close(*actual, expected.position);
    }
}

#[test]
fn rotate_z_preserves_distance_from_center() {
    let position = [2.0, 2.0, 0.0];
    let rotated = rotate_z(position, CENTER, 45.0);
    let original_distance = (position[0] - CENTER[0]).hypot(position[1] - CENTER[1]);
    let rotated_distance = (rotated[0] - CENTER[0]).hypot(rotated[1] - CENTER[1]);

    assert_close(rotated_distance, original_distance);
}

#[test]
fn rotate_y_preserves_distance_from_center() {
    let position = [2.0, 2.0, 0.0];
    let rotated = rotate_y(position, CENTER, 45.0);
    let original_distance = (position[0] - CENTER[0]).hypot(position[2] - CENTER[2]);
    let rotated_distance = (rotated[0] - CENTER[0]).hypot(rotated[2] - CENTER[2]);

    assert_close(rotated_distance, original_distance);
}
