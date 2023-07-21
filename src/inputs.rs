use crate::cubic::Cube;
use crate::cubic::OrientationKind;
use crate::game::Game;
use crate::Layout;
use crate::cubic;
use crate::map_editor::Editor;
use crate::map_editor::save_map;
use crate::mquad::Assets;
use crate::world::LocalityCategory;
use crate::world::Tile;
use crate::world::World;
use macroquad::input::*;
use macroquad::prelude::*;

const PAN_SPEED: f32 = 5.;
const ZOOM_SPEED: f32 = 1.;

fn poll_camera_inputs(layout: &mut Layout<f32>) {
    let (_, mouse_wheel_y) = mouse_wheel();
    if mouse_wheel_y > 0. {
        layout.size[0] += ZOOM_SPEED;
        layout.size[1] += ZOOM_SPEED;
    } else if mouse_wheel_y < 0. {
        layout.size[0] -= ZOOM_SPEED;
        layout.size[1] -= ZOOM_SPEED;
    }
    if layout.size[0] <= 8. {layout.size[0] = 8.}
    if layout.size[1] <= 8. {layout.size[1] = 8.}

    if is_key_down(KeyCode::Right) {
        layout.origin[0] -= PAN_SPEED;
    }
    if is_key_down(KeyCode::Left) {
        layout.origin[0] += PAN_SPEED;
    }
    if is_key_down(KeyCode::Up) {
        layout.origin[1] += PAN_SPEED;
    }
    if is_key_down(KeyCode::Down) {
        layout.origin[1] -= PAN_SPEED;
    }
}

pub fn poll_map_editor_inputs(editor: &mut Editor, layout: &mut Layout<f32>) {
    if is_mouse_button_down(MouseButton::Left) {
        let pos = mouse_position().into();
        let cube = cubic::pixel_to_cube(layout, pos).round::<i32>();
        editor.click(&cube);
    }

    if is_mouse_button_pressed(MouseButton::Right) {
        editor.right_click();
    }

    if is_key_pressed(KeyCode::Tab) {
        editor.toggle_layer();
    }

    if is_key_down(KeyCode::F5) {
        //save_map(&game.world.world, "assets/saves/quicksave.json");
        save_map(&editor.world, "assets/maps/quicksave.json");
    }
    if is_key_down(KeyCode::F9) {
        *editor = Editor::new(World::from_json("assets/maps/quicksave.json"));
        // *editor.world = World::from_json("assets/maps/quicksave.json");
    }

    poll_camera_inputs(layout);
}

pub fn poll_inputs(game: &mut Game, layout: &mut Layout<f32>) {
    // if is_key_down() {
    //     let key = last_key_pressed();
    // }

    let player_index = game.current_player_index();
    let player = &game.players[player_index];

    if is_mouse_button_pressed(MouseButton::Left) & player.ai.is_none() {
        let pos = mouse_position().into();
        // let xyz = Cube::from::<i32>(Cube::new(1.,1.));
        let cube = cubic::pixel_to_cube(layout, pos).round::<i32>();
        if let Some(_) = game.world.get(&cube) {
            game.click(&cube);
        }
    }

    poll_camera_inputs(layout);

    if is_key_down(KeyCode::F5) {
        //save_map(&game.world.world, "assets/saves/quicksave.json");
        game.to_json("assets/saves/quicksave.json");
    }
    if is_key_down(KeyCode::F9) {
        *game = Game::from_json("assets/saves/quicksave.json");
    }
}

pub fn draw_tile_selector(&layout: &Layout<f32>) {
    let vertical = match layout.orientation {
        OrientationKind::Pointy(_) => true,
        OrientationKind::Flat(_) => false,
    };
    let size = layout.size[0];
    let pos = mouse_position().into();
    // let pos = mouse_position_local().into();
    let cube = cubic::pixel_to_cube(&layout, pos).round();
    let [x, y] = cube.to_pixel(&layout);
    // println!("{:?}", pos);
    let color = Color::from_rgba(224, 208, 64, 136); // 0x88d0e040
    draw_hexagon(x, y, size, size/10., vertical, BLACK, color);
}

fn draw_locality_name(layout: &Layout<f32>, cube: &Cube<i32>, tile: &Tile, font: Font) {
    let params = TextParams{
        font,
        font_size: (0.9 * layout.size[0]) as u16, // 6,
        //font_scale: (0.7 * layout.size[0]),
        ..Default::default()
    };
    if let Some(locality) = &tile.locality {
        // if let Some(category) = locality.category {}
        let color = match locality.category {
            LocalityCategory::Capital => WHITE, // tile.locality.starting_owner.color,
            _ => WHITE,
        };

        let [mut x, mut y] = Cube::<f32>::from(*cube).to_pixel(&layout);
        let center: Vec2 = get_text_center(locality.name.as_str(), Some(params.font), params.font_size, params.font_scale, params.rotation);
        x -= center.x;
        y -= layout.size[1] + center.y;
        draw_text_ex(locality.name.as_str(), x, y, params);
    }

}


// Draws names of all localities within 5 cubes of the cursor.
pub fn draw_all_locality_names(world: &World, layout: &Layout<f32>, assets: &Assets) {
    let pos = mouse_position().into();
    let cube = cubic::pixel_to_cube(&layout, pos).round();

    let cubes_within_draw_range = cube.disc(5);
    for cube in cubes_within_draw_range {
        match world.get(&cube) {
            Some(tile) if tile.locality.is_some() => draw_locality_name(layout, &cube, tile, assets.font),
            _ => {},
        }
    }
}
