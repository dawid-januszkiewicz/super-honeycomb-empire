/// The worldgen module contains random world generation functions.
/// It exports the generate_world() function for the Game class __init__()
/// method. In the future it will be used by some other module that will
/// allow the user to tweak the worldgen settings from within the game.

// import data
// from playergen import Player
use core::cmp::max;
use core::cmp::min;
use std::collections::HashSet;

use crate::Player;
use crate::World;
use crate::Tile;
use crate::TileCategory;
use crate::Cube;
use crate::Locality;
use crate::LocalityCategory;
use super::extend_borders;
// use crate::cubic::Layout;
// use crate::cubic::POINTY;
// use crate::cubic::FLAT;

extern crate rand;
use rand::Rng;
use rand::random;
use rand::seq::index::sample;

// #layout = cubic.Layout(cubic.orientation_pointy, cubic.Point(50, 50), cubic.Point(800, 550))
// let layout = Layout(POINTY, (.02, .02), (.2, 0));
// #layout = cubic.Layout(cubic.orientation_pointy, cubic.Point(1, 1), cubic.Point(0, 0))

pub enum CapitalsGen {
    Classic,
    Random,
    MaxDist,
}

pub enum LocalitiesGen {
    Random,
    RandomOts, // One Tile of Space
}

pub enum ShapeGen {
    Classic,
    Hexagonal,
}

impl World {
    /// Populates and returns new World instance with cubes, resulting in a
    /// map identical in shape and size to the original hex empire 1 world map.
    fn gen_water(&mut self) {
        self.iter_mut().for_each(|(_, tile)| {
            if random::<f32>() > 0.7 {
                tile.category = TileCategory::Water
            }
        })
    }

    fn gen_classic_shape(&mut self) {
        // layout.orientation = FLAT
        // layout.origin = (-10.0, 10.0)
        let width = 20;
        let height = 11;
        for q in 0..width {
            let q_offset = q >> 1;
            for r in (-1 * q_offset)..(height - q_offset) {
                self.insert(Cube::new(q,r), Tile{owner_index: None, category: TileCategory::Farmland, locality: None, army: None});
            }
        }
    }

    /// Populates and returns a new World instance with cubes forming a hexagonal shape.
    fn gen_hexagonal_shape(&mut self, radius: i32) {
        // align_hex_top_left(map_radius)
        for q in (-1 * radius)..(radius + 1) {
            let r1 = max(-radius, -q - radius);
            let r2 = min(radius, -q + radius);
            for r in r1..(r2 + 1) {
                self.insert(Cube::new(q,r), Tile{owner_index: None, category: TileCategory::Farmland, locality: None, army: None});
            }
        }
    }

    fn choose_shape_gen(&mut self, shape: ShapeGen, radius: i32) {
        match shape {
            ShapeGen::Classic => self.gen_classic_shape(),
            ShapeGen::Hexagonal => self.gen_hexagonal_shape(radius),
        }
    }

    fn gen_random_localities(&mut self, locality_names: &mut Vec<&str>) {
        let mut rng = rand::thread_rng();
        let amount = self.len() / 10;
        let random_positions = sample(&mut rng, self.len(), amount);
        // let mut categories: Vec<LocalityCategory> = Vec::new();

        for (i, world_index) in random_positions.into_iter().enumerate() {
            let cube = self.keys().skip(world_index).next().unwrap();
            let neighbours = cube.disc(1);
            let mut category = LocalityCategory::City;
            for n in neighbours {
                // let mut category: LocalityCategory = LocalityCategory::City;
                if self.get(&n).is_some_and(|t| matches!(t.category, TileCategory::Water)) {
                    // 50% chance to turn city into portcity
                    let roll = rng.gen::<f32>();
                    if roll > 0.5 {
                        category = LocalityCategory::PortCity;
                    }
                    
                    // categories.push(category);
                    break;
                }
                // categories.push(category);
            }

            // 10% chance to turn into airport
            let roll = rng.gen::<f32>();
            if roll > 0.9 {
                category = LocalityCategory::Airport;
            }

            let tile = self.values_mut().skip(world_index).next().unwrap();
            if !matches!(tile.category, TileCategory::Water) {
                tile.locality = Some(Locality::new(locality_names.pop().unwrap_or(&"city"), category))//categories.remove(i)))
            }
        }

        // for (world_index, locality_name) in random_positions.into_iter().zip(locality_names.into_iter().cycle()) {
        //     let tile = self.values_mut().skip(world_index).next().unwrap();
        //     tile.category = categories[i]
        // }

        // for ((cube, tile), locality_name) in self.iter().zip(locality_names.into_iter().cycle()) {
        //     if !matches!(tile.category, TileCategory::Water) && rand::thread_rng().gen::<f32>() > 0.9 {
        //         // check for an adjacent water tile
        //         let mut category = LocalityCategory::City;
        //         for neighbour in cube.disc(1) {
        //             if self.get(&neighbour).is_some_and(|tile| matches!(tile.category, TileCategory::Water)) {
        //                 category = LocalityCategory::PortCity;
        //                 break;
        //             }
        //         }
        //         tile.locality = Some(Locality::new(locality_name, category))
        //     }
        // }
    }

