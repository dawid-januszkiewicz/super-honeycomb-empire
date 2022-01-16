/// The army module describes the logic of the army struct.
/// The only function it exports for external use is the issue_order()
/// function, which provides the Player class with a high level logic
/// for interacting with the game world. Some other modules might also
/// need to import some of the constants or the army dataclass itself.

use std::ops::Add;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use std::collections::HashMap;
use std::collections::hash_map;
use std::collections::HashSet;

use std::cmp::min;
use std::cmp::max;

use std::ptr;

use crate::cubic::Cube;
use crate::Tile;
use crate::Player;

const MAX_TRAVEL_DISTANCE: i32 = 2;
const MAX_STACK_SIZE: i32 = 99;
const BASE_GROWTH_CITY: i32 = 5;
const BASE_GROWTH_CAPITAL: i32 = 10;
const BONUS_GROWTH_PER_TILE: i32 = 1;

const MORALE_BONUS_ANNEX_RURAL: i32 = 1;
const MORALE_BONUS_ANNEX_CITY_ORIGIN: i32 = 20;
const MORALE_BONUS_ANNEX_CITY_ALL: i32 = 10;
const MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ORIGIN: i32 = 80;
const MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ALL: i32 = 50;
const MORALE_PENALTY_LOSING_CITY: i32 = 10;
const MORALE_PENALTY_PER_MANPOWER_LOSING_BATTLE: f32 = 0.1;
const MORALE_PENALTY_IDLE_ARMY: i32 = 1;


// Players interact with the game world by issuing commands to tiles containing an army,
// effectively moving armies across tiles.
#[derive(Debug)]
pub struct Army {
    pub manpower: i32,
    pub morale: i32,
    pub owner_index: Option<usize>,
    pub can_move: bool,
}

impl Army {
    // fn update_morale() ?
    fn apply_morale_bonus(&mut self, bonus: i32) {
        assert!(bonus > 0);
        self.morale = min(self.manpower, self.morale + bonus)
    }
    fn apply_morale_penalty(&mut self, penalty: i32, minimum_morale_value: i32) {
        assert!(penalty < 0);
        let minimum_morale = min(self.manpower, minimum_morale_value);
        self.morale = max(minimum_morale, self.morale - penalty);
    }
    fn combat_strength(&self) -> i32 {
        self.manpower + self.morale
    }
}

impl Display for Army {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}/{})", self.manpower, self.morale)
    }
}

/// Sets the owner of the nearest neighbours (NN) of the target tile,
/// to the owner of the origin tile, subject to conditions.
/// Conditions: The NN tile does not contain any armies or localities,
/// and does not already belong to origin.owner.
pub fn extend_borders(world: &mut HashMap<Cube<i32>, Tile>, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
    let origin_tile = world.remove(origin_cube).unwrap();
    let neighbours_cube = target_cube.disc(1); // Find the NN of the target cube

    // let neighbours_tile: Vec<_> = neighbours_cube.iter().filter_map(|x| world.get_mut(x)).collect(); // doesn't work
    // let mut world_mut = world.iter_mut().collect::<HashMap<_, _>>();
    // let neighbours_tile: Vec<_> = neighbours_cube.iter().filter_map(|x| world_mut.remove(x)).collect();

    // Check the conditions and if satisfied, change the ownership
    let mut morale_bonus = 0;
    for cube in neighbours_cube {
        if let Some(tile) = world.get_mut(&cube) {
            if tile.army.is_none() && tile.locality.is_none() && tile.owner_index != origin_tile.owner_index {
                tile.owner_index = origin_tile.owner_index;
                morale_bonus += 1;
            }
        }
    }

    // Apply the morale bonus
    for tile in world.values_mut() {
        if let Some(army) = &mut tile.army {
            if tile.owner_index == origin_tile.owner_index {
                army.apply_morale_bonus(morale_bonus);
            }
        }
    }
    world.insert(*origin_cube, origin_tile);
}

