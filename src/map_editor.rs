use crate::{mquad::*, inputs::{draw_tile_selector, poll_inputs, poll_map_editor_inputs}, cubic::{Layout, self}, world::TileCategory};

use std::{collections::{HashMap, HashSet}, fs::{OpenOptions, File}};
use std::slice::Iter;
use macroquad::prelude::*;

use crate::{cubic::Cube, world::{Tile, World, LocalityCategory}};

// const MODES: [BrushMode; 2] = [BrushMode::Place, BrushMode::Remove];

const TILE_CATS: [TileCategory; 2] = [TileCategory::Farmland, TileCategory::Water];
const LOC_CATS: [LocalityCategory; 5] = [LocalityCategory::Capital, LocalityCategory::SatelliteCapital, LocalityCategory::City, LocalityCategory::PortCity, LocalityCategory::Airport];
const BRUSH_CATS: [BrushLayer; 2] = [BrushLayer::Tile, BrushLayer::Locality];

pub struct Editor {
    pub world: World,
    pub brush_idx: usize,
    pub brush: BrushLayer,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BrushLayer {
    Tile,
    Locality,
}

// impl BrushMode {
//     pub fn iterator() -> Iter<'static, BrushMode> {
//         static MODES: [BrushMode; 2] = [BrushMode::Place, BrushMode::Remove];
//         MODES.iter()
//     }
// }

impl Editor {
    pub fn new(world: World) -> Self {
        Editor{world, brush_idx: 0, brush: BrushLayer::Tile}
    }
    fn paint_tile(&mut self, cube: &Cube<i32>) {
        match self.brush_idx {
            2 => self.remove_tile(cube),
            _ => {
                let category = TILE_CATS[self.brush_idx].clone();
                self.place_tile(cube, category)
            },
        }
    }
    fn paint_locality(&mut self, cube: &Cube<i32>) {
        match self.brush_idx {
            5 => self.remove_locality(cube),
            _ => {
                let category = LOC_CATS[self.brush_idx].clone();
                self.place_locality(cube, category)
            },
        }
    }
    pub fn click(&mut self, cube: &Cube<i32>) {
        match self.brush {
            BrushLayer::Tile => self.paint_tile(cube),
            BrushLayer::Locality => self.paint_locality(cube),
        };
    }
    pub fn right_click(&mut self) {
        self.brush_idx += 1;
        let max = match self.brush {
            BrushLayer::Tile => TILE_CATS.len(),
            BrushLayer::Locality => LOC_CATS.len(),
        };
        self.brush_idx %= max + 1;
    }
    pub fn toggle_layer(&mut self) {
        let index = BRUSH_CATS.iter().position(|x| x == &self.brush).unwrap();
        println!("{}", index);
        let next_index = (index + 1) % BRUSH_CATS.len();
        self.brush = BRUSH_CATS[next_index].clone();
        self.brush_idx = 0;
    }
    fn place_tile(&mut self, &cube: &Cube<i32>, category: TileCategory) {
        let tile = Tile::new(category);
        self.world.insert(cube, tile);
    }
    fn remove_tile(&mut self, cube: &Cube<i32>) {
        self.world.remove(cube);
    }
    fn place_locality(&mut self, &cube: &Cube<i32>, category: LocalityCategory) {
        unimplemented!()
    }
    fn remove_locality(&mut self, &cube: &Cube<i32>) {
        unimplemented!()
    }
}

pub fn save_map(hashmap: &HashMap<Cube<i32>, Tile>, path: &str) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .expect("Failed to open the file.");

    match serde_json::to_writer(file, &hashmap) {
        Ok(()) => println!("HashMap serialized and saved successfully."),
        Err(e) => eprintln!("Error during serialization: {}", e),
    }
}

impl World {

    pub fn from_json(path: &str) -> Self {
        let f = File::open(path)
            .expect("file should open read only");


        serde_json::from_reader(f).expect("file should be proper JSON")
        // let world: HashMap<Cube<i32>, Tile> = serde_json::from_reader(f).expect("file should be proper JSON");

        // let mut cubes_by_ownership: HashMap<usize, HashSet<Cube<i32>>> = HashMap::new();
        // let mut cubes_with_airport: HashSet<Cube<i32>> = HashSet::new();

        // world.iter().for_each(|(cube, tile)| {
        //     if let Some(index) = tile.owner_index {
        //         let set = cubes_by_ownership.entry(index).or_insert(HashSet::new());
        //         set.insert(*cube);
        //     }
        //     if let Some(locality) = &tile.locality {
        //         if matches!(locality.category, LocalityCategory::Airport) {
        //             cubes_with_airport.insert(*cube);
        //         }
        //     }
        // });

        // World {
        //     world,
        //     cubes_by_ownership,
        //     cubes_with_airport,
        // }
        
    }
}

// pub async fn run_editor(world: &World, layout: &Layout<f32>, assets: &Assets, time: f32) {
pub async fn run_editor(assets: &Assets) {

    let mut editor = Editor::new(World::new());

    let size = [32.,32.];
    // let size = [0.1,0.1]; // use this if in local coords
    let origin = [300., 300.];//[100.,600.];//[100.,300.];
    let mut layout = cubic::Layout{orientation: cubic::OrientationKind::Flat(cubic::FLAT), size, origin};

    let mut time = 0.0;

    loop {
        clear_background(DARKGRAY);

        poll_map_editor_inputs(&mut editor, &mut layout);

        draw_editor(&editor, &layout, &assets, time);

        next_frame().await;
        time += get_frame_time();
    }
    
}