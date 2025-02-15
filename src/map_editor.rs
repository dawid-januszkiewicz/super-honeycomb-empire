use crate::{cubic::{self, Layout}, game::VictoryCondition, inputs::{draw_tile_selector, poll_inputs, poll_map_editor_inputs}, mquad::*, rules::Ruleset, world::{Locality, Player, TileCategory}, Component, Fog, VisibilityMask};

use std::{collections::{HashMap, HashSet}, fs::{OpenOptions, File}};
use std::slice::Iter;
use macroquad::prelude::*;

use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum::EnumIter;

use crate::{cubic::Cube, world::{Tile, World, LocalityCategory}};

#[derive(Serialize, Deserialize)]
pub struct Editor {
    pub world: World,
    pub players: Vec<Player>,
    pub player_views: HashMap<usize, World>,
    pub rules: Ruleset,
    pub brush: Brush,
}

#[derive(Serialize, Deserialize)]
pub struct Brush {
    idx: usize,
    layer: BrushLayer,
    size: usize,
}

impl Default for Brush {
    fn default() -> Self {
        Brush {idx: 0, layer: BrushLayer::Tile, size: 1}
    }
}

#[derive(Debug, PartialEq, EnumIter, Serialize, Deserialize)]
pub enum BrushLayer {
    Tile,
    Locality,
    Owner,
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
    fn place(&self, editor: &mut Editor, cube: &Cube<i32>);
}

trait RemoveItem {
    fn remove(editor: &mut Editor, cube: &Cube<i32>);
}

impl PlaceItem for LocalityCategory {
    fn place(&self, editor: &mut Editor, cube: &Cube<i32>) {
        if let Some(tile) = editor.world.get_mut(&cube) {
            let category = self;
            match &mut tile.locality {
                Some(locality) if !matches!(&locality.category, category) => locality.category = self.clone(),
                None => tile.locality = Some(Locality::new("", self.clone())),
                _ => return,
            }
            // tile.locality = Some(self.clone().into());
            if matches!(self, LocalityCategory::Capital(_)) {
                let player_count = editor.players.iter().len();
                editor.players.push(Player{name: "".to_string(), actions: 5, controller: crate::Controller::Human, selection: None});
                tile.owner_index = Some(player_count);
            }
        }
    }
}

impl RemoveItem for LocalityCategory {
    fn remove(editor: &mut Editor, cube: &Cube<i32>) {
        if let Some(tile) = editor.world.get_mut(&cube) {tile.locality = None}
    }
}

impl PlaceItem for TileCategory {
    fn place(&self, editor: &mut Editor, cube: &Cube<i32>) {
        editor.world.entry(*cube)
             .and_modify(|t| t.category = self.clone())
             .or_insert(self.clone().into());
    }
}

impl RemoveItem for TileCategory {
    fn remove(editor: &mut Editor, cube: &Cube<i32>) {
        editor.world.remove(cube);
    }
}

impl Editor {
    pub fn new(world: World, players: Vec<Player>) -> Self {
        let rules = Ruleset::default(crate::VictoryCondition::Elimination, &players);
        Editor{world, brush: Brush::default(), players, player_views: HashMap::new(), rules}
    }
    pub fn to_json(&self, path: &str) {
        let file = File::create(&path).expect("Failed to open the file.");
    
        match serde_json::to_writer(file, self) {
            Ok(()) => println!("HashMap serialized and saved successfully."),
            Err(e) => eprintln!("Error during serialization: {}", e),
        }
    }
    pub fn from_json(path: &str) -> Self {
        let f = File::open(path)
            .expect("file should open read only");

        serde_json::from_reader(f).expect("file should be proper JSON")
    }
    fn paint<T: IntoEnumIterator + PlaceItem + RemoveItem>(&mut self, cube: &Cube<i32>) {
        match T::iter().nth(self.brush.idx) {
            Some(category) => T::place(&category, self, &cube),
            None => T::remove(self, cube)
        }
    }
    pub fn click(&mut self, cube: &Cube<i32>) {
        // println!("{:?}", self.players);
        match self.brush.layer {
            BrushLayer::Tile => self.paint::<TileCategory>(cube),
            BrushLayer::Locality => self.paint::<LocalityCategory>(cube),
            BrushLayer::Owner => if let Some(tile) = self.world.get_mut(cube) {
                self.world.set_tile_owner(cube, self.brush.idx);
                //tile.owner_index = Some(self.brush.idx);
                //self.world.cubes_by_ownership.get_mut(&self.brush.idx).get_or_insert(value).insert(cube);
            },
        };
    }
    pub fn right_click(&mut self) {
        self.brush.idx += 1;
        let max = match self.brush.layer {
            BrushLayer::Tile => TileCategory::iter().len(),
            BrushLayer::Locality => LocalityCategory::iter().len(),
            BrushLayer::Owner => self.players.iter().len(),
        };
        self.brush.idx %= max + 1;
    }
    pub fn toggle_layer(&mut self) {
        let mut b = BrushLayer::iter();
        let max = b.len();
        let index = b.position(|x| x == self.brush.layer).unwrap();
        let new_index: usize = (index + 1) % max;

        self.brush.layer = BrushLayer::iter().nth(new_index).unwrap();
        self.brush.idx = 0;
    }
}



