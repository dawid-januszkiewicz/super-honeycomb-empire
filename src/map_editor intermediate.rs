use crate::{mquad::*, inputs::{draw_tile_selector, poll_inputs, poll_map_editor_inputs}, cubic::{Layout, self}, world::{TileCategory, Locality}};

use std::{collections::{HashMap, HashSet}, fs::{OpenOptions, File}};
use std::slice::Iter;
use macroquad::prelude::*;

use strum::IntoEnumIterator;
use strum::EnumIter;

use crate::{cubic::Cube, world::{Tile, World, LocalityCategory}};

pub struct Editor {
    pub world: World,
    pub brush_idx: usize,
    pub brush: BrushLayer,
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum BrushLayer {
    Tile,
    Locality,
}

// impl BrushLayer {
//     fn paint<U, T: IntoEnumIterator + Into<U>>(&mut self, &cube: &Cube<i32>, category: T) {
//         let s = T::into(category);

//         match self {
//             Tile => {},
//             Locality => {}
//         }

//         if U == Tile {
//             // let tile = Tile::new(category);
//             self.world.insert(cube, s);
//         } else {
//             if let Some(tile) = self.world.get(&cube) {
//                 tile.locality = s;
//             }
//         }
//     }
// }

// impl BrushLayer {
//     pub fn iter() -> Iter<'static, BrushLayer> {
//         static MODES: [BrushLayer; 2] = [BrushLayer::Tile, BrushLayer::Locality];
//         MODES.iter()
//     }
// }

trait PlaceItem {
    fn place(&self, world: &mut World, cube: &Cube<i32>);
}

trait RemoveItem {
    fn remove(world: &mut World, cube: &Cube<i32>);
}

impl PlaceItem for LocalityCategory {
    fn place(&self, world: &mut World, cube: &Cube<i32>) {
        if let Some(tile) = world.get_mut(&cube) {tile.locality = Some(self.clone().into())}
    }
}

impl RemoveItem for LocalityCategory {
    fn remove(world: &mut World, cube: &Cube<i32>) {
        if let Some(tile) = world.get_mut(&cube) {tile.locality = None}
    }
}

impl PlaceItem for TileCategory {
    fn place(&self, world: &mut World, cube: &Cube<i32>) {
        world.insert(*cube, self.clone().into());
    }
}

impl RemoveItem for TileCategory {
    fn remove(world: &mut World, cube: &Cube<i32>) {
        world.remove(cube);
    }
}

impl Editor {
    pub fn new(world: World) -> Self {
        Editor{world, brush_idx: 0, brush: BrushLayer::Tile}
    }
    fn paint_tile(&mut self, cube: &Cube<i32>) {
        match self.brush_idx {
            2 => self.remove_tile(cube),
            _ => {
                let category = TileCategory::iter().nth(self.brush_idx).unwrap();
                // self.place_tile(cube, category)
                TileCategory::place(&category, &mut self.world, &cube);
            },
        }
    }
    fn paint_locality(&mut self, cube: &Cube<i32>) {
        let c = LocalityCategory::iter().count() + 1;
        match self.brush_idx {
            c => self.remove_locality(cube),
            _ => {
                let category = LocalityCategory::iter().nth(self.brush_idx).unwrap();
                // self.place_locality(cube, category)
                LocalityCategory::place(&category, &mut self.world, &cube);
            },
        }
    }

    fn paint<T: IntoEnumIterator + PlaceItem + RemoveItem>(&mut self, cube: &Cube<i32>) {
        match T::iter().nth(self.brush_idx) {
            Some(category) => T::place(&category, &mut self.world, &cube),
            None => T::remove(&mut self.world, cube)
        }
    }
    // fn paint<T: IntoEnumIterator>(&mut self, cube: &Cube<i32>) {
    //     let c = T::iter().count() + 1;
    //     match self.brush_idx {
    //         c => self.remove(cube),
    //         _ => {
    //             let category = T::iter().nth(self.brush_idx).unwrap();
    //             self.place(cube, category)
    //         },
    //     }
    // }
    // fn paint_item<T: Clone>(&mut self, cube: &Cube<i32>, categories: &[T], category: &T) {
    //     if self.brush_idx == categories.len() {
    //         self.remove_item(cube);
    //     } else {
    //         let selected_category = categories[self.brush_idx].clone();
    //         self.place_item(cube, selected_category);
    //     }
    // }
    fn place<T: IntoEnumIterator + PlaceItem>(&mut self, &cube: &Cube<i32>, category: T) {
        // let s = category.into();
        T::place(&category, &mut self.world, &cube);
        // match self.brush {
        //     Tile => self.world.insert(cube, category.into()),
        //     Locality => {if let Some(tile) = self.world.get(&cube) {tile.locality = Some(category.into())}}
        // }
    }
    pub fn click(&mut self, cube: &Cube<i32>) {
        match self.brush {
            BrushLayer::Tile => self.paint::<TileCategory>(cube), //self.paint_tile(cube),
            BrushLayer::Locality => self.paint::<LocalityCategory>(cube)//self.paint_locality(cube),
        };
    }
    pub fn right_click(&mut self) {
        self.brush_idx += 1;
        let max = match self.brush {
            BrushLayer::Tile => TileCategory::iter().count(),
            BrushLayer::Locality => LocalityCategory::iter().count(),
        };
        self.brush_idx %= max + 1;
    }
    pub fn toggle_layer(&mut self) {
        let mut bc = BrushLayer::iter();
        let index = bc.position(|x| x == self.brush).unwrap();
        self.brush = bc.cycle().nth(index + 1).unwrap();
    }
    // fn place_tile(&mut self, &cube: &Cube<i32>, category: TileCategory) {
    //     let tile = Tile::new(category);
    //     self.world.insert(cube, tile);
    // }
    fn remove_tile(&mut self, cube: &Cube<i32>) {
        self.world.remove(cube);
    }
    // fn place_locality(&mut self, &cube: &Cube<i32>, category: LocalityCategory) {
    //     unimplemented!()
    // }
    fn remove_locality(&mut self, &cube: &Cube<i32>) {
        unimplemented!()
    }
}

pub fn save_map(hashmap: &HashMap<Cube<i32>, Tile>, path: &str) {
    let file = File::create(&path).expect("Failed to open the file.");

    match serde_json::to_writer(file, &hashmap) {
        Ok(()) => println!("Map saved successfully!"),
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