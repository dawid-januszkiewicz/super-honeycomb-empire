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
use crate::AI;

const ACTIONS_PER_TURN: i32 = 5;

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
            selection: Some(Cube::new(0,0)),
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
        write!(f, "({})", self.name)
    }
}