/// Issues an appropriate order to the origin tile,
/// with the target tile as the order target.
/// This function is called from within the Player.click_on_tile() method,
/// and the order to be issued is determined based on the following conditions:
/// move_to() - the target tile has no army and belongs to the origin tile owner.
/// capture_tile() - the target tile has no army.
/// regroup() - the target tile has an allied army.
/// attack() - the target tile has a hostile army.
pub fn issue_order(mut world: &mut HashMap<Cube<i32>, Tile>, mut players: &mut Vec<Player>, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
    let target = world.get(target_cube).unwrap();
    let origin_owner = world.get(origin_cube).unwrap().owner_index;
    let target_owner = world.get(target_cube).unwrap().owner_index;
    // let target_army = world.get(target_cube).unwrap().army.as_ref();

    let mut extend = true;

    // one can view move_to() as a special case of regroup(), as with attack() and capture()...
    match &target.army {
        Some(army) if target_owner == origin_owner => regroup(&mut world, origin_cube, target_cube),
        Some(army) => { // attack
            let losing_player = attack(&mut world, origin_cube, target_cube);
            if losing_player == origin_owner {
                extend = false;
            }
        },
        None if origin_owner == target_owner => move_to(&mut world, origin_cube, target_cube), // own empty
        None if origin_owner != target_owner => capture_tile(&mut world, origin_cube, target_cube), // else's empty
        _ => unreachable!(),
    }

    if extend {
        extend_borders(&mut world, origin_cube, target_cube);
    }

    let mut origin_tile = world.get_mut(origin_cube).unwrap();
    origin_tile.owner(&mut players).unwrap().actions -= 1;
}

// Moves the origin tile army to the target tile.
fn move_to(mut world: &mut HashMap<Cube<i32>, Tile>, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.get_mut(target_cube).unwrap();
    target.army = origin.army;
    target.army.as_mut().unwrap().can_move = false;
    origin.army = None;
    world.insert(*origin_cube, origin);
}

// Combines the origin tile army with an allied target tile army.
fn regroup(mut world: &mut HashMap<Cube<i32>, Tile>, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.get_mut(target_cube).unwrap();
    let mut origin_army = origin.army.as_mut().unwrap();
    let mut target_army = target.army.as_mut().unwrap();

    let total_manpower = origin_army.manpower + target_army.manpower;
    let army_over_max_stack = total_manpower - MAX_STACK_SIZE;
    if army_over_max_stack <= 0 {
        target_army.manpower = total_manpower;
        target_army.morale = ((origin_army.morale + target_army.morale) as f32 / 2.).round() as i32;
        origin.army = None;
    } else {
        let origin_morale_per_manpower = origin_army.morale as f32 / origin_army.manpower as f32;
        target_army.manpower += origin_army.manpower - army_over_max_stack;
        target_army.morale = ((origin_army.morale as f32 - (army_over_max_stack as f32 * origin_morale_per_manpower) + target_army.morale as f32) / 2.).round() as i32;
        origin_army.manpower = army_over_max_stack;
        origin_army.morale = (army_over_max_stack as f32 * origin_morale_per_manpower).round() as i32;
    }
    target_army.can_move = false;

    world.insert(*origin_cube, origin);
}

/// Attacks the target tile from the origin tile.
fn attack(mut world: &mut HashMap<Cube<i32>, Tile>, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) -> Option<usize> {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.remove(target_cube).unwrap();

    println!("{:?} attacks {:?} with {:?} against {:?}", origin.owner_index, target.owner_index, origin.army, target.army);

    origin.army.as_mut().unwrap().can_move = false;

    let mut manpower_lost = 0;
    let mut losing_player = None;

    let diff = origin.army.as_ref().unwrap().combat_strength() - target.army.as_ref().unwrap().combat_strength();

    let combat_strength_to_army = (diff as f32/2.).ceil() as i32;

    if diff > 0 {
        losing_player = target.owner_index;
        match target.army {
            Some(army) => {
                target.army = None;
                manpower_lost = army.manpower;
            },
            None => manpower_lost = 0,
        }
        origin.army.as_mut().unwrap().manpower = combat_strength_to_army;
        origin.army.as_mut().unwrap().morale = combat_strength_to_army;
    } else {
        losing_player = origin.owner_index;
        manpower_lost = origin.army.unwrap().manpower;
        origin.army = None;
        target.army.as_mut().unwrap().manpower = max(1, -combat_strength_to_army);
        target.army.as_mut().unwrap().morale = max(1, -combat_strength_to_army);
    }

    world.insert(*origin_cube, origin);
    world.insert(*target_cube, target);
    if diff > 0 {capture_tile(&mut world, origin_cube, target_cube);}
    apply_morale_penalty_losing_combat(&mut world, losing_player.unwrap(), manpower_lost);
    losing_player
}

