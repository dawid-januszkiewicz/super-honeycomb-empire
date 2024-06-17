extern crate shapefile;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use shapefile::{Reader, Shape};

pub fn extract_vertices(in_file_path: &str) -> Result<(Vec<f32>, Vec<f32>, Vec<i32>), Box<dyn Error>> {
    // Open the .shp file
    // let file_path = "path/to/your/file.shp";
    let mut reader = Reader::from_path(in_file_path)?;

    // Create a CSV file
    // let mut vertices = Vec::new();
    let mut vertices = (Vec::new(), Vec::new(), Vec::new());

    // X, Y, vertex_part

    // Iterate over shapes
    for shape_result in reader.iter_shapes_and_records() {
        let shape_with_data = shape_result?;
        // Extract vertices depending on the shape type
        match shape_with_data.0 {
            Shape::Point(p) => {
                // let row = vec!(p.x, p.y, 0);
                // vertices.append(&mut row);
                vertices.0.push(p.x as f32);
                vertices.1.push(p.y as f32);
                vertices.2.push(0);
            }
            Shape::Polyline(polyline) => {
                for (part_index, part) in polyline.parts().iter().enumerate() {
                    for (vertex_index, point) in part.iter().enumerate() {
                        // let row = vec!(point.x, point.y, part_index);
                        // vertices.append(&mut row);
                        vertices.0.push(point.x as f32);
                        vertices.1.push(point.y as f32);
                        vertices.2.push(part_index as i32);
                    }
                }
            }
            Shape::Polygon(polygon) => {
                for (ring_index, ring) in polygon.rings().iter().enumerate() {
                    for (vertex_index, point) in ring.points().iter().enumerate() {
                        // let row = vec!(point.x, point.y, ring_index);
                        // vertices.append(&mut row);
                        vertices.0.push(point.x as f32);
                        vertices.1.push(point.y as f32);
                        vertices.2.push(ring_index as i32);
                    }
                }
            }
            _ => {
                println!("Unsupported Shape Type");
            }
        }
    }

    Ok(vertices)
}
