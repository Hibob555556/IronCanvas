#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub const CENTER: [f32; 3] = [1.5, 1.0, 0.0];
const VERTEX_COUNT: usize = 11;
const ZERO_EPSILON: f32 = 0.000_001;

const VERTICES: [Vertex; VERTEX_COUNT] = [
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.0, 2.0, 0.0],
    },
    Vertex {
        position: [1.0, 2.0, 0.0],
    },
    Vertex {
        position: [2.0, 2.0, 0.0],
    },
    Vertex {
        position: [3.0, 2.0, 0.0],
    },
    Vertex {
        position: [3.0, 1.0, 0.0],
    },
    Vertex {
        position: [3.0, 0.0, 0.0],
    },
    Vertex {
        position: [2.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [0.0, 0.0, 0.0],
    },
];

static mut CURRENT_VERTICES: [Vertex; VERTEX_COUNT] = VERTICES;

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
pub extern "C" fn reset_current_rectangle() {
    unsafe {
        core::ptr::copy_nonoverlapping(
            VERTICES.as_ptr(),
            core::ptr::addr_of_mut!(CURRENT_VERTICES) as *mut Vertex,
            VERTEX_COUNT,
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

        assert_close(rotated[0], 0.5);
        assert_close(rotated[1], 2.5);
        assert_close(rotated[2], 0.0);
    }

    #[test]
    fn clamp_zero_removes_negative_zero_and_float_dust() {
        let value = clamp_zero(-0.000_000_1);

        assert_eq!(value, 0.0);
        assert!(!value.is_sign_negative());
    }
}
