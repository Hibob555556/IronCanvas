#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub const CENTER: [f32; 3] = [1.0, 1.0, 1.0];
const VERTEX_COUNT: usize = 24;
const FACE_VERTEX_COUNT: usize = 24;
const FACE_COUNT: usize = FACE_VERTEX_COUNT / 4;
const FACE_COLOR_FLOAT_COUNT: usize = FACE_COUNT * 3;
const ZERO_EPSILON: f32 = 0.000_001;
const SOLID_FACE_COLOR: [f32; 3] = [154.0, 163.0, 173.0];
const ORBIT_LIGHT: [f32; 3] = [-0.580_275, 0.720_341, 0.380_18];

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
static mut CURRENT_CAMERA_VERTICES: [Vertex; VERTEX_COUNT] = VERTICES;
static mut CURRENT_CAMERA_FACE_VERTICES: [Vertex; FACE_VERTEX_COUNT] = FACE_VERTICES;
static mut CURRENT_FACE_COLORS: [f32; FACE_COLOR_FLOAT_COUNT] =
    [SOLID_FACE_COLOR[0]; FACE_COLOR_FLOAT_COUNT];

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
pub extern "C" fn current_cube_camera_vertices_ptr() -> *const f32 {
    core::ptr::addr_of!(CURRENT_CAMERA_VERTICES) as *const Vertex as *const f32
}

#[unsafe(no_mangle)]
pub extern "C" fn current_cube_camera_face_vertices_ptr() -> *const f32 {
    core::ptr::addr_of!(CURRENT_CAMERA_FACE_VERTICES) as *const Vertex as *const f32
}

#[unsafe(no_mangle)]
pub extern "C" fn cube_face_color_count() -> usize {
    FACE_COUNT
}

#[unsafe(no_mangle)]
pub extern "C" fn current_cube_face_colors_ptr() -> *const f32 {
    core::ptr::addr_of!(CURRENT_FACE_COLORS) as *const f32
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
    set_current_cube_camera(-0.72, -0.34);
}

#[unsafe(no_mangle)]
pub extern "C" fn set_current_cube_camera(yaw: f32, pitch: f32) {
    transform_camera_buffer(
        core::ptr::addr_of!(CURRENT_VERTICES) as *const Vertex,
        core::ptr::addr_of_mut!(CURRENT_CAMERA_VERTICES) as *mut Vertex,
        VERTEX_COUNT,
        yaw,
        pitch,
    );
    transform_camera_buffer(
        core::ptr::addr_of!(CURRENT_FACE_VERTICES) as *const Vertex,
        core::ptr::addr_of_mut!(CURRENT_CAMERA_FACE_VERTICES) as *mut Vertex,
        FACE_VERTEX_COUNT,
        yaw,
        pitch,
    );
    update_face_colors();
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
    let angle = degrees.to_radians();

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

pub fn camera_position(position: [f32; 3], center: [f32; 3], yaw: f32, pitch: f32) -> [f32; 3] {
    let centered_x = position[0] - center[0];
    let centered_y = position[1] - center[1];
    let centered_z = position[2] - center[2];
    let yaw_cos = yaw.cos();
    let yaw_sin = yaw.sin();
    let pitch_cos = pitch.cos();
    let pitch_sin = pitch.sin();
    let yaw_x = centered_x * yaw_cos - centered_z * yaw_sin;
    let yaw_z = centered_x * yaw_sin + centered_z * yaw_cos;

    [
        clamp_zero(center[0] + yaw_x),
        clamp_zero(center[1] + centered_y * pitch_cos - yaw_z * pitch_sin),
        clamp_zero(center[2] + centered_y * pitch_sin + yaw_z * pitch_cos),
    ]
}

fn transform_camera_buffer(
    source_ptr: *const Vertex,
    target_ptr: *mut Vertex,
    vertex_count: usize,
    yaw: f32,
    pitch: f32,
) {
    if source_ptr.is_null() || target_ptr.is_null() {
        return;
    }

    let source = unsafe { std::slice::from_raw_parts(source_ptr, vertex_count) };
    let target = unsafe { std::slice::from_raw_parts_mut(target_ptr, vertex_count) };

    for (source_vertex, target_vertex) in source.iter().zip(target.iter_mut()) {
        target_vertex.position = camera_position(source_vertex.position, CENTER, yaw, pitch);
    }
}

fn update_face_colors() {
    let faces = unsafe { &*core::ptr::addr_of!(CURRENT_CAMERA_FACE_VERTICES) };
    let colors = unsafe { &mut *core::ptr::addr_of_mut!(CURRENT_FACE_COLORS) };

    for face_index in 0..FACE_COUNT {
        let vertex_index = face_index * 4;
        let normal = face_normal(
            faces[vertex_index].position,
            faces[vertex_index + 1].position,
            faces[vertex_index + 2].position,
        );
        let light_amount = dot(normal, ORBIT_LIGHT).max(0.0);
        let brightness = 0.28 + light_amount * 0.72;
        let color_index = face_index * 3;

        colors[color_index] = clamp_color(SOLID_FACE_COLOR[0] * brightness);
        colors[color_index + 1] = clamp_color(SOLID_FACE_COLOR[1] * brightness);
        colors[color_index + 2] = clamp_color(SOLID_FACE_COLOR[2] * brightness);
    }
}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let edge_a = subtract(b, a);
    let edge_b = subtract(c, a);
    let normal = normalize(cross(edge_a, edge_b));

    if normal[2] < 0.0 {
        [-normal[0], -normal[1], -normal[2]]
    } else {
        normal
    }
}

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let length = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();

    if length < ZERO_EPSILON {
        [0.0, 0.0, 0.0]
    } else {
        [vector[0] / length, vector[1] / length, vector[2] / length]
    }
}

