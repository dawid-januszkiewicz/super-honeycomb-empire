//! Game logic module

// import ai
// import worldgen
// import playergen

use std::collections::HashMap;
use std::option::Option;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::cubic::*;
use crate::Army;
use crate::player::Player;
use crate::World;

#[derive(Debug)]
pub struct Locality {
    name: String,
    pub category: String, 
    starting_owner_index: Option<usize>,
}

impl Locality {
    fn new(name: &str) -> Self {
        Locality {
            name: name.to_string(),
            category: String::default(),
            starting_owner_index: None,
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    pub owner_index: Option<usize>,
    pub category: String,
    pub locality: Option<Locality>,
    pub army: Option<Army>,
}

impl Tile {
    pub fn new(category: &str) -> Self {
        Tile {
            owner_index: None,
            category: category.to_string(),
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
        write!(f, "({})", self.category)
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

pub struct Game {
    pub turn: i32,
    pub players: Vec<Player>,//[&'a Player<'a>],//Vec<&Player>,
    //pub current_player: &'a Player, // change it to a function?
    pub world: World,
}

impl Game { // Game<'_>
    pub fn current_player_index(&self) -> usize {
        (self.turn - 1) as usize % self.players.len()
    }
    pub fn current_player(&mut self) -> &mut Player {
        let index = self.current_player_index();
        &mut self.players[index]
    }
    fn next_turn(&mut self) {
        self.current_player().selection = None;
        self.current_player().actions = 5;
        self.train_armies();
        // Reset army movement points
        self.apply_idle_morale_penalty();
        for tile in self.world.values_mut() {
            if let Some(army) = &mut tile.army {
                army.can_move = true;
            }
            // This is eqivalent but w/o pattern matching:
            // if tile.army.is_some() {
            //     let army = tile.army.as_mut().unwrap();
            //     army.can_move = true;
            //}
        }

        self.turn += 1;
        let turn = self.turn;
        println!("turn: {}, player: {}", turn, self.current_player());
    }
    pub fn update_world(&mut self) {
        if self.players.len() <= 1 {
            println!("{} wins!", self.current_player());
        }

        // Force a player to skip a turn if he has no units to move or no action points left.
        let can_player_issue_a_command = Game::can_player_issue_a_command(self.current_player_index());
        if self.current_player().actions == 0 || !can_player_issue_a_command {
            self.next_turn();
        }

        // Let AI make a move
        // if self.current_player().ai {
        //     target_generator = ai.generate_targets(self)
        //     for target in target_generator:
        //         if self.current_player.actions > 0:
        //             ai.controller(self, target)
        //         else:
        //             break
        //     self.current_player.skip_turn()
        // }
    }
    fn train_armies(&self) {
        let mut tiles_owned_by_player = vec![];
        for tile in self.world.values() {
            if tile.owner_index == Some(self.current_player_index()) {
                tiles_owned_by_player.push(tile)
            }
        }

        // First apply base growth
        // for tile in tiles_owned_by_player:
        //     if not tile.locality:
        //         continue
        //     if tile.locality.category == "City":
        //         if not tile.army:
        //             tile.army = army.Army(army.BASE_GROWTH_CITY, 1/2 * army.BASE_GROWTH_CITY, tile.owner)
        //         elif tile.army.manpower < army.MAX_STACK_SIZE:
        //             tile.army.manpower += army.BASE_GROWTH_CITY
        //             tile.army.morale += 1/2 * army.BASE_GROWTH_CITY
        //     elif tile.locality.category == "Capital":
        //         if not tile.army:
        //             tile.army = army.Army(army.BASE_GROWTH_CAPITAL, 1/2 * army.BASE_GROWTH_CAPITAL, tile.owner)
        //         elif tile.army.manpower < army.MAX_TRAVEL_DISTANCE:
        //             tile.army.manpower += army.BASE_GROWTH_CITY
        //             tile.army.morale += 1/2 * army.BASE_GROWTH_CITY
        //     if tile.army:
        //         if tile.army.manpower > army.MAX_STACK_SIZE:
        //             tile.army.manpower = army.MAX_STACK_SIZE
        //         if tile.army.morale > army.MAX_STACK_SIZE:
        //             tile.army.morale = army.MAX_STACK_SIZE

//         # Then apply bonus growth
//         player_bonus_growth = len(tiles_owned_by_player) * army.BONUS_GROWTH_PER_TILE
//         while player_bonus_growth > 0:
//             tiles_with_max_army_stack = 0
//             for tile in tiles_owned_by_player:
//                 if tile.locality and tile.army.manpower < army.MAX_STACK_SIZE:
//                     tile.army.manpower += 1
//                     tile.army.morale += 0.5
//                     player_bonus_growth -= 1
//                 else:
//                     tiles_with_max_army_stack += 1

//                 # break if we can't apply the bonus anywhere
//                 if len(tiles_owned_by_player) == tiles_with_max_army_stack:
//                     player_bonus_growth = 0
//                     break

//         # Round morale values
//         for tile in self.world.values():
//             if tile.army:
//                 tile.army.morale = min(army.MAX_STACK_SIZE, tile.army.morale)
//                 tile.army.morale = round(tile.army.morale)
    }
    fn apply_idle_morale_penalty(&self) {
        todo!();
    }
    fn can_player_issue_a_command(player_index: usize) -> bool {
        todo!();
    }
}

// class Game:
//     playergen : object
//         an infinite generator provided by itertools.cycle
//     """
//     def __init__(self):
//         self.turn = 0
//         self.players = playergen.classic(self)
//         self.playergen = itertools.cycle(self.players)
//         self.current_player = self.players[0]
//         #self.world = worldgen.generate_world(shape='classic', radius=6, algorithm='random_ots', spawntype='classic', players=self.players)
//         self.world = worldgen.generate_world(shape='hexagon', radius=20, algorithm='random_ots', spawntype='random', players=self.players)
//         self.initial_layout = worldgen.layout
//         playergen.create_player_cameras(self)

//     def defeat_player(self, player):
//         """Removes a player instance from the players list."""
//         player.is_defeated = True
//         print(player, "has been defeated!")

//     def surrender_to_player(self, defeated_player, player):
//         """Transfer the ownership of all of defeated_player tiles to player."""
//         for tile in self.world.values():
//             if tile.owner == defeated_player:
//                 tile.army = None
//                 tile.owner = player



//     def can_player_issue_a_command(self, player):
//         own_tiles = []
//         for tile in self.world.values():
//             if tile.owner == player and tile.army and tile.army.can_move:
//                 own_tiles.append(tile)
//         if len(own_tiles) == 0:
//             return False
//         return True

//     def check_victory_condition(self):
//         for player in self.players:
//             starting_tile = self.world.get(player.starting_cube)
//             if starting_tile.owner != player and not player.is_defeated:
//                 self.defeat_player(player)
//                 self.surrender_to_player(player, starting_tile.owner)

//     def apply_idle_morale_penalty(self):
//         """Applies a morale penalty to idle armies."""
//         for tile in self.world.values():
//             if  (tile.army and tile.army.can_move
//                  and tile.owner == self.current_player):

//                 tile.army.morale -= army.MORALE_PENALTY_IDLE_ARMY
//                 tile.army.morale = max(tile.army.morale, army.calculate_minimum_morale(self.world.values(), tile.army))

//     def print_world_state(self):
//         """A debugging function used in graphicless mode."""
//         for tile in self.world.values():
//             if tile.army:
//                 print(tile, tile.owner, tile.army)

// def main():
//     """For debugging purposes"""
//     game = Game()
//     game.current_player.actions = 0
//     while len(game.players) > 1:
//         game.update_world()