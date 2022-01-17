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

use crate::World;
use crate::cubic::Cube;
use crate::Tile;

const ACTIONS_PER_TURN: i32 = 5;

pub struct Player {
    pub name: String,
    pub actions: i32,
    pub ai: bool,
    pub selection: Option<Cube<i32>>,

    // self.camera = None
    // self.starting_cube = None
    // self.color = color
    // self.is_defeated = False
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            actions: ACTIONS_PER_TURN,
            ai: false,
            selection: Some(Cube::new(0,0)),
        }
    }
    fn player_index(&self, world: &World) -> Option<usize> {
        todo!()
        // for cube in self.cubes_owned.iter() {
        //     let first_tile = world.get(cube).unwrap();
        //     return first_tile.owner_index;
        // }
        // None
    }
    // Clicking on a tile with an army selects it. If the player
    // already has a selection, it will issue a command with the
    // clicked tile as the target of the command. If no command can
    // be issued, and the clicked tile has an army, it will be
    // selected. If it does not, the player's selection will be set
    // to None. Clicking on the selected tile deselects it.
    fn click(&mut self, mut world: &mut World, target_cube: &Cube<i32>) {
        let target = world.get(target_cube).unwrap();
        // let is_target_selectable = if let Some(army) = &target.army {
        //     && target.owner_index == self.player_index(&world)
        //     && army.can_move
        //     && Some(target_cube) != self.selection
        // };
        let mut is_target_selectable = false;
        if let Some(army) = &target.army {
            if target.owner_index == self.player_index(&world)
            && army.can_move
            && Some(*target_cube) != self.selection {
                is_target_selectable = true;
            }
        };
        if let Some(selection) = &mut self.selection {
            let legal_moves = world.get_reachable_cubes(&selection);
            if legal_moves.contains(target_cube) {
                world.execute_army_order(&selection, &target_cube);
                self.actions -= 1;
                // self.game.check_victory_condition() // call outside
                self.selection = None; // deselect
            }
        }
        if is_target_selectable {
            self.selection = Some(*target_cube);
        } else {
            self.selection = None;
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({})", self.name)
    }
}
