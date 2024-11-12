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
mod shapefiles;
mod fog;
mod rules;
mod network;
mod cli;
mod ui;

use clap::Parser;
use fog::*;
use ai::*;
use cli::*;
use cubic::*;
use game::*;
use miniquad::native::linux_x11::libx11::VisibilityChangeMask;
use rules::Ruleset;
use ui::main_menu;
use world::*;
use inputs::*;
use map_editor::*;
use network::*;
use std::{collections::HashMap, f32::consts::PI, fs::File};
// use crate::pixels::*;
use mquad::*;
// use macroquad::{file::load_file, miniquad::fs::load_file, prelude::*};
use macroquad::{file::load_file, prelude::*};
use dbase;

const WATER_FRAGMENT_SHADER: &'static str = include_str!("../assets/water_fragment_shader.glsl");
const WATER_VERTEX_SHADER: &'static str = include_str!("../assets/water_vertex_shader.glsl");

fn load_rivers(shape: &Vec<(f32, f32)>) -> Vec<(usize, f32, f32)> {
    let x_min = shape.iter().fold(f32::NAN, |a, &b| a.min(b.0));
    let y_min = shape.iter().fold(f32::NAN, |a, &b| a.min(b.1));
    // shape = shape.iter().map(|(x, y)| (x - x_min, y - y_min)).collect();

    let mut river: Vec<(usize, f32, f32)> = Vec::new();
    let file = File::open("assets/shapes/ua-rivers.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    // Iterate over each record in the CSV and parse the values
    for (idx, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        let first_value: f32 = record.get(23).unwrap().parse().unwrap();
        let second_value: f32 = record.get(24).unwrap().parse().unwrap();
        let id: i32 = record.get(0).unwrap().parse().unwrap();
        let id: usize = id.abs() as usize;
        // match record.get(12).unwrap().parse() {
        //     Ok(_) => {},
        //     Err(e) => print!("{:}", e),
        // }
        // let vertex_part: i32 = record.get(12).unwrap().parse().unwrap();
        // let vertex_part_ring: i32 = record.get(13).unwrap().parse().unwrap();



        let r = 6371000.0 / 750.; //1:250 is nearly max
        let y = r * ((std::f32::consts::PI/4.) + (second_value.to_radians()/2.)).tan().ln();
        let x = r * first_value.to_radians();
        let val = (id, x - x_min, (y * -1.) - y_min);
        // if id == 4029011 {river.push((id, x, y * -1.));}
        if id == 25582 {river.push(val);}
        // river.push(val);



        // if vertex_part == 158 && vertex_part_ring == 0 {
        //     // shape.push((first_value * r, second_value*(-1.) * r));
        //     shape.push((x, y * -1.));
        // }
        //  if idx > 50000 {break}
    }
    river
}

async fn load_assets() -> Assets {
    // let mut reader = dbase::Reader::from_path("assets/ua_shp/ukr_admbnda_adm0_sspe_20230201.dbf").unwrap();
    // let f = File::open("assets/cities.json").expect("file should open read only");
    let f = include_bytes!("../assets/cities.json");
    let json: serde_json::Value = serde_json::from_reader(&f[..]).expect("file should be proper JSON");
    let locality_names: Vec<_> = json["data"].as_array().unwrap().iter().map(|el| el["asciiname"].to_string().replace("\"", "")).collect();
    // let locality_names = locality_names_v.iter().map(String::as_str).collect();
    // let locality_names: Vec<&str> = locality_names_v.iter().map(|s| &**s).collect();

    let fb = include_bytes!("../assets/Iceberg-Regular.ttf");
    let font = load_ttf_font_from_bytes(fb).unwrap();
    // let font = load_ttf_font("assets/Iceberg-Regular.ttf").await.unwrap();
    // let army = Texture2D::from_file_with_format(
    //     include_bytes!("../assets/army.png"),
    //     None,
    // );
    // let army: Texture2D = load_texture("assets/army.png").await.unwrap();
    let army_f = macroquad::prelude::load_file("army.png").await.unwrap();
    let army = Texture2D::from_file_with_format(&army_f, None);

    // let port: Texture2D = load_texture("assets/port.png").await.unwrap();
    let port = Texture2D::from_file_with_format(
        include_bytes!("../assets/port.png"),
        None,
    );

    let airport = Texture2D::from_file_with_format(
        include_bytes!("../assets/airport.png"),
        None,
    );
    // let airport: Texture2D = load_texture("assets/airport.png").await.unwrap();

    let fields = Texture2D::from_file_with_format(
        include_bytes!("../assets/grass.png"),
        None,
    );
    // let fields = load_texture("assets/grass.png").await.expect("Failed to load texture");

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
    // std::fs::create_dir_all("assets/shapes");
    let vertices = shapefiles::extract_vertices("assets/ua_shp/ukr_admbnda_adm0_sspe_20230201.shp").unwrap();
    // let file = File::open("assets/shapes/ua-100k_v2.csv").unwrap();
    // let mut rdr = csv::Reader::from_reader(file);
    // let file = load_file(path);

    // Create a Vec<(f32, f32)> to store the data
    let mut shape: Vec<(f32, f32)> = Vec::new();

    // Iterate over each record in the CSV and parse the values
    for idx in 0..vertices.0.len() {
        let first_value = vertices.0.get(idx).unwrap();//row.get(0).unwrap();
        let second_value = vertices.1.get(idx).unwrap(); //row.get(1).unwrap();
        let vertex_part = vertices.2.get(idx).unwrap(); //row.get(2).unwrap().round() as i32;
        // when using qgis-derived file
        // let vertex_part: i32 = record.get(12).unwrap().parse().unwrap();
        // let vertex_part_ring: i32 = record.get(13).unwrap().parse().unwrap();

        let r = 6371000.0 / 750.; //1:250 is nearly max
        let y = r * ((std::f32::consts::PI/4.) + (second_value.to_radians()/2.)).tan().ln();
        let x = r * first_value.to_radians();
        
        if *vertex_part == 158 {//&& vertex_part_ring == 0 { // include rhs when using qgis-derived file
            // shape.push((first_value * r, second_value*(-1.) * r));
            shape.push((x, y * -1.));
        }
        //  if idx > 50000 {break}
    }

    // let river = load_rivers(&shape);
    let river = vec![];

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
    let player1 = Player::new("Redosia", Controller::Human);
    // let player2 = Player::new("Bluekraine", Some(ai2)); // Umberaine?
    let player2 = Player::new("Bluegaria", Controller::AI(ai2));
    let player3 = Player::new("Greenland", Controller::AI(ai3));
    // let player4 = Player::new("Violetnam", Some(ai4));

    // let players: Vec<Player> = vec![player1, player2, player3];
    let players: Vec<Player> = Vec::new();

    // let players = vec![player1, player2, player3, player4];
    // let players = vec![player1, player2, ];//player3, player4];

    // let world = World::new();
    // // save_map(&game.world.world);
    // // let mut world = World::from_json("assets/maps/map.json");
    // // let mut world = World::from_json("assets/saves/quicksave.json");

    Game::new(players, assets)
}

// async fn game_loop(game: &mut Game, layout: &mut Layout<f32>, assets: &Assets) {
//     let mut is_yet_won = false;

//     let mut time = 0.0;

//     while !is_yet_won {
//         clear_background(DARKGRAY);

//         poll_inputs(game, layout);

//         if is_key_pressed(KeyCode::F1) {
//             break
//         }

//         draw(&game, &layout, &assets, time);
    
//         game.update();

//         is_yet_won = game.rules.victory_condition.check(&game.world, game.current_player_index());

//         next_frame().await;
//         time += get_frame_time();
//     }
//     println!("Player {} won!", game.current_player_index());
// }

// struct App<T: Component>(T);

// fn get_app<T: Component>(assets: &mut Assets) -> dyn Component {
//     new_game(assets)
// }

enum Endpoint_ <T: Component> {
    Client(Client<T>),
    Server(Server<T>),
}

#[macroquad::main(window_conf)]
async fn main() {
    ui::test_ui();
    set_pc_assets_folder("assets");
    let mut assets = load_assets().await;

    // run_editor(&assets).await;

    let mut game = new_game(&mut assets);

    // Game {turn: 0, Vec::new(), World::new(), HashMap::new(), Ruleset::new()}

    // let river = crate::river::generate_river(game.world.keys().collect());
    // println!("river: {:?}", river);

    // let mut app: &mut dyn Component = &mut game;

    //// let mut app_endpoint: &mut dyn Endpoint_ = &mut endpoint;
    //// have a Endpoint_<Game> and an Endpoint_<Editor> and swap between the two

    let args = Cli::parse();
    match args.mode {
        Mode::Client => println!("Running in client mode"),
        Mode::Server => println!("Running in server mode"),
    }

    // let mut endpoint_game = match mode {
    //     "client" => Endpoint_::Client(Client::new(game, "").unwrap()),
    //     "server" => Endpoint_::Server(Server::new(game, "").unwrap()),
    // };
    // let mut endpoint_editor = match mode {
    //     "client" => Endpoint_::Client(Client::new(editor, "").unwrap()),
    //     "server" => Endpoint_::Server(Server::new(editor, "").unwrap()),
    // };
    // let &mut endpoint = &mut endpoint_game;



    // endpoint = match endpoint {
    //     Endpoint_::Client(e) => {
    //         Endpoint_::Client(Client::new(e.app.swap(), "").unwrap())
    //     },
    //     Endpoint_::Server(e) => {todo!()},
    // };

    let mut endpoint: Box<dyn Endpoint> = match args.mode { // possibly replace Box<dyn Endpoint> with trait Endpoint if and when existential types are stabilised
        Mode::Client => Box::new(Client::new(game, &args.addrs).unwrap()),
        Mode::Server => Box::new(Server::new(game, &args.addrs).unwrap()),
    };
    println!("endpoint initialised!");
    // endpoint = Box::new(endpoint.swap_app())
    
    // let mut client = Client::new(game, "").unwrap();
    // let mut client_e = Client::new(Editor::new(World::new(), Vec::new()), "").unwrap();
    // let mut server = Server::new(new_game(&mut assets), "").unwrap();
    // let mut server_e = Server::new(Editor::new(World::new(), Vec::new()), "").unwrap();
    // let mut endpoint: &mut dyn Endpoint = &mut client;

    // endpoint.app = endpoint.app.swap();
    // either box and getters and setters or
    // enum and matching 


    // let app: &mut dyn Component = &mut match state {
    //     State::Game(game) => game,
    //     State::Editor(editor) => editor,
    // };//&mut game;

    // let app: &mut dyn Component = &mut game;

    let mut layout = assets.init_layout.clone();

    let mut time = 0.0;
    let mut exit = false;
    while !exit {
        exit = endpoint.poll(&mut layout);
        endpoint.draw(&mut layout, &mut assets, time);
        main_menu();
        next_frame().await;
        endpoint = endpoint.update();
        // if is_key_pressed(KeyCode::F1) {
        //     endpoint = endpoint.swap_app();
        // }
        time += get_frame_time();
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