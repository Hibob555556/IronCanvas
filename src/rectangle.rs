fn main() {
    let vertices = iron_canvas::rectangle_vertices();

    for vertex in vertices {
        println!("{:?}", vertex);
    }

    println!("\nRotated vertices:");

    let rotated_vertices: Vec<iron_canvas::Vertex> = vertices
        .iter()
        .map(|vertex| iron_canvas::Vertex {
            position: iron_canvas::rotate_z(vertex.position, iron_canvas::CENTER, 90.0),
        })
        .collect();

    for vertex in rotated_vertices {
        println!("{:?}", vertex);
    }
}