fn clamp_color(value: f32) -> f32 {
    value.clamp(0.0, 255.0).round()
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

        assert_close(rotated[0], 0.0);
        assert_close(rotated[1], 0.0);
        assert_close(rotated[2], 2.0);
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
        assert!(!current_cube_camera_vertices_ptr().is_null());
        assert!(!current_cube_camera_face_vertices_ptr().is_null());
        assert_eq!(cube_face_color_count(), FACE_COUNT);
        assert!(!current_cube_face_colors_ptr().is_null());
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

        assert_vertex_close(vertices[0], [0.0, 0.0, 2.0]);

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

    #[test]
    fn camera_transform_keeps_display_scale_stable() {
        let position = [0.0, 0.0, 0.0];
        let camera_a = camera_position(position, CENTER, -0.72, -0.34);
        let camera_b = camera_position(position, CENTER, 0.84, 0.38);

        assert_close(
            distance_from_center(camera_a),
            distance_from_center(position),
        );
        assert_close(
            distance_from_center(camera_b),
            distance_from_center(position),
        );
    }

    #[test]
    fn exported_camera_transform_updates_camera_buffers() {
        let _guard = CURRENT_BUFFER_TEST_LOCK.lock().unwrap();

        reset_current_rectangle();
        set_current_cube_camera(0.0, 0.0);

        let vertices = unsafe {
            std::slice::from_raw_parts(
                current_cube_camera_vertices_ptr() as *const Vertex,
                rectangle_vertex_count(),
            )
        };

        assert_vertex_close(vertices[0], [0.0, 0.0, 0.0]);

        set_current_cube_camera(1.0, 0.5);
        let vertices = unsafe {
            std::slice::from_raw_parts(
                current_cube_camera_vertices_ptr() as *const Vertex,
                rectangle_vertex_count(),
            )
        };

        assert_ne!(vertices[0].position, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn exported_camera_transform_updates_face_colors() {
        let _guard = CURRENT_BUFFER_TEST_LOCK.lock().unwrap();

        reset_current_rectangle();
        set_current_cube_camera(0.0, 0.0);
        let first_colors = unsafe {
            std::slice::from_raw_parts(current_cube_face_colors_ptr(), cube_face_color_count() * 3)
        }
        .to_vec();

        assert_eq!(first_colors.len(), FACE_COLOR_FLOAT_COUNT);
        assert!(
            first_colors
                .iter()
                .all(|value| *value >= 0.0 && *value <= 255.0)
        );

        set_current_cube_camera(1.0, 0.5);
        let second_colors = unsafe {
            std::slice::from_raw_parts(current_cube_face_colors_ptr(), cube_face_color_count() * 3)
        };

        assert_ne!(first_colors, second_colors);
    }

    fn assert_vertex_close(actual: Vertex, expected: [f32; 3]) {
        assert_close(actual.position[0], expected[0]);
        assert_close(actual.position[1], expected[1]);
        assert_close(actual.position[2], expected[2]);
    }

    fn distance_from_center(position: [f32; 3]) -> f32 {
        let x = position[0] - CENTER[0];
        let y = position[1] - CENTER[1];
        let z = position[2] - CENTER[2];

        (x * x + y * y + z * z).sqrt()
    }
}
