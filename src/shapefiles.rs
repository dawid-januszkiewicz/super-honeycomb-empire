extern crate shapefile;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use shapefile::{Reader, Shape};

pub fn extract_vertices(in_file_path: &str, out_file_path: &str) -> Result<(), Box<dyn Error>> {
    // Open the .shp file
    // let file_path = "path/to/your/file.shp";
    let mut reader = Reader::from_path(in_file_path)?;

    // Create a CSV file
    let mut csv_file = File::create(out_file_path)?;

    // Write headers to CSV file
    writeln!(csv_file, "X,Y,vertex_part")?;

    // Iterate over shapes
    for shape_result in reader.iter_shapes_and_records() {
        let shape_with_data = shape_result?;
        // Extract vertices depending on the shape type
        match shape_with_data.0 {
            Shape::Point(p) => {
                writeln!(csv_file, "{},{},{}", p.x, p.y, 0)?;
            }
            Shape::Polyline(polyline) => {
                for (part_index, part) in polyline.parts().iter().enumerate() {
                    for (vertex_index, point) in part.iter().enumerate() {
                        writeln!(
                            csv_file,
                            "{},{},{}",
                            point.x, point.y, part_index
                        )?;
                    }
                }
            }
            Shape::Polygon(polygon) => {
                for (ring_index, ring) in polygon.rings().iter().enumerate() {
                    for (vertex_index, point) in ring.points().iter().enumerate() {
                        writeln!(
                            csv_file,
                            "{},{},{}",
                            point.x, point.y, ring_index
                        )?;
                    }
                }
            }
            _ => {
                println!("Unsupported Shape Type");
            }
        }
    }

    println!("CSV file generated successfully!");
    Ok(())
}