    fn gen_random_localities_with_ots(&mut self, locality_names: &mut Vec<&str>) {
        unimplemented!();
    }
    /// Same as localgen_random(), but ensures there is one tile of space between every locality.
    // get bounds, calc legal cubes, 
    // fn gen_random_localities_with_ots(&mut self, locality_names: Vec<&str>) {
    //     for (cube, locality_name) in self.keys().zip(locality_names) {
    //         let mut tile = self.remove(cube).unwrap();
    //         let mut flag = true;
    //         if tile.locality.is_some() {
    //             flag = false;
    //         }
    //         for neighbour in cube.disc(1) {
    //             if let Some(tile) = self.get(&neighbour) {
    //                 if tile.locality.is_some() {
    //                     flag = false;
    //                     break;
    //                 }
    //             }
    //         }
    //         if flag && rand::thread_rng().gen::<f32>() > 0.9 {
    //             tile.locality = Some(Locality::new(locality_name, LocalityCategory::City));
    //         }
    //     }
    // }

    fn choose_localities_gen(&mut self, gen: LocalitiesGen, locality_names: &mut Vec<&str>) {
        match gen {
            LocalitiesGen::Random => self.gen_random_localities(locality_names),
            LocalitiesGen::RandomOts => self.gen_random_localities_with_ots(locality_names),
        }
    }

    /// Spawn positions hardcoded to correspond to the original hex empire 1 spawn positions.
    fn gen_classic_capitals(&mut self, locality_names: &mut Vec<&str>, mut players: &mut Vec<Player>) {
        let starting_positions = [
            Cube::new(1, 1),
            Cube::new(1, 9),
            Cube::new(18, -8),
            Cube::new(18, 0),
        ];

        for (index, ((player, pos), locality_name)) in players.iter_mut().zip(starting_positions.iter()).zip(locality_names.iter()).enumerate() {
            let mut tile = self.get_mut(&pos).unwrap();
            tile.category = TileCategory::Farmland;
            tile.owner_index = Some(index);
            tile.locality = Some(Locality::new(locality_name, LocalityCategory::Capital));
            player.capital_pos = Some(*pos);

            let set = self.cubes_by_ownership.entry(index).or_insert(HashSet::new());
            set.insert(*pos);

            // extend_borders(self, &pos);
            for neighbour in pos.disc(1) {
                if let Some(tile) = self.get_mut(&neighbour) {
                    tile.category = TileCategory::Farmland;
                    tile.owner_index = Some(index);
                }
            }
        }
    }
    // def gen_random_capitals(filled_world, players):
    //     for player in players:
    //         starting_cube = random.choice(list(filled_world.keys()))
    //         starting_tile = filled_world.get(starting_cube)
    //         starting_tile.owner = player
    //         starting_tile.locality = Locality(data.choose_random_city_name(), "Capital")
    //         player.starting_cube = starting_cube
    fn gen_random_capitals(&mut self, locality_names: &mut Vec<&str>, mut players: &mut Vec<Player>) {
        let start_pos = sample(&mut rand::thread_rng(), self.len(), players.len());
        players.iter_mut().enumerate().for_each(|(player_index, player)| {
            let index = start_pos.index(player_index);
            let cube = self.keys().skip(index).next().unwrap().clone();
            let tile = self.get_mut(&cube).unwrap();
            tile.category = TileCategory::Farmland;
            tile.owner_index = Some(player_index);
            tile.locality = Some(Locality::new("", LocalityCategory::Capital));
            player.capital_pos = Some(cube);
            let set = self.cubes_by_ownership.entry(player_index).or_insert(HashSet::new());
            set.insert(cube);
        });
    }
    fn gen_maxdist_capitals(&mut self, locality_names: &mut Vec<&str>, mut players: &mut Vec<Player>) {
        unimplemented!()
    }

    fn choose_capitals_gen(&mut self, gen: CapitalsGen, mut players: &mut Vec<Player>, locality_names: &mut Vec<&str>) {
        match gen {
            CapitalsGen::Classic => self.gen_classic_capitals(locality_names, &mut players),
            CapitalsGen::Random => self.gen_random_capitals(locality_names, &mut players),
            CapitalsGen::MaxDist => self.gen_maxdist_capitals(locality_names, &mut players),
        }
    }

    pub fn generate(&mut self, players: &mut Vec<Player>, shape_gen: ShapeGen, radius: i32, localities_gen: LocalitiesGen, capitals_gen: CapitalsGen, locality_names: &mut Vec<&str>) {
        self.choose_shape_gen(shape_gen, radius);
        self.gen_water();
        self.choose_capitals_gen(capitals_gen, players, locality_names);
        self.choose_localities_gen(localities_gen, locality_names);
    }
}