use std::{collections::HashMap, f32::consts::PI};
use glam::Vec2;

#[derive(Copy, Clone)]
pub struct Vertex {
    local_position: [f32; 2],
}
glium::implement_vertex!(Vertex, local_position);

#[derive(Copy, Clone)]
pub struct InstanceAttribute {
    pub position: [f32; 2],
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub color: [f32; 3]
}

glium::implement_vertex!(
    InstanceAttribute,
    position,
    scale_x,
    scale_y,
    rotation,
    color
);

/// Adds verticies and indices representing circle shape to existing conainer.
/// `radius` must be greater than `0.0`
///
/// `nr_of_triangles` specifies accuracy of the circle, must be at least 3 or higher.
///
/// 3 => isosceles triangle
///
/// 4 => `PI/2` tilted square
///
/// 5 => pentagon
///
/// 6 => hexagon
///
pub fn disk_mesh(nr_of_triangles: u16) -> (Vec<Vertex>, Vec<u16>) {
    assert!(nr_of_triangles > 2);

    let mut verticies: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    let delta_angle = (2.0 * PI) / nr_of_triangles as f32;

    verticies.push(Vertex {
        local_position: [0.0, 0.0],
    });
    verticies.push(Vertex {
        local_position: [0.0, 1.0],
    });

    for i in 2..nr_of_triangles + 1 {
        let angle = delta_angle * (i - 1) as f32;
        let x = angle.sin();
        let y = angle.cos();
        verticies.push(Vertex {
            local_position: [x, y],
        });
        indices.push(0);
        indices.push(i - 1);
        indices.push(i);
    }

    indices.push(0);
    indices.push(nr_of_triangles);
    indices.push(1);

    (verticies, indices)
}

pub fn square_mesh() -> (Vec<Vertex>, Vec<u16>) {
    let mut verticies: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    verticies.push(Vertex {
        local_position: [-0.5, 0.5],
    });
    verticies.push(Vertex {
        local_position: [-0.5, -0.5],
    });
    verticies.push(Vertex {
        local_position: [0.5, -0.5],
    });
    verticies.push(Vertex {
        local_position: [0.5, 0.5],
    });

    indices.push(0);
    indices.push(1);
    indices.push(2);

    indices.push(0);
    indices.push(3);
    indices.push(2);

    (verticies, indices)
}