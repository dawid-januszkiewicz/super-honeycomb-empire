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

const MAX_TRAVEL_DISTANCE: i32 = 2;
const BASE_GROWTH_CITY: i32 = 5;
const BASE_GROWTH_CAPITAL: i32 = 10;
const BONUS_GROWTH_PER_TILE: i32 = 1;
const MORALE_PENALTY_IDLE_ARMY: i32 = 1;

pub struct Player {
    pub name: String,
    pub actions: i32,
    pub ai: bool,
    pub selection: Option<Cube<i32>>,
    pub cubes_owned: HashSet<Cube<i32>>,

    // self.game = game
    // self.camera = None
    // self.actions = ACTIONS_PER_TURN
    // self.starting_cube = None
    // self.color = color
    // self.is_defeated = False
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            actions: 5,
            ai: false,
            selection: Some(Cube::new(0,0)),
            cubes_owned: HashSet::new(),
        }
    }
    // Clicking on a tile with an army selects it. If the player
    // already has a selection, it will issue a command with the
    // clicked tile as the target of the command. If no command can
    // be issued, and the clicked tile has an army, it will be
    // selected. If it does not, the player's selection will be set
    // to None. Clicking on the selected tile deselects it.
    fn click(&mut self, mut world: &mut HashMap<Cube<i32>, Tile>, target_cube: &Cube<i32>) {
        let target = world.get(target_cube).unwrap();
        // let is_target_selectable = if let Some(army) = &target.army {
        //     && clicked_tile.owner == self.game.current_player
        //     && target.army.can_move
        //     && Some(target_cube) != self.selection
        // };
        // if let Some(selection) = &mut self.selection {
        //     let legal_moves = cubic.get_reachable_cubes(self.game.world, self.selection[0], player.MAX_TRAVEL_DISTANCE);
        //     if target_cube in legal_moves {
        //         issue_order(self.game.world, &self.selection, &target_cube)
        //         self.actions -= 1;
        //         self.game.check_victory_condition()
        //         self.selection = None // deselect
        //     }
        //     None => {}
        // if is_target_selectable {
        //     self.selection = target_cube;
        // } else {
        //     self.selection = None;
        // }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", self.name)
    }
}
