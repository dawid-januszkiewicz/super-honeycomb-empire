//! Game logic module

// import ai
// import worldgen
// import playergen

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::option::Option;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use serde::Deserialize;
use serde::Serialize;

use crate::cubic::*;
use crate::Army;
use crate::Player;
use crate::World;
use crate::mquad::Assets;
use crate::world::MAX_STACK_SIZE;
use crate::world::ACTIONS_PER_TURN;

use crate::world::TileCategory;
use crate::world::gen::*;

#[derive(Serialize, Deserialize)]
pub enum VictoryCondition {
    Elimination,
    Territory(f32),
    CaptureAndHold(HashSet<Cube<i32>>),
}

impl VictoryCondition {
    fn check_territory(world: &World, player_index: usize, target_percentage: f32) -> bool {
        let world_total = world.len();
        let player_total = world.iter().filter(|&(_, tile)| tile.owner_index == Some(player_index)).count();
        player_total as f32 / world_total as f32 >= target_percentage
    }
    pub fn check(&self, world: &World, player_index: usize) -> bool {
        match self {
            Self::Elimination => unimplemented!(),
            Self::Territory(x) => VictoryCondition::check_territory(&world, player_index, *x),
            _ => unimplemented!(),
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Game {
    pub turn: i32,
    pub players: Vec<Player>,//[&'a Player<'a>],//Vec<&Player>,
    //pub current_player: &'a Player, // change it to a function?
    pub world: World,
    pub victory_condition: VictoryCondition,
}

impl From<crate::map_editor::Editor> for Game {
    fn from(value: crate::map_editor::Editor) -> Self {
        // let player_idx: std::collections::hash_map::Keys<'_, usize, HashSet<Cube<i32>>> = value.world.cubes_by_ownership.keys();
        // let mut players: Vec<Player> = vec!();
        // player_idx.for_each(|i| {
        //     let player = Player::new("", Some(crate::ai::AI::new()));
        //         players.push(player);
        //     }
        // );
        crate::Game{turn: 1, players: value.players, world: value.world, victory_condition: VictoryCondition::Elimination}
    }
}

impl From<Game> for crate::map_editor::Editor {
    fn from(value: Game) -> Self {
        crate::map_editor::Editor::new(value.world, value.players)
    }
}

impl Game {
    // pub async fn draw(&self, &layout: &Layout<f32>, assets: &Assets, time: f32) {
    //     crate::draw(&self, &layout, assets, time).await;
    // }
    // pub fn poll(&mut self, layout: &mut Layout<f32>) {
    //     crate::poll_inputs(self, layout);
    // }
    // pub fn swap(self) -> crate::map_editor::Editor {
    //     self.into()
    // }
    // pub fn update(&mut self) {
    //     self._update()
    // }
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
    pub fn init_world(&mut self, assets: &mut Assets) {
        // let shape_gen = ShapeGen::Custom(assets.shape.clone());
        let shape_gen = ShapeGen::Hexagonal(8);
        let river_gen = RiverGen::Custom(assets.river.clone());
        // let river_gen = RiverGen::Random(300, 0.3);
        let localities_gen = LocalitiesGen::Random;
        let capitals_gen = CapitalsGen::Random;
        self.world.generate(
            &mut self.players,
            shape_gen,
            river_gen,
            localities_gen,
            capitals_gen,
            &mut assets.locality_names.iter().map(|s| &**s).collect(),
            &assets.init_layout,
        );
        // println!("{}", self.world.len());
        // println!("river (debug): {:?}", self.world.rivers);
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
        println!("current_selection: {:?}", current_player.selection);
        println!("click: {:?}", target_cube);
        // let is_target_selectable = if let Some(army) = &target.army {
        //     && target.owner_index == self.player_index(&world)
        //     && army.can_move
        //     && Some(target_cube) != self.selection
        // };
        let mut is_target_selectable = false;
        if let Some(army) = &target.army {
            if army.owner_index == Some(current_player_index)
            && army.can_move // shouldnt this be origin.army.canmove?
            && Some(*target_cube) != current_player.selection {
            // && self.world.is_cube_targetable(&current_player.selection, target_cube) {
                is_target_selectable = true;
            }
        };
        if let Some(selection) = current_player.selection {
            let legal_moves = self.world.get_all_legal_moves(&selection, &current_player_index); // self.world.get_reachable_cubes(&selection);
            if legal_moves.contains(target_cube) { // && self.world.is_cube_targetable(&selection, target_cube) { // !matches!(target.category, TileCategory::Water) {
                self.world.execute_army_order(&selection, &target_cube);
                let current_player = self.current_player_mut();
                current_player.actions -= 1;
                current_player.selection = None; // deselect
                return;
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
        self.current_player_mut().actions = ACTIONS_PER_TURN;
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
    pub fn _update(&mut self) {
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

impl crate::Component for Game {
    fn draw(&self, &layout: &Layout<f32>, assets: &Assets, time: f32) {
        crate::draw(&self, &layout, assets, time);
    }
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool {
        crate::poll_inputs(self, layout)
    }
    // fn swap(self) -> crate::map_editor::Editor {
    //     self.into()
    // }
    fn update(&mut self) {
        self._update()
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