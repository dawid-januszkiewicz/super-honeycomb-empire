#![feature(trait_alias)]

mod cubic;
mod game;
mod world;
mod ai;
// mod pixels;
mod mquad;
mod inputs;
mod map_editor;
mod river;

use ai::*;
use cubic::*;
use game::*;
use world::*;
use inputs::*;
use map_editor::*;
use std::{fs::File, f32::consts::PI};
// use crate::pixels::*;
use mquad::*;
use macroquad::prelude::*;

const WATER_FRAGMENT_SHADER: &'static str = include_str!("../assets/water_fragment_shader.glsl");
const WATER_VERTEX_SHADER: &'static str = include_str!("../assets/water_vertex_shader.glsl");

pub trait Component {
    fn poll(&mut self, layout: &mut Layout<f32>) -> bool;
    fn draw(&self, layout: &Layout<f32>, assets: &Assets, time: f32);
    fn update(&mut self);
    // fn swap(self) -> dyn Component;
}

async fn load_assets() -> Assets {
    let f = File::open("assets/cities.json").expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(f).expect("file should be proper JSON");
    let locality_names: Vec<_> = json["data"].as_array().unwrap().iter().map(|el| el["asciiname"].to_string().replace("\"", "")).collect();
    // let locality_names = locality_names_v.iter().map(String::as_str).collect();
    // let locality_names: Vec<&str> = locality_names_v.iter().map(|s| &**s).collect();


    let font = load_ttf_font("assets/Iceberg-Regular.ttf").await.unwrap();
    let army: Texture2D = load_texture("assets/army.png").await.unwrap();
    let port: Texture2D = load_texture("assets/port.png").await.unwrap();
    let airport: Texture2D = load_texture("assets/airport.png").await.unwrap();
    let fields = load_texture("assets/grass.png").await.expect("Failed to load texture");
    let water_material = load_material(
        WATER_VERTEX_SHADER,
        WATER_FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![
                ("Time".to_owned(), UniformType::Float1),
                ("RectSize".to_owned(), UniformType::Float2),
            ],
            ..Default::default()
        },
    ).unwrap();
    let size = [32.,32.];
    let origin = [0., 0.];
    let init_layout = cubic::Layout{orientation: cubic::OrientationKind::Flat(cubic::FLAT), size, origin};
    
    water_material.set_uniform("RectSize", (init_layout.size[0], init_layout.size[1]));
    //let shape = vec!((300.,10.), (1000., 100.), (1000., 500.), (5000., 500.), (5000., 100.), (300., 10.));
    //let v: serde_json::Value = serde_json::from_str(data).unwrap();
    // let shape: Vec<(f32, f32)> = serde_json::from_str(data).unwrap();
    // Open the CSV file
    let file = File::open("assets/shapes/ua-100k.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    // Create a Vec<(f32, f32)> to store the data
    let mut shape: Vec<(f32, f32)> = Vec::new();

    // Iterate over each record in the CSV and parse the values
    for (idx, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        let first_value: f32 = record.get(0).unwrap().parse().unwrap();
        let second_value: f32 = record.get(1).unwrap().parse().unwrap();
        let vertex_part: i32 = record.get(12).unwrap().parse().unwrap();
        let vertex_part_ring: i32 = record.get(13).unwrap().parse().unwrap();

        let r = 6371000.0 / 500.; //1:250 is nearly max
        let y = r * ((std::f32::consts::PI/4.) + (second_value.to_radians()/2.)).tan().ln();
        let x = r * first_value.to_radians();
        
        if vertex_part == 158 && vertex_part_ring == 0 {
            // shape.push((first_value * r, second_value*(-1.) * r));
            shape.push((x, y * -1.));
        }
        //  if idx > 50000 {break}
    }

    let mut river: Vec<(f32, f32)> = Vec::new();
    let file = File::open("assets/shapes/ua-rivers.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    // Iterate over each record in the CSV and parse the values
    for (idx, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        let first_value: f32 = record.get(23).unwrap().parse().unwrap();
        let second_value: f32 = record.get(24).unwrap().parse().unwrap();
        // let vertex_part: i32 = record.get(12).unwrap().parse().unwrap();
        // let vertex_part_ring: i32 = record.get(13).unwrap().parse().unwrap();

        let r = 6371000.0 / 500.; //1:250 is nearly max
        let y = r * ((std::f32::consts::PI/4.) + (second_value.to_radians()/2.)).tan().ln();
        let x = r * first_value.to_radians();
        river.push((x, y * -1.));

        // if vertex_part == 158 && vertex_part_ring == 0 {
        //     // shape.push((first_value * r, second_value*(-1.) * r));
        //     shape.push((x, y * -1.));
        // }
        //  if idx > 50000 {break}
    }

    //println!("{:?}", shape);
    //let shape = vec!((0.,0.), (500., -950.), (1000., 0.), (1000.,-1000.), (500., -950.), (0.,-1000.));
    // let shape = vec!((0.,0.), (1000., 0.), (1000.,-1000.), (0.,-1000.));
    // let (min_x, min_y) = shape.iter().fold(0., |init: f32, (x, y)| (init.min(x), init.min(y)));
    // let min_x = shape.iter().fold(0., |init: f32, (x, y)| init.min(*x));
    // let min_y = shape.iter().fold(0., |init: f32, (x, y)| init.min(*y));

    Assets{locality_names, font, army, port, airport, fields, water_material, init_layout, shape, river}
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Super Honeycomb Empire".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

fn new_game(assets: &mut Assets) -> Game {
    let ai1 = AI{scores: DEFAULT_SCORES};
    let ai2 = AI{scores: DEFAULT_SCORES};
    let ai3 = AI{scores: DEFAULT_SCORES};
    let ai4 = AI{scores: DEFAULT_SCORES};

    // let player1 = Player::new("Redosia", Some(ai1));
    let player1 = Player::new("Redosia", None);
    let player2 = Player::new("Bluekraine", Some(ai2)); // Umberaine?
    // let player2 = Player::new("Bluegaria", Some(ai2));
    // let player3 = Player::new("Greenland", Some(ai3));
    // let player4 = Player::new("Violetnam", Some(ai4));

    let players = vec![player1, player2, ];//player3, player4];

    let world = World::new();

    // save_map(&game.world.world);
    // let mut world = World::from_json("assets/maps/map.json");
    // let mut world = World::from_json("assets/saves/quicksave.json");

    let mut game = Game {
        turn: 1,
        players,
        world,
        victory_condition: game::VictoryCondition::Territory(0.30)
    };

    game.init_world(assets);
    game
}

async fn game_loop(game: &mut Game, layout: &mut Layout<f32>, assets: &Assets) {
    let mut is_yet_won = false;

    let mut time = 0.0;

    while !is_yet_won {
        clear_background(DARKGRAY);

        poll_inputs(game, layout);

        if is_key_pressed(KeyCode::F1) {
            break
        }

        draw(&game, &layout, &assets, time);
    
        game.update();

        is_yet_won = game.victory_condition.check(&game.world, game.current_player_index());

        next_frame().await;
        time += get_frame_time();
    }
    println!("Player {} won!", game.current_player_index());
}

// struct App<T: Component>(T);

// fn get_app<T: Component>(assets: &mut Assets) -> dyn Component {
//     new_game(assets)
// }

enum State {
    Game,
    Editor,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut assets = load_assets().await;

    // run_editor(&assets).await;

    let mut game = new_game(&mut assets);
    let mut editor = Editor::new(World::new(), vec!());

    // let river = crate::river::generate_river(game.world.keys().collect());
    // println!("river: {:?}", river);

    let mut state = State::Game;
    let mut app: &mut dyn Component = &mut game;


    // let app: &mut dyn Component = &mut match state {
    //     State::Game(game) => game,
    //     State::Editor(editor) => editor,
    // };//&mut game;

    // let app: &mut dyn Component = &mut game;

    let mut layout = assets.init_layout.clone();

    let mut time = 0.0;
    let mut exit = false;
    while !exit {
        app.draw(&mut layout, &mut assets, time);
        exit = app.poll(&mut layout);
        next_frame().await;
        app.update();
        if is_key_pressed(KeyCode::F1) {
            match state {
                State::Game => {
                    editor = game.into();
                    game = Game{turn: 0, players: vec!(), world: World::new(), victory_condition: VictoryCondition::Elimination};
                    app = &mut editor;
                    state = State::Editor;
                }
                State::Editor => {
                    game = editor.into();
                    editor = Editor::new(World::new(), vec!());
                    app = &mut game;
                    state = State::Game;
                }
            }
            // editor = game.into::<Editor>();
        }
        time += get_frame_time();
        // game_loop(&mut game, &mut layout, &assets).await;
        // run_editor(&assets).await;
    }

}


// let size = [0.1,0.1]; // use this if in local coords
// let camera = &Camera2D {
//     // zoom: vec2(0.001, 0.001),
//     // offset: vec2(-0.5,-0.1),
//     zoom: vec2(1., -1.),
//     ..Default::default()
// };

// // set_camera(camera);