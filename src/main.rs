mod cubic;
mod game;

use cubic::*;
use game::*;
use std::collections::HashMap;

fn main() {
    // let c = Cube(1,1);
    // let c2 = Cube(1,-1);
    // let c3 = c - c2;
    // let cf = Cube(0.4, 0.2);
    // let cffromi = Cube(1,1);
    let player0 = Player {name: String::from("None"), actions: 0};
    let player1 = Player {name: String::from("Redosia"), actions: 5};
    let player2 = Player {name: String::from("Bluegaria"), actions: 5};
    let player3 = Player {name: String::from("Greenland"), actions: 5};
    let player4 = Player {name: String::from("Violetnam"), actions: 5};
    let player5 = Player {name: String::from("Customia"), actions: 5};

    let players = vec![player1, player2, player3, player4, player5];
    let world = HashMap::new();

    let mut game = Game {
        turn: 1,
        players: players,
        current_player: &player0,
        world: world
    };

    println!("{:?}", game.current_player());
    game.turn += 1;
    println!("{:?}", game.current_player());
    game.turn += 1;
    println!("{:?}", game.current_player());
    game.turn += 1;
    println!("{:?}", game.current_player());
    game.turn += 1;
    println!("{:?}", game.current_player());
    game.turn += 1;
    println!("{:?}", game.current_player());

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