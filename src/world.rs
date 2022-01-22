use crate::Cube;
use crate::DIRECTIONS;
use crate::AI;

use std::collections::HashSet;
use std::collections::HashMap;

use std::ops::Deref;
use std::ops::DerefMut;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use std::cmp::max;
use std::cmp::min;

const ACTIONS_PER_TURN: i32 = 5;
const MAX_TRAVEL_DISTANCE: i32 = 2;
pub const MAX_STACK_SIZE: i32 = 99;
const MORALE_BONUS_ANNEX_RURAL: i32 = 1;
const MORALE_BONUS_ANNEX_CITY_ORIGIN: i32 = 20;
const MORALE_BONUS_ANNEX_CITY_ALL: i32 = 10;
const MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ORIGIN: i32 = 80;
const MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ALL: i32 = 50;
const MORALE_PENALTY_LOSING_CITY: i32 = 10;
const MORALE_PENALTY_PER_MANPOWER_LOSING_BATTLE: f32 = 0.1;
const MORALE_PENALTY_IDLE_ARMY: i32 = 1;
const BASE_GROWTH_CITY: i32 = 5;
const BASE_GROWTH_CAPITAL: i32 = 10;
const BONUS_GROWTH_PER_TILE: i32 = 1;

// My own made up constants
const BASE_GROWTH_SATELLITE_CAPITAL: i32 = 7;
const MORALE_BONUS_ANNEX_SATELLITE_CAPITAL_ORIGIN: i32 = 40;
const MORALE_BONUS_ANNEX_SATELLITE_CAPITAL_ALL: i32 = 25;

pub struct Player {
    pub name: String,
    pub actions: i32,
    pub ai: Option<AI>,
    pub selection: Option<Cube<i32>>,

    // self.camera = None
    // self.starting_cube = None
    // self.color = color
    // self.is_defeated = False
}

impl Player {
    pub fn new(name: &str, ai: Option<AI>) -> Self {
        Player {
            name: name.to_string(),
            actions: ACTIONS_PER_TURN,
            ai,
            selection: None,
        }
    }
    pub fn skip_turn(&mut self) {
        self.actions = 0;
    }
    // fn player_index(&self, world: &World) -> Option<usize> {
    //     for cube in self.cubes_owned.iter() {
    //         let first_tile = world.get(cube).unwrap();
    //         return first_tile.owner_index;
    //     }
    //     None
    // }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

pub enum LocalityCategory {
    City,
    PortCity,
    Capital,
    SatelliteCapital,
}

impl Display for LocalityCategory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            LocalityCategory::City => write!(f, "City"),
            LocalityCategory::PortCity => write!(f, "Port City"),
            LocalityCategory::Capital => write!(f, "Capital"),
            LocalityCategory::SatelliteCapital => write!(f, "Satellite Capital"),
        }
        // write!(f, "({})", self)
    }
}

pub struct Locality {
    name: String,
    pub category: LocalityCategory, 
    starting_owner_index: Option<usize>,
}

impl Display for Locality {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} of {})", self.category, self.name)
    }
}

impl Locality {
    fn new(name: &str, category: LocalityCategory) -> Self {
        Locality {
            name: name.to_string(),
            category: category,
            starting_owner_index: None,
        }
    }
}

pub enum TileCategory {
    Farmland,
    Water,
}

impl Display for TileCategory {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            TileCategory::Farmland => write!(f, "Farmland"),
            TileCategory::Water => write!(f, "Water"),
        }
    }
}

pub struct Tile {
    pub owner_index: Option<usize>,
    pub category: TileCategory,
    pub locality: Option<Locality>,
    pub army: Option<Army>,
}

impl Tile {
    pub fn new(category: TileCategory) -> Self {
        Tile {
            owner_index: None,
            category: category,
            locality: None,
            army: None,
        }
    }
}

