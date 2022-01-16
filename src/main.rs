mod cubic;
mod game;
mod player;
mod world;

use cubic::*;
use game::*;
use player::*;
use world::*;
use std::collections::HashMap;

fn main() {
    let player1 = Player::new("Redosia");
    let player2 = Player::new("Bluegaria");
    let player3 = Player::new("Greenland");
    let player4 = Player::new("Violetnam");
    let mut players = vec![player1, player2, player3, player4];
    let map: HashMap<Cube<i32>, Tile> = HashMap::new();
    let mut world = World(map);
    // world.insert(Cube::new(0,0), Tile::new("a"));
    // world.insert(Cube::new(1,0), Tile::new("b"));
    // world.insert(Cube::new(0,1), Tile::new("c"));
    // world.insert(Cube::new(1,1), Tile::new("d"));
    // world.insert(Cube::new(0,-1), Tile::new("e"));
    // world.insert(Cube::new(-1,0), Tile::new("f"));
    // world.insert(Cube::new(-1,-1), Tile::new("g"));
    // world.insert(Cube::new(-2,-1), Tile::new("h"));
    // world.insert(Cube::new(-1,-2), Tile::new("i"));
    // world.insert(Cube::new(-2,-2), Tile::new("j"));
    // world.insert(Cube::new(-2,0), Tile::new("k"));
    // world.insert(Cube::new(0,-2), Tile::new("l"));
    // world.insert(Cube::new(1,2), Tile::new("m"));
    // world.insert(Cube::new(2,1), Tile::new("n"));
    // world.insert(Cube::new(2,0), Tile::new("o"));
    // world.insert(Cube::new(0,2), Tile::new("p"));
    // world.insert(Cube::new(1,-1), Tile::new("r"));
    // world.insert(Cube::new(-1,1), Tile::new("s"));
    // world.insert(Cube::new(2,-2), Tile::new("t"));
    // world.insert(Cube::new(2,-1), Tile::new("u"));
    // world.insert(Cube::new(1,-2), Tile::new("v"));

    let army1 = Army {manpower: 50, morale: 22, owner_index: Some(0), can_move: true};
    let army2 = Army {manpower: 88, morale: 44, owner_index: Some(1), can_move: true};
    let army3 = Army {manpower: 88, morale: 1, owner_index: Some(2), can_move: true};

    let mut army1 = Some(army1);
    let mut army2 = Some(army2);
    let mut army3 = Some(army3);

    world.insert(Cube::new(0,0), Tile {owner_index: Some(0), category: "farmland".to_string(), locality: None, army: army1});
    world.insert(Cube::new(1,0), Tile {owner_index: Some(1), category: "farmland".to_string(), locality: None, army: army2});
    world.insert(Cube::new(0,-1), Tile {owner_index: Some(2), category: "farmland".to_string(), locality: None, army: army3});

    println!("(0,0): {:?}", world.get(&Cube::new(0,0)));
    println!("(1,0): {:?}", world.get(&Cube::new(1,0)));
    println!("(0,-1): {:?}", world.get(&Cube::new(0,-1)));

    world.execute_army_order(&Cube::new(1,0), &Cube::new(0,0));
    // army::issue_order(&mut world, &mut players, &Cube::new(0,0), &Cube::new(0,-1));

    println!("(0,0): {:?}", world.get(&Cube::new(0,0)));
    println!("(1,0): {:?}", world.get(&Cube::new(1,0)));
    println!("(0,-1): {:?}", world.get(&Cube::new(0,-1)));

    let mut game = Game {
        turn: 1,
        players: players,
        world: world
    };

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