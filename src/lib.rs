#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub const CENTER: [f32; 3] = [1.0, 1.0, 1.0];
const VERTEX_COUNT: usize = 24;
const FACE_VERTEX_COUNT: usize = 24;
const ZERO_EPSILON: f32 = 0.000_001;

const VERTICES: [Vertex; VERTEX_COUNT] = [
    // Front face, z = 0.0.
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    // Back face, z = 2.0.
    Vertex {
        position: [0.0, 0.0, 2.0],
    },
    Vertex {
        position: [0.0, 2.0, 2.0],
    },
    Vertex {
        position: [0.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 0.0, 2.0],
    },
    Vertex {
        position: [2.0, 0.0, 2.0],
    },
    Vertex {
        position: [0.0, 0.0, 2.0],
    },
    // Connecting edges between faces.
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 2.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 2.0],
    },
];

const FACE_VERTICES: [Vertex; FACE_VERTEX_COUNT] = [
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 2.0],
    },
    Vertex {
        position: [2.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 2.0],
    },
    Vertex {
        position: [0.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 2.0, 2.0],
    },
    Vertex {
        position: [2.0, 0.0, 2.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 2.0],
    },
    Vertex {
        position: [0.0, 0.0, 2.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 2.0],
    },
    Vertex {
        position: [0.0, 2.0, 2.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 2.0],
    },
    Vertex {
        position: [2.0, 0.0, 2.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
];

static mut CURRENT_VERTICES: [Vertex; VERTEX_COUNT] = VERTICES;
static mut CURRENT_FACE_VERTICES: [Vertex; FACE_VERTEX_COUNT] = FACE_VERTICES;

pub fn rectangle_vertices() -> &'static [Vertex] {
    &VERTICES
}