impl Tile {
    pub fn owner<'a>(&self, players: &'a mut Vec<Player>) -> Option<&'a mut Player> {
        match self.owner_index {
            Some(index) => Some(&mut players[index]),
            None => None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        //     if self.locality:
        //         string = str(self.locality.name)
        //     else:
        //         string = str(self.category)
        //     return string
        // write!(f, "({})", self.category)
        if self.locality.is_some() {
            write!(f, "({})", self.locality.as_ref().unwrap())
        }
        else {
            write!(f, "{}", self.category)
        }
    }
}

// impl<'a> FromIterator<&'a Tile> for [&'a Tile; 6] {
//     fn from_iter<I: IntoIterator<Item=&'a Tile>>(iter: I) -> Self {
//         let mut arr = [&Tile{owner_index: 0, category: "".to_string(), army: None}; 6];

//         let mut i = 0;
//         for elem in iter {
//             arr[i] = elem;
//             i += 1;
//         }
//         // for i in iter {
//         //     c.add(i);
//         // }

//         arr
//     }
// }

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
    pub fn new(manpower: i32, owner_index: Option<usize>) -> Self {
        Army {
            manpower,
            morale: (manpower as f32 / 2.).round() as i32,
            owner_index,
            can_move: true,
        }
    }
    // Grow the army up to MAX_STACK_SIZE, and return any growth overflow.
    pub fn grow(&mut self, manpower: i32) -> i32 {
        let new_manpower = self.manpower + manpower;
        self.manpower = min(MAX_STACK_SIZE, new_manpower);
        self.morale = min(MAX_STACK_SIZE, self.morale + (manpower as f32 /2.).round() as i32); // I think this will be buggy, as morale will continue to grow each turn in proportion to no of tiles owned.
        if new_manpower > MAX_STACK_SIZE {
            new_manpower - MAX_STACK_SIZE
        } else {
            0
        }
    }
    // fn update_morale() ?
    fn apply_morale_bonus(&mut self, bonus: i32) {
        assert!(bonus > 0);
        self.morale = min(self.manpower, self.morale + bonus)
    }
    fn apply_morale_penalty(&mut self, penalty: i32, total_manpower: i32) {
        assert!(penalty > 0);
        assert!(total_manpower >= 0); // sanity error catching, remove later
        let minimum_morale = min(self.manpower, total_manpower / 50);
        self.morale = max(minimum_morale, self.morale - penalty);
    }
    pub fn combat_strength(&self) -> i32 {
        self.manpower + self.morale
    }
}

impl Display for Army {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}/{})", self.manpower, self.morale)
    }
}

// pub struct World(pub HashMap<Cube<i32>, Tile>);

pub struct World {
    pub world: HashMap<Cube<i32>, Tile>,
    pub cubes_by_ownership: HashMap<usize, HashSet<Cube<i32>>>,
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} tile-sized world with {} players)", self.world.len(), self.cubes_by_ownership.len())
    }
}

impl Deref for World {
    type Target = HashMap<Cube<i32>, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}