/// Calculates the minimum morale value an army can have.
/// If you use this function you have to remember to still account for the fact
/// that an army's morale shouldn't exceed its manpower.
fn calculate_minimum_morale(world: &HashMap<Cube<i32>, Tile>, player_index: usize) -> i32 {
    let mut total_manpower = 0;
    for tile in world.values() {
        if tile.owner_index == Some(player_index) {
            if let Some(army) = &tile.army {
                total_manpower += army.manpower;
            }
        }
    }
    total_manpower / 50 // implicit floor()
}

/// Calculates and applies the morale penalty to every army of the losing player.
fn apply_morale_penalty_losing_combat(mut world: &mut HashMap<Cube<i32>, Tile>, losing_player_index: usize, manpower_lost: i32) {
    let penalty = (MORALE_PENALTY_PER_MANPOWER_LOSING_BATTLE * manpower_lost as f32) as i32; // implicit floor
    let minimum_morale_value = calculate_minimum_morale(&world, losing_player_index);
    println!("Player {:?} suffers {} morale penalty", losing_player_index, penalty);
    for tile in world.values_mut() {
        if tile.owner_index == Some(losing_player_index) {
            if let Some(army) = tile.army.as_mut() {
                army.apply_morale_penalty(penalty, minimum_morale_value);
            }
        }
    }
}

/// Change the owner of the target tile to that of the origin tile,
/// and apply appropriate morale modifiers to the owners of those tiles.
fn capture_tile(mut world: &mut HashMap<Cube<i32>, Tile>, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
// fn capture_tile(mut game_world_tiles: &mut hash_map::ValuesMut<Cube<i32>, Tile>, mut origin: &mut Tile, mut target: &mut Tile) {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.remove(target_cube).unwrap();
    println!("{:?} captures {} from {:?}", origin.owner_index, target, target.owner_index);
    let mut capturing_army_morale_bonus = 0;
    let mut origin_owner_morale_bonus = 0;
    let mut target_owner_morale_penalty = None;
    let minimum_morale_value = -1; // this better not get executed

    // Calculate morale bonus/penalty
    // let target = world.get_mut(target_cube).unwrap();
    match &target.locality {
        Some(locality) if locality.category == "Capital".to_string() => {
            capturing_army_morale_bonus = MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ORIGIN;
            let origin_owner_morale_bonus = MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ALL;
        },

        Some(locality) if locality.category == "City".to_string() => {
            capturing_army_morale_bonus = MORALE_BONUS_ANNEX_CITY_ORIGIN;
            let origin_owner_morale_bonus = MORALE_BONUS_ANNEX_CITY_ALL;
            let minimum_morale_value = calculate_minimum_morale(&world, target.owner_index.unwrap());
            target_owner_morale_penalty = Some(MORALE_PENALTY_LOSING_CITY);
        },

        None if target.category == "farmland" => {
            capturing_army_morale_bonus = MORALE_BONUS_ANNEX_RURAL;
            let origin_owner_morale_bonus = MORALE_BONUS_ANNEX_RURAL;
        },
        _ => unreachable!(),
    };

    // Apply morale bonus/penalty
    // ... to the capturing army:
    origin.army.as_mut().unwrap().apply_morale_bonus(capturing_army_morale_bonus);

    // ... to other armies:
    for tile in world.values_mut() {
        if tile.army.is_some() {
            if tile.owner_index == origin.owner_index {
                // tile.army.as_mut().unwrap().morale = origin_owner_morale_bonus(tile);
                tile.army.as_mut().unwrap().apply_morale_bonus(origin_owner_morale_bonus);
            } else if tile.owner_index == target.owner_index {
                // tile.army.as_mut().unwrap().morale = target_owner_morale_penalty(tile);
                if let Some(penalty) = target_owner_morale_penalty {
                    tile.army.as_mut().unwrap().apply_morale_penalty(penalty, minimum_morale_value);
                }
            }
        }
    }
    
    // Actually capture the tile
    target.owner_index = origin.owner_index;
    world.insert(*origin_cube, origin);
    world.insert(*target_cube, target);
    move_to(&mut world, origin_cube, target_cube);
}