#[unsafe(no_mangle)]
pub extern "C" fn rectangle_vertex_count() -> usize {
    VERTICES.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn rectangle_vertices_ptr() -> *const f32 {
    VERTICES.as_ptr() as *const f32
}

#[unsafe(no_mangle)]
pub extern "C" fn current_rectangle_vertices_ptr() -> *const f32 {
    core::ptr::addr_of!(CURRENT_VERTICES) as *const Vertex as *const f32
}

#[unsafe(no_mangle)]
pub extern "C" fn cube_face_vertex_count() -> usize {
    FACE_VERTICES.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn current_cube_face_vertices_ptr() -> *const f32 {
    core::ptr::addr_of!(CURRENT_FACE_VERTICES) as *const Vertex as *const f32
}

#[unsafe(no_mangle)]
pub extern "C" fn reset_current_rectangle() {
    unsafe {
        core::ptr::copy_nonoverlapping(
            VERTICES.as_ptr(),
            core::ptr::addr_of_mut!(CURRENT_VERTICES) as *mut Vertex,
            VERTEX_COUNT,
        );
        core::ptr::copy_nonoverlapping(
            FACE_VERTICES.as_ptr(),
            core::ptr::addr_of_mut!(CURRENT_FACE_VERTICES) as *mut Vertex,
            FACE_VERTEX_COUNT,
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_current_rectangle_90_clockwise() {
    rotate_current_rectangle_z(90.0);
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_current_rectangle_z(degrees: f32) {
    rotate_rectangle_z(
        core::ptr::addr_of_mut!(CURRENT_VERTICES) as *mut Vertex,
        VERTEX_COUNT,
        degrees,
    );
    rotate_rectangle_z(
        core::ptr::addr_of_mut!(CURRENT_FACE_VERTICES) as *mut Vertex,
        FACE_VERTEX_COUNT,
        degrees,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_current_cube_x(degrees: f32) {
    rotate_rectangle_x(
        core::ptr::addr_of_mut!(CURRENT_VERTICES) as *mut Vertex,
        VERTEX_COUNT,
        degrees,
    );
    rotate_rectangle_x(
        core::ptr::addr_of_mut!(CURRENT_FACE_VERTICES) as *mut Vertex,
        FACE_VERTEX_COUNT,
        degrees,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_current_cube_y(degrees: f32) {
    rotate_rectangle_y(
        core::ptr::addr_of_mut!(CURRENT_VERTICES) as *mut Vertex,
        VERTEX_COUNT,
        degrees,
    );
    rotate_rectangle_y(
        core::ptr::addr_of_mut!(CURRENT_FACE_VERTICES) as *mut Vertex,
        FACE_VERTEX_COUNT,
        degrees,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_rectangle_90_clockwise(vertices_ptr: *mut Vertex, vertex_count: usize) {
    rotate_rectangle_z(vertices_ptr, vertex_count, 90.0);
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_rectangle_z(vertices_ptr: *mut Vertex, vertex_count: usize, degrees: f32) {
    if vertices_ptr.is_null() {
        return;
    }

    let vertices = unsafe { std::slice::from_raw_parts_mut(vertices_ptr, vertex_count) };

    for vertex in vertices.iter_mut() {
        vertex.position = rotate_z(vertex.position, CENTER, degrees);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_rectangle_x(vertices_ptr: *mut Vertex, vertex_count: usize, degrees: f32) {
    if vertices_ptr.is_null() {
        return;
    }

    let vertices = unsafe { std::slice::from_raw_parts_mut(vertices_ptr, vertex_count) };

    for vertex in vertices.iter_mut() {
        vertex.position = rotate_x(vertex.position, CENTER, degrees);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rotate_rectangle_y(vertices_ptr: *mut Vertex, vertex_count: usize, degrees: f32) {
    if vertices_ptr.is_null() {
        return;
    }

    let vertices = unsafe { std::slice::from_raw_parts_mut(vertices_ptr, vertex_count) };

    for vertex in vertices.iter_mut() {
        vertex.position = rotate_y(vertex.position, CENTER, degrees);
    }
}

pub fn rotate_x(position: [f32; 3], center: [f32; 3], degrees: f32) -> [f32; 3] {
    let angle = -degrees.to_radians();

    let x = position[0];
    let y = position[1] - center[1];
    let z = position[2] - center[2];

    [
        clamp_zero(x),
        clamp_zero(center[1] + y * angle.cos() - z * angle.sin()),
        clamp_zero(center[2] + y * angle.sin() + z * angle.cos()),
    ]
}

pub fn rotate_y(position: [f32; 3], center: [f32; 3], degrees: f32) -> [f32; 3] {
    let angle = -degrees.to_radians();

    let x = position[0] - center[0];
    let y = position[1];
    let z = position[2] - center[2];

    [
        clamp_zero(center[0] + x * angle.cos() + z * angle.sin()),
        clamp_zero(y),
        clamp_zero(center[2] - x * angle.sin() + z * angle.cos()),
    ]
}

pub fn rotate_z(position: [f32; 3], center: [f32; 3], degrees: f32) -> [f32; 3] {
    let angle = -degrees.to_radians();

    let x = position[0] - center[0];
    let y = position[1] - center[1];
    let z = position[2];

    [
        clamp_zero(center[0] + x * angle.cos() - y * angle.sin()),
        clamp_zero(center[1] + x * angle.sin() + y * angle.cos()),
        clamp_zero(z),
    ]
}

fn clamp_zero(value: f32) -> f32 {
    if value.abs() < ZERO_EPSILON {
        0.0
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static CURRENT_BUFFER_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.000_01,
            "expected {expected}, got {actual}",
        );
    }

    #[test]
    fn rotate_z_keeps_center_fixed() {
        let rotated = rotate_z(CENTER, CENTER, 45.0);

        assert_close(rotated[0], CENTER[0]);
        assert_close(rotated[1], CENTER[1]);
        assert_close(rotated[2], CENTER[2]);
    }

    #[test]
    fn rotate_z_uses_clockwise_positive_degrees() {
        let rotated = rotate_z([0.0, 0.0, 0.0], CENTER, 90.0);

        assert_close(rotated[0], 0.0);
        assert_close(rotated[1], 2.0);
        assert_close(rotated[2], 0.0);
    }

    #[test]
    fn rotate_x_uses_cube_center_as_pivot() {
        let rotated = rotate_x([0.0, 0.0, 0.0], CENTER, 90.0);

        assert_close(rotated[0], 0.0);
        assert_close(rotated[1], 0.0);
        assert_close(rotated[2], 2.0);
    }

    #[test]
    fn rotate_y_uses_cube_center_as_pivot() {
        let rotated = rotate_y([0.0, 0.0, 0.0], CENTER, 90.0);

        assert_close(rotated[0], 2.0);
        assert_close(rotated[1], 0.0);
        assert_close(rotated[2], 0.0);
    }

    #[test]
    fn clamp_zero_removes_negative_zero_and_float_dust() {
        let value = clamp_zero(-0.000_000_1);

        assert_eq!(value, 0.0);
        assert!(!value.is_sign_negative());
    }

    #[test]
    fn exported_counts_and_pointers_match_runtime_buffers() {
        let _guard = CURRENT_BUFFER_TEST_LOCK.lock().unwrap();

        reset_current_rectangle();

        assert_eq!(rectangle_vertex_count(), VERTEX_COUNT);
        assert_eq!(cube_face_vertex_count(), FACE_VERTEX_COUNT);
        assert!(!rectangle_vertices_ptr().is_null());
        assert!(!current_rectangle_vertices_ptr().is_null());
        assert!(!current_cube_face_vertices_ptr().is_null());
    }

    #[test]
    fn exported_reset_restores_current_edge_buffer() {
        let _guard = CURRENT_BUFFER_TEST_LOCK.lock().unwrap();

        reset_current_rectangle();
        rotate_current_rectangle_z(90.0);
        reset_current_rectangle();

        let vertices = unsafe {
            std::slice::from_raw_parts(
                current_rectangle_vertices_ptr() as *const Vertex,
                rectangle_vertex_count(),
            )
        };

        assert_eq!(vertices[0].position, VERTICES[0].position);
        assert_eq!(
            vertices[VERTEX_COUNT - 1].position,
            VERTICES[VERTEX_COUNT - 1].position
        );
    }

    #[test]
    fn exported_current_rotations_mutate_edges_and_faces() {
        let _guard = CURRENT_BUFFER_TEST_LOCK.lock().unwrap();

        reset_current_rectangle();
        rotate_current_cube_x(90.0);

        let vertices = unsafe {
            std::slice::from_raw_parts(
                current_rectangle_vertices_ptr() as *const Vertex,
                rectangle_vertex_count(),
            )
        };
        let faces = unsafe {
            std::slice::from_raw_parts(
                current_cube_face_vertices_ptr() as *const Vertex,
                cube_face_vertex_count(),
            )
        };

        assert_vertex_close(vertices[0], [0.0, 0.0, 2.0]);
        assert_vertex_close(faces[0], [0.0, 0.0, 2.0]);

        reset_current_rectangle();
        rotate_current_cube_y(90.0);
        let vertices = unsafe {
            std::slice::from_raw_parts(
                current_rectangle_vertices_ptr() as *const Vertex,
                rectangle_vertex_count(),
            )
        };

        assert_vertex_close(vertices[0], [2.0, 0.0, 0.0]);

        reset_current_rectangle();
        rotate_current_rectangle_90_clockwise();
        let vertices = unsafe {
            std::slice::from_raw_parts(
                current_rectangle_vertices_ptr() as *const Vertex,
                rectangle_vertex_count(),
            )
        };

        assert_vertex_close(vertices[0], [0.0, 2.0, 0.0]);
    }

    #[test]
    fn raw_rotation_exports_ignore_null_pointers() {
        rotate_rectangle_90_clockwise(core::ptr::null_mut(), VERTEX_COUNT);
        rotate_rectangle_x(core::ptr::null_mut(), VERTEX_COUNT, 90.0);
        rotate_rectangle_y(core::ptr::null_mut(), VERTEX_COUNT, 90.0);
        rotate_rectangle_z(core::ptr::null_mut(), VERTEX_COUNT, 90.0);
    }

    fn assert_vertex_close(actual: Vertex, expected: [f32; 3]) {
        assert_close(actual.position[0], expected[0]);
        assert_close(actual.position[1], expected[1]);
        assert_close(actual.position[2], expected[2]);
    }
}
