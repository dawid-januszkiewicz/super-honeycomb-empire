mod cubic;
mod game;
mod world;
mod ai;

use ai::*;
use cubic::*;
use game::*;
use world::*;
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
extern {
    // #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

// #[wasm_bindgen]
pub fn main() {
    let ai1 = AI{scores: DEFAULT_SCORES};
    let ai2 = AI{scores: DEFAULT_SCORES};
    let ai3 = AI{scores: DEFAULT_SCORES};
    let ai4 = AI{scores: DEFAULT_SCORES};

    let mut player1 = Player::new("Redosia", Some(ai1));
    let player2 = Player::new("Bluegaria", Some(ai2));
    let player3 = Player::new("Greenland", Some(ai3));
    let player4 = Player::new("Violetnam", Some(ai4));

    // let mut players = vec![player1, player2];
    let mut players = vec![player1, player2, player3, player4];
    let mut world = World::new();

    // let army1 = Army {manpower: 99, morale: 99, owner_index: Some(0), can_move: true};
    // let army2 = Army {manpower: 70, morale: 70, owner_index: Some(1), can_move: true};
    // let army3 = Army {manpower: 70, morale: 70, owner_index: Some(1), can_move: true};

    // let mut army1 = Some(army1);
    // let mut army2 = Some(army2);
    // let mut army3 = Some(army3);

    // world.insert(Cube::new(0,0), Tile {owner_index: Some(0), category: TileCategory::Farmland, locality: None, army: army1});
    // world.insert(Cube::new(1,0), Tile {owner_index: Some(1), category: TileCategory::Farmland, locality: None, army: army2});
    // world.insert(Cube::new(0,-1), Tile {owner_index: Some(1), category: TileCategory::Farmland, locality: None, army: army3});

    // println!("(0,0): {:?}", world.get(&Cube::new(0,0)));
    // println!("(1,0): {:?}", world.get(&Cube::new(1,0)));

    // println!("(0,-1): {}", world.get(&Cube::new(0,-1)).unwrap());
    // println!("{:?}", world.cubes_by_ownership);

    // world.execute_army_order(&Cube::new(1,0), &Cube::new(0,0));
    // army::issue_order(&mut world, &mut players, &Cube::new(0,0), &Cube::new(0,-1));

    // println!("(0,0): {:?}", world.get(&Cube::new(0,0)));
    // println!("(1,0): {:?}", world.get(&Cube::new(1,0)));

    // println!("(0,-1): {}", world.get(&Cube::new(0,-1)).unwrap());
    // println!("{:?}", world.cubes_by_ownership);

    let mut game = Game {
        turn: 1,
        players: players,
        world: world
    };
    game.init_world();
    println!("{:?}", game.world.cubes_by_ownership);

    // game.update_world();
    // game.update_world();
    // game.update_world();
    // game.update_world();

    let mut is_yet_won = false;
    while !is_yet_won {
        // log(&format!("cbs: {:?}!", game.world.cubes_by_ownership));
        game.update_world();
        for (player_index, player_cubes) in game.world.cubes_by_ownership.iter() {
            if player_cubes.len() == game.world.len() {
                println!("Player {} won!", player_index);
                is_yet_won = true;
                break;
            }
        }
    }


    // println!("{}", game.current_player());
    // game.turn += 1;
    // println!("{}", game.current_player());
    // game.turn += 1;
    // println!("{}", game.current_player());
    // game.turn += 1;
    // println!("{}", game.current_player());
    // game.turn += 1;
    // println!("{}", game.current_player());
    // game.turn += 1;
    // println!("{}", game.current_player());

    // army::extend_borders(&mut game.world, Tile::new("farmland"), Cube::new(0,0));

    // println!("{:?}", cf.round());

    // test_cube_round();

    // println!("{:?}", c3);
    // println!("{:?}", c3.distance(&c));
    // println!("{:?}", c3);
    // println!("{:?}", c);
    // println!("{:?}", cf*5.2);
    // println!("{:?}", convert(cffromi, f64));
    // println!("{:?}", Cube::<f64>::from(c));

    // let Point1 = Point(0,0,0);
    // let Point2 = Point(1.4,1.5,1.1);
    // let Point3 = Point(3,5,-12);

    // println!("{:?}", f64::from(Point3) + 0.245);

    // let Pointfloat = PointFloat(4.2,2.3,1.11);

    // println!("{}", Point3.length());
    // println!("{}", Point3.0);
    // println!("{:?}", DIRECTIONS[0]);
    // println!("{:?}", Point3.get_neighbour(0));
    // println!("{:?}", Point2.get_nearest_neighbours());
    // println!("{:?}", Point1.get_n_nearest_neighbours(2));

    // println!("{:?}", Pointfloat.round());

    //println!("{:?}", Point2 - Point3);

    // let mut a: HashSet<char> = HashSet::from_iter(['a', 'b', 'c']);
    // let mut b: HashSet<char> = HashSet::from_iter(['d']);
    // let c: HashSet<&char> = a.union(&b).collect();
    // println!("{:?}{:?}{:?}", a, b, c);

}

// wgle moge cie tak zasypywac losowymi pytaniami o utlenku metalow?