//! Game logic module

// import ai
// import worldgen
// import playergen

use std::collections::HashMap;
use std::collections::HashSet;
use std::option::Option;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::cubic::*;
use crate::Army;
use crate::player::Player;
use crate::World;
use crate::world::MAX_STACK_SIZE;
use crate::apply_idle_morale_penalty;

const BASE_GROWTH_CITY: i32 = 5;
const BASE_GROWTH_CAPITAL: i32 = 10;
const BONUS_GROWTH_PER_TILE: i32 = 1;

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
    pub fn current_player_mut(&mut self) -> &mut Player {
        let index = self.current_player_index();
        &mut self.players[index]
    }
    pub fn current_player(&self) -> &Player {
        let index = self.current_player_index();
        &self.players[index]
    }
        // Clicking on a tile with an army selects it. If the player
    // already has a selection, it will issue a command with the
    // clicked tile as the target of the command. If no command can
    // be issued, and the clicked tile has an army, it will be
    // selected. If it does not, the player's selection will be set
    // to None. Clicking on the selected tile deselects it.
    pub fn click(&mut self, target_cube: &Cube<i32>) {
        let target = self.world.remove(target_cube).unwrap();
        let current_player_index = self.current_player_index();
        let current_player = self.current_player();
        // let is_target_selectable = if let Some(army) = &target.army {
        //     && target.owner_index == self.player_index(&world)
        //     && army.can_move
        //     && Some(target_cube) != self.selection
        // };
        let mut is_target_selectable = false;
        if let Some(army) = &target.army {
            if target.owner_index == Some(current_player_index)
            && army.can_move
            && Some(*target_cube) != current_player.selection {
                is_target_selectable = true;
            }
        };
        if let Some(selection) = current_player.selection {
            let legal_moves = self.world.get_reachable_cubes(&selection);
            if legal_moves.contains(target_cube) {
                self.world.insert(*target_cube, target);
                self.world.execute_army_order(&selection, &target_cube);
                let current_player = self.current_player_mut();
                current_player.actions -= 1;
                // self.game.check_victory_condition()
                current_player.selection = None; // deselect
            }
        }
        let current_player = self.current_player_mut();
        if is_target_selectable {
            current_player.selection = Some(*target_cube);
        } else {
            current_player.selection = None;
        }
    }
    fn next_turn(&mut self) {
        let current_player_index = self.current_player_index();
        self.current_player_mut().selection = None;
        self.current_player_mut().actions = 5;
        self.train_armies();

        // Reset army movement points
        apply_idle_morale_penalty(&mut self.world, current_player_index);
        for tile in self.world.values_mut() {
            if let Some(army) = &mut tile.army {
                army.can_move = true;
            }
        }

        self.turn += 1;
        println!("turn: {}, player: {}", self.turn, self.current_player());
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
    fn train_armies(&mut self) {
        let current_player_index = self.current_player_index();
        let cubes_of_current_player = self.world.cubes_by_ownership.get(&current_player_index).unwrap().clone();

        // First apply base growth
        for cube in &cubes_of_current_player {
            let mut tile = self.world.get_mut(&cube).unwrap();
            let mut growth = 0;
            match &tile.locality {
                None => {continue},
                Some(locality) if locality.category == "City" => {
                    growth = BASE_GROWTH_CITY;
                },
                Some(locality) if locality.category == "Capital" => {
                    growth = BASE_GROWTH_CAPITAL;
                },
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
        let mut bonus_growth = cubes_of_current_player.len() as i32 * BONUS_GROWTH_PER_TILE;
        let mut tiles_with_max_army_stack = HashSet::new();
        for cube in cubes_of_current_player.iter().cycle() {
            // break if we can't apply the bonus anywhere
            if cubes_of_current_player.difference(&tiles_with_max_army_stack).collect::<HashSet<_>>().len() == 0 || bonus_growth <= 0 {
                break;
            }
            let tile = self.world.get_mut(&cube).unwrap();
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