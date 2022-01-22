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
use crate::Player;
use crate::World;
use crate::world::MAX_STACK_SIZE;

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
        let target = self.world.get(target_cube).unwrap();
        let current_player_index = self.current_player_index();
        let current_player = self.current_player();
        // println!("current_selection: {:?}", current_player.selection);
        // println!("click: {:?}", target_cube);
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
        self.world.train_armies(&current_player_index);

        // Reset army movement points
        self.world.apply_idle_morale_penalty(current_player_index);
        for tile in self.world.values_mut() {
            if let Some(army) = &mut tile.army {
                army.can_move = true;
            }
        }

        self.turn += 1;
        println!("Turn {}: {}", self.turn, self.current_player());
    }
    pub fn update_world(&mut self) {
        let current_player_index = self.current_player_index();
        if self.players.len() <= 1 {
            println!("{} wins!", self.current_player());
        }

        // Force a player to skip a turn if he has no units to move or no action points left.
        let can_player_issue_a_command = self.world.can_player_issue_a_command(&current_player_index);
        if self.current_player().actions == 0 || !can_player_issue_a_command {
            self.next_turn();
            return
        }

        // Let AI make a move
        if let Some(ai) = &self.current_player().ai {
            let targets = ai.generate_targets(&current_player_index, &self.world);
            for target in targets {
                if self.current_player().actions > 0 {
                    self.click(&target.origin);
                    self.click(&target.target);
                } else {
                    break
                }
            }
            self.current_player_mut().skip_turn();
        }
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

//     def check_victory_condition(self):
//         for player in self.players:
//             starting_tile = self.world.get(player.starting_cube)
//             if starting_tile.owner != player and not player.is_defeated:
//                 self.defeat_player(player)
//                 self.surrender_to_player(player, starting_tile.owner)

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