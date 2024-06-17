extern crate serde;

use std::{collections::{HashMap, HashSet}, hash::Hash, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

use crate::{Cube, Tile, World};

#[derive(Debug, Serialize, Deserialize)]
pub enum Visibility {
    Clear,
    Shroud,
    Fog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisibilityMask(HashMap<Cube<i32>, Visibility>);

impl VisibilityMask {
    // iter over overlap of cubes_owned_by_pid & cubes_with_army_or_locality
    // For each Cube, take a ring around it and set everything to Visibility::Clear
    // Take diff against World, and set missing Cubes to Fog.
    pub fn new(world: &World, player: usize) -> Self {
        Self((world.iter().map(
            |(cube, tile)| {
                let mut visibility = Visibility::Fog;
                if tile.owner_index.is_some_and(|i| i == player) || tile.army.as_ref().is_some_and(|a| a.owner_index.is_some_and(|i| i == player)) {
                    visibility = Visibility::Clear;
                }
                (*cube, visibility)
            }
        )).collect())
    }
    // Get current mask (s1)
    // Get overlap of cubes_owned_by_pid & cubes_with_army_or_locality (s2)
    // Take a ring around every cube in s2 (s3)
    // Everything in s3 gets set to Visibility::Clear
    // Everything in s1 but not in s3 gets set to Visibility::Shroud
    // rest stays as-is
    pub fn update(&mut self, world: &World, player: usize) {
        let non_foggy: HashSet<_> = self.iter().filter_map(|(k, v)| match v {
            Visibility::Clear => Some(k.clone()),
            Visibility::Shroud => Some(k.clone()),
            Visibility::Fog => None,
        }).collect();
        //let player_cubes: &HashSet<Cube<i32>> = world.cubes_by_ownership.get(&player).unwrap();
        let observers: HashSet<_> = world.iter().filter_map(|(cube, tile)| {
            if tile.owner_index.is_some_and(|i| i == player) || tile.army.as_ref().is_some_and(|a| a.owner_index.is_some_and(|i| i == player)) {
                Some(cube)
            } else {
                None
            }
        }).collect();
        let observed: HashSet<_> = observers.iter().fold(Vec::new(), |mut prev, cube| {
            prev.append(&mut cube.disc(1));
            prev
        }).into_iter().collect();
        observed.iter().for_each(|cube| {self.insert(*cube, Visibility::Clear);});
        non_foggy.difference(&observed).for_each(|cube| {self.insert(*cube, Visibility::Shroud);});
    }
}

impl Deref for VisibilityMask {
    type Target = HashMap<Cube<i32>, Visibility>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VisibilityMask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// enum VisibilityMode {
//     Shroud,
//     Fog,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Fog {
//     reference_counts: HashMap<Cube<i32>, usize>,
//     fog_or_shroud: HashSet<Cube<i32>>,
//     mode: VisibilityMode,
// }

// impl Fog {
//     fn new(mode: VisibilityMode) -> Self {
//         let reference_counts = HashMap::new();
//         let fog_or_shroud = HashSet::new();
//         Self {reference_counts, fog_or_shroud, mode}
//     }

//     fn flip(&mut self, fog_or_shroud: HashSet<Cube<i32>>) {
//         self.fog_or_shroud = fog_or_shroud;
//         self.mode = match self.mode {
//             VisibilityMode::Shroud => VisibilityMode::Fog,
//             VisibilityMode::Fog => VisibilityMode::Shroud,
//         }
//     }
// }

// impl Default for Fog {
//     fn default() -> Self {
//         Self::new(VisibilityMode::Shroud)
//     }
// }

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Fog(HashMap<Cube<i32>, usize>);

impl Fog {
    fn new() -> Self {
        Self {0: HashMap::new()}
    }
}

impl Deref for Fog {
    type Target = HashMap<Cube<i32>, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl World {
    pub fn get_visible_subset(&self, fog: &Fog) -> HashMap<Cube<i32>, &Tile> {
        // let visible = HashSet::new();
        // fog.iter().for_each(|(cube, _)| {
        //     let visible_for_this_observer = self.get_reachable_cubes(cube);
        // });
        // todo!()
        let mut subset = HashMap::new();
        for (key, _) in fog.iter() {
            if let Some(value) = self.world.get(&key) {
                subset.insert(key.clone(), value);
            }
        }
        subset
    }
}