impl World {
    pub fn new() -> Self {
        World {
            world: HashMap::new(),
            cubes_by_ownership: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: Cube<i32>, value: Tile) {
        // let Some((key, value)) = self.world.get_key_value(&key);
        if let Some(index) = value.owner_index {
            let set = self.cubes_by_ownership.entry(index).or_insert(HashSet::new());
            set.insert(key);
        }
        self.world.insert(key, value);
    }
    pub fn remove(&mut self, k: &Cube<i32>) -> Option<Tile> {
        let value = self.world.remove(k);
        if value.is_some() {
            if let Some(index) = value.as_ref().unwrap().owner_index {
                let set = self.cubes_by_ownership.get_mut(&index).unwrap();
                set.remove(k);
            } 
        }
        value
    }
    /// Issues an appropriate order to the origin tile,
    /// with the target tile as the order target.
    /// Returns a set of captured coordinates.
    /// This function is called from within the Player.click_on_tile() method,
    /// and the order to be issued is determined based on the following conditions:
    /// move_to() - the target tile has no army and belongs to the origin tile owner.
    /// capture_tile() - the target tile has no army.
    /// regroup() - the target tile has an allied army.
    /// attack() - the target tile has a hostile army.
    pub fn execute_army_order(&mut self, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
        let target = self.get(target_cube).unwrap();
        let origin_owner = self.get(origin_cube).unwrap().owner_index;
        let target_owner = self.get(target_cube).unwrap().owner_index;
        // let target_army = self.get(target_cube).unwrap().army.as_ref();
    
        let mut extend = true;
    
        // one can view move_to() as a special case of regroup(), as with attack() and capture()...
        match &target.army {
            Some(army) if target_owner == origin_owner => regroup(self, origin_cube, target_cube),
            Some(army) => { // attack
                let losing_player = attack(self, origin_cube, target_cube);
                if losing_player == origin_owner {
                    extend = false;
                }
            },
            None if origin_owner == target_owner => move_to(self, origin_cube, target_cube), // own empty
            None if origin_owner != target_owner => capture_tile(self, origin_cube, target_cube), // else's empty
            _ => unreachable!(),
        }
    
        if extend {
            extend_borders(self, target_cube);
        }
    }

    pub fn is_cube_passable(&self, cube: &Cube<i32>) -> bool {
        match self.get(cube) {
            Some(tile) => (tile.army.is_none() || tile.locality.is_none()),
            None => false,
        }
    }

    pub fn get_reachable_cubes(&self, start_cube: &Cube<i32>) -> HashSet<Cube<i32>> {
        let mut visited = HashSet::new();
        visited.insert(*start_cube);
        let mut fringes = vec!(); // vec of vecs of cubes
        //fringes.push(&vec!(start_cube));
        fringes.push(*start_cube);
        
        for _ in 0..(MAX_TRAVEL_DISTANCE as usize) {
            //fringes.push(&vec!());
            let mut next_fringes = vec!();
            for cube in fringes {
                for direction in DIRECTIONS {
                    let neighbour = cube + direction;
                    if !visited.contains(&neighbour) {
                        // This way we also add obstacles themselves, if they exist
                        if self.get(&neighbour).is_some() {
                            visited.insert(neighbour);
                        }
                        if self.is_cube_passable(&neighbour) {
                            next_fringes.push(neighbour);
                        }
                    }
                }
            }
            fringes = next_fringes;
        }
        visited.remove(start_cube);
        visited
    }

    pub fn is_there_capturable_tile_within_range(&self, cube: &Cube<i32>) -> bool {
        let valid_targets = self.get_reachable_cubes(&cube);
        for target in valid_targets {
            let origin = self.get(cube).unwrap();
            let target = self.get(&target).unwrap();
            if origin.owner_index != target.owner_index {
                return true
            }
        }
        false
    }

    pub fn train_armies(&mut self, &player_index: &usize) {
        let (world, cubes_by_ownership) = self.split_fields();
        let player_cubes = cubes_by_ownership.get(&player_index).unwrap();

        // First apply base growth
        for cube in player_cubes {
            let mut tile = world.get_mut(&cube).unwrap();
            let mut growth = 0;
            match &tile.locality {
                Some(locality) => match &locality.category {
                    LocalityCategory::City => growth = BASE_GROWTH_CITY,
                    LocalityCategory::PortCity => growth = 0,
                    LocalityCategory::Capital => growth = BASE_GROWTH_CAPITAL,
                    LocalityCategory::SatelliteCapital => growth = BASE_GROWTH_SATELLITE_CAPITAL,
                }
                _ => {continue},
            }
            if growth > 0 { // redundant?
                match &mut tile.army {
                    Some(army) => {
                        army.grow(growth);
                    }
                    None => {
                        tile.army = Some(Army::new(growth, tile.owner_index));
                    }
                }
            }
        }

        // Then apply bonus growth
        let mut bonus_growth = player_cubes.len() as i32 * BONUS_GROWTH_PER_TILE;
        let mut tiles_with_max_army_stack = HashSet::new();
        for cube in player_cubes.iter().cycle() {
            // break if we can't apply the bonus anywhere
            if player_cubes.difference(&tiles_with_max_army_stack).collect::<HashSet<_>>().len() == 0 || bonus_growth <= 0 {
                break;
            }
            let tile = world.get_mut(&cube).unwrap();
            match &tile.locality {
                Some(locality) => match &mut tile.army {
                    Some(army) if army.manpower < MAX_STACK_SIZE => {
                        let overflow = army.grow(2); // ideally 1, but here 2 so we grow morale by a whole number.
                        bonus_growth -= overflow;
                    }
                    _ => {tiles_with_max_army_stack.insert(*cube);}
                }
                None => {tiles_with_max_army_stack.insert(*cube);}
            }
        }
    }

    // Called from within Game at the end of the turn. Applies a morale penalty to idle armies.
    pub fn apply_idle_morale_penalty(&mut self, player_index: usize) {
        let total_manpower = player_total_manpower(&self, player_index);
        let (world, cubes_by_ownership) = self.split_fields();
        let player_cubes = cubes_by_ownership.get(&player_index).unwrap();
        for cube in player_cubes.iter() {
            let tile = world.get_mut(cube).unwrap();
            if let Some(army) = &mut tile.army {
                if army.can_move {
                    army.apply_morale_penalty(MORALE_PENALTY_IDLE_ARMY, total_manpower);
                }
            }
        }
    }
    pub fn can_player_issue_a_command(&self, player_index: &usize) -> bool {
        for cube in self.cubes_by_ownership.get(&player_index).unwrap() {
            let tile = self.world.get(cube).unwrap();
            if let Some(army) = &tile.army {
                if army.can_move {
                    return true
                }
            }
        }
        false
    }
    // Transfer the ownership of all of defeated_player tiles to player.
    pub fn surrender_to_player(&mut self, surrendering_player_index: &usize, conquering_player_index: &usize) {
        let (world, cubes_by_ownership) = self.split_fields();
        for cube in cubes_by_ownership.get(&surrendering_player_index).unwrap() {
            let tile = world.get_mut(cube).unwrap();
            tile.army = None;
            tile.owner_index = Some(*conquering_player_index);
        }
    }
    
    //Split struct fields into separate variables.
    pub fn split_fields(&mut self) -> (&mut HashMap<Cube<i32>, Tile>, &mut HashMap<usize, HashSet<Cube<i32>>>) {
        (&mut self.world, &mut self.cubes_by_ownership)
    }
}

/// Sets the owner of the nearest neighbours (NN) of the target tile,
/// to the owner of the origin tile, subject to conditions.
/// Conditions: The NN tile does not contain any armies or localities,
/// and does not already belong to origin.owner.
fn extend_borders(mut world: &mut World, target_cube: &Cube<i32>) {
    // Origin has now captured the target and so is irrelevant
    let target_owner_index = world.get(&target_cube).unwrap().owner_index;
    let neighbours_cube = target_cube.disc(1); // Find the NN of the target cube
    for cube in neighbours_cube {
        if let Some(tile) = world.get(&cube) {
            if tile.army.is_none() && tile.locality.is_none() && tile.owner_index != target_owner_index {
                capture_tile(&mut world, &target_cube, &cube);
            }
        }
    }
}

// Moves the origin tile army to the target tile.
fn move_to(mut world: &mut World, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.get_mut(target_cube).unwrap();
    target.army = origin.army;
    target.army.as_mut().unwrap().can_move = false;
    origin.army = None;
    world.insert(*origin_cube, origin);
}

// Combines the origin tile army with an allied target tile army.
fn regroup(mut world: &mut World, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
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
fn attack(mut world: &mut World, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) -> Option<usize> {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.remove(target_cube).unwrap();

    println!("{:?} attacks {:?} with {} against {}", origin.owner_index, target.owner_index, origin.army.as_ref().unwrap(), target.army.as_ref().unwrap());

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
fn player_total_manpower(world: &World, player_index: usize) -> i32 {
    let mut total_manpower = 0;
    // let cubes_by_ownership = world.cubes_by_ownership;
    // let player_cubes = cubes_by_ownership.get(&player_index).unwrap();
    let player_cubes = world.cubes_by_ownership.get(&player_index).unwrap();
    for cube in player_cubes.iter() {
        let tile = world.get(&cube).unwrap();
        if tile.owner_index == Some(player_index) {
            if let Some(army) = &tile.army {
                total_manpower += army.manpower;
            }
        }
    }
    total_manpower
}

/// Calculates and applies the morale penalty to every army of the losing player.
fn apply_morale_penalty_losing_combat(mut world: &mut World, losing_player_index: usize, manpower_lost: i32) {
    let penalty = (MORALE_PENALTY_PER_MANPOWER_LOSING_BATTLE * manpower_lost as f32) as i32; // implicit floor
    let total_manpower = player_total_manpower(&world, losing_player_index);
    println!("Player {:?} suffers {} morale penalty", losing_player_index, penalty);
    for tile in world.values_mut() {
        if tile.owner_index == Some(losing_player_index) {
            if let Some(army) = tile.army.as_mut() {
                army.apply_morale_penalty(penalty, total_manpower);
            }
        }
    }
}

/// Change the owner of the target tile to that of the origin tile,
/// and apply appropriate morale modifiers to the owners of those tiles.
fn capture_tile(mut world: &mut World, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
// fn capture_tile(mut game_world_tiles: &mut hash_map::ValuesMut<Cube<i32>, Tile>, mut origin: &mut Tile, mut target: &mut Tile) {
    let mut origin = world.remove(origin_cube).unwrap();
    let mut target = world.remove(target_cube).unwrap();
    println!("{:?} captures {} {} from {:?}", origin.owner_index, target, target_cube, target.owner_index);
    let mut capturing_army_morale_bonus = 0;
    let mut origin_owner_morale_bonus = 0;
    let mut target_owner_morale_penalty = None;
    let target_total_manpower = -1; // this better not get executed

    // Calculate morale bonus/penalty
    // let target = world.get_mut(target_cube).unwrap();
    match &target.locality {
        Some(locality) => match &locality.category {
            LocalityCategory::Capital => {
                capturing_army_morale_bonus = MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ORIGIN;
                let origin_owner_morale_bonus = MORALE_BONUS_ANNEX_SOVEREIGN_CAPITAL_ALL;
            }
            LocalityCategory::SatelliteCapital => {
                capturing_army_morale_bonus = MORALE_BONUS_ANNEX_SATELLITE_CAPITAL_ORIGIN;
                let origin_owner_morale_bonus = MORALE_BONUS_ANNEX_SATELLITE_CAPITAL_ALL;
            }
            LocalityCategory::City | LocalityCategory::PortCity => {
                capturing_army_morale_bonus = MORALE_BONUS_ANNEX_CITY_ORIGIN;
                let origin_owner_morale_bonus = MORALE_BONUS_ANNEX_CITY_ALL;
                let target_total_manpower = player_total_manpower(&world, target.owner_index.unwrap());
                target_owner_morale_penalty = Some(MORALE_PENALTY_LOSING_CITY);
            }
        }
        None if matches!(target.category, TileCategory::Farmland) => {
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
                tile.army.as_mut().unwrap().apply_morale_bonus(origin_owner_morale_bonus);
            } else if tile.owner_index == target.owner_index {
                if let Some(penalty) = target_owner_morale_penalty {
                    tile.army.as_mut().unwrap().apply_morale_penalty(penalty, target_total_manpower);
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
