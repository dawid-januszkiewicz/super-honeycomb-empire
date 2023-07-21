#![feature(trait_alias)]

mod cubic;
mod game;
mod world;
mod ai;
// mod pixels;
mod mquad;
mod inputs;
mod map_editor;

use ai::*;
use cubic::*;
use game::*;
use world::*;
use inputs::*;
use map_editor::*;
use std::{collections::{HashMap, HashSet}, fs::File};
// use crate::pixels::*;
use mquad::*;
use macroquad::prelude::*;

const WATER_FRAGMENT_SHADER: &'static str = include_str!("../assets/water_fragment_shader.glsl");
const WATER_VERTEX_SHADER: &'static str = include_str!("../assets/water_vertex_shader.glsl");

async fn load_assets() -> Assets {
    let f = File::open("assets/cities.json").expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(f).expect("file should be proper JSON");
    let locality_names: Vec<_> = json["data"].as_array().unwrap().iter().map(|el| el["asciiname"].to_string().replace("\"", "")).collect();
    // let locality_names = locality_names_v.iter().map(String::as_str).collect();
    // let locality_names: Vec<&str> = locality_names_v.iter().map(|s| &**s).collect();


    let font = load_ttf_font("assets/Iceberg-Regular.ttf").await.unwrap();
    let army: Texture2D = load_texture("assets/army.png").await.unwrap();
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
    Assets{locality_names, font, army, airport, fields, water_material}
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Super Honeycomb Empire".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut assets = load_assets().await;

    run_editor(&assets).await;

    let ai1 = AI{scores: DEFAULT_SCORES};
    let ai2 = AI{scores: DEFAULT_SCORES};
    let ai3 = AI{scores: DEFAULT_SCORES};
    let ai4 = AI{scores: DEFAULT_SCORES};

    // let player1 = Player::new("Redosia", Some(ai1));
    let player1 = Player::new("Redosia", None);
    let player2 = Player::new("Bluegaria", Some(ai2));
    let player3 = Player::new("Greenland", Some(ai3));
    let player4 = Player::new("Violetnam", Some(ai4));

    let players = vec![player1, player2, player3, player4];

    let mut world = World::new();

    // save_map(&game.world.world);
    // let mut world = World::from_json("assets/maps/map.json");
    // let mut world = World::from_json("assets/saves/quicksave.json");

    let mut game = Game {
        turn: 1,
        players,
        world,
        victory_condition: game::VictoryCondition::Territory(0.85)
    };

    game.init_world(&mut assets);

    let size = [32.,32.];
    // let size = [0.1,0.1]; // use this if in local coords
    let origin = [300., 300.];//[100.,600.];//[100.,300.];
    let mut layout = cubic::Layout{orientation: cubic::OrientationKind::Flat(cubic::FLAT), size, origin};

    assets.water_material.set_uniform("RectSize", (size[0], size[1]));

    let mut is_yet_won = false;

    let camera = &Camera2D {
        // zoom: vec2(0.001, 0.001),
        // offset: vec2(-0.5,-0.1),
        zoom: vec2(1., -1.),
        ..Default::default()
    };

    // set_camera(camera);

    let mut time = 0.0;

    while !is_yet_won {
        clear_background(DARKGRAY);

        poll_inputs(&mut game, &mut layout);

        draw(&game, &layout, &assets, time);
    
        game.update_world();

        is_yet_won = game.victory_condition.check(&game.world, game.current_player_index());

        next_frame().await;
        time += get_frame_time();
    }
    println!("Player {} won!", game.current_player_index());

}