// pub fn to_json(&self, path: &str) {
//     let w_path = path.to_string();
//     w_path.push_str("map.json");
//     self.world.to_json(&w_path);

//     let p_path = path.to_string();
//     p_path.push_str("players.json");
//     let file = File::create(&p_path).expect("Failed to open the file.");
//     match serde_json::to_writer(file, &self.players) {
//         Ok(()) => println!("Map saved successfully!"),
//         Err(e) => eprintln!("Error during serialization: {}", e),
//     }
// }

// pub fn from_json(path: &str) -> Self {
//     let mut w_path = path.to_string();
//     w_path.push_str("map.json");
//     let world = World::from_json(path);

//     let mut p_path = path.to_string();
//     p_path.push_str("players.json");
//     let players = 
//     Editor::new()
// }

impl crate::Component for Editor {
    // type Swap = crate::Game;
    fn draw(&self, &layout: &Layout<f32>, assets: &Assets, time: f32) {
        crate::draw_editor(&self, &layout, assets, time);
    }
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
        crate::poll_map_editor_inputs(self, layout)
    }
    // fn swap(self) -> Self::Swap{//impl Component {
    //     crate::Game::from(self)
    // }
    fn swap(self) -> impl Component {
        crate::Game::from(self)
    }
    fn update(&mut self) {
        {}
    }
    // fn empty() -> Self {
    //     let players = vec!();
    //     let world: World = World::new();
    //     let player_views: HashMap<usize, World> = HashMap::new();
    //     let victory_condition = VictoryCondition::default();
    //     let rules = Ruleset::default(victory_condition, &players);

    //     Self {world, brush: Brush::default(), players, player_views: HashMap::new(), rules}
    // }
}

// pub fn save_world(hashmap: &HashMap<Cube<i32>, Tile>, path: &str) {
//     let file = File::create(&path).expect("Failed to open the file.");

//     match serde_json::to_writer(file, &hashmap) {
//         Ok(()) => println!("Map saved successfully!"),
//         Err(e) => eprintln!("Error during serialization: {}", e),
//     }
// }

impl World {
    pub fn to_json(self, path: &str) {
        let file = File::create(&path).expect("Failed to open the file.");
    
        match serde_json::to_writer(file, &self.world) {
            Ok(()) => println!("Map saved successfully!"),
            Err(e) => eprintln!("Error during serialization: {}", e),
        }
    }

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

    let mut editor = Editor::new(World::new(), vec!());

    let size = [32.,32.];
    // let size = [0.1,0.1]; // use this if in local coords
    let origin = [300., 300.];//[100.,600.];//[100.,300.];
    let mut layout = cubic::Layout{orientation: cubic::OrientationKind::Flat(cubic::FLAT), size, origin};

    let mut time = 0.0;

    loop {
        clear_background(DARKGRAY);

        poll_map_editor_inputs(&mut editor, &mut layout);

        if is_key_pressed(KeyCode::F1) {
            break
        }

        draw_editor(&editor, &layout, &assets, time);

        next_frame().await;
        time += get_frame_time();
    }
    
}