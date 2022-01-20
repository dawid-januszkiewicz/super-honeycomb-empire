/// The AI module

enum SCORES {
    Farmland(i32),
    City(i32),
    CapitalSatellite(i32),
    Capital(i32),
    Manpower(i32),
}

struct Score {
    farmland: i32,
    city: i32
    capital_satellite: i32,
    capital: i32,
    manpower: i32,
}

struct ScoredMove {
    score: Score,
    origin: Cube,
    target: Cube,
}

const SCORES = Score {
    manpower: 1,
    farmland: 1,
    city: 10,
    capital_satellite: 15,
    capital: 100,
}

/// Calculates the base score for capturing a tile.
fn calculate_tile_capture_score(tile: &Tile) -> i32:
    // TODO: implement distinguishing satellite capitals
    match tile.locality {
        Some(locality) => return SCORES.get(locality.category)
        None => return SCORES.get(tile.category)
    }

/// Calculates the bonus score for capturing neighbouring tiles of a cube.
fn calculate_extended_border_score(own_player_index: &usize, world: &World, cube: &Cube<i32>):
    let mut score = 0;
    let neighbours_cube = target_cube.disc(1); // Find the NN of the target cube
    for neighbour in neighbours_cube {
        if let Some(tile) = world.get(neighbour).unwrap() {
            if world.is_cube_passable(neighbour) && tile.owner != own_player_index {
                score += SCORES.get(tile.category);
            }
        }
    }
    score
}

/// Calculates the combat score component.
fn calculate_combat_score(world: &World, origin_cube: &Cube<i32>, target_cube: &Cube<i32>):
    let origin = world.get(origin_cube).unwrap();
    let target = world.get(target_cube).unwrap();
    let origin_army = origin.army.as_ref().unwrap();
    let target_army = target.army.as_ref().unwrap()
    let diff = origin_army.combat_strength() - target_army.combat_strength();
    let mut score = diff / 10;
    if diff > 0 {
        score += target_army.manpower / 10;
    } else {
        score -= origin_army.manpower / 10;
    }
    score

/// Calculates and returns a score value for a move.
fn calculate_score(own_player_index: &usize, world: World, origin_cube: &Cube<i32>, target_cube: &Cube<i32>) {
    let origin = world.get(origin_cube).unwrap();
    let target = world.get(target_cube).unwrap();
    if target_tile.owner != Some(own_player_index) {
        match tile.army {
            Some(target_army) => {
                let origin_army = origin.army().unwrap();
                let mut score = calculate_combat_score(&own_player_index, &world, &origin_cube, target_cube);
                let diff = origin_army.calculate_combat_strength() - target_army.calculate_combat_strength();
                if diff > 0 {
                    score += calculate_tile_capture_score(&target_tile);
                    score += calculate_extended_border_score(&own_player_index, &world, &target_cube);
                }
            }
            None => {
                score = calculate_tile_capture_score(target_tile)
                score += calculate_extended_border_score(game, target_cube)
            }
        }
    } else {
        let score = calculate_extended_border_score(&own_player_index, &world, &target_cube)
    }
    score
}

// def is_army_likely_to_be_useful(game, army_tilepair):
//     """Don't waste cycles exploring paths that aren't likely to be useful."""
//     cube, tile = army_tilepair
//     valid_targets = cubic.get_reachable_cubes(game.world, cube, army.MAX_TRAVEL_DISTANCE)
//     for target in valid_targets:
//         target_tile = game.world.get(target)
//         if target_tile.owner != tile.owner:
//             return True
//     return False


// def create_owned_armies_world_subset(game):
//     """Creates a subset of the game world containing only entries with own armies."""
//     world_subset = {}
//     for cube, tile in game.world.items():
//         if tile.army and tile.army.can_move and tile.owner == game.current_player:
//             if is_army_likely_to_be_useful(game, (cube, tile)):
//                 world_subset[cube] = tile
//     return world_subset

// def explore_army_targets(game, cube):
//     """Explores the scores a tile containing an army can achieve for all valid targets."""
//     res = []
//     # cube, tile = tilepair
//     valid_targets = cubic.get_reachable_cubes(game.world, cube, army.MAX_TRAVEL_DISTANCE)
//     for target in valid_targets:
//         score = calculate_score(game, cube, target)
//         element = TargetListElement(score, cube, target)
//         res.append(element)
//     return res

// def create_target_list(game):
//     """Score every possible valid move."""
//     subset = create_owned_armies_world_subset(game)
//     target_list = []
//     for cube in subset:
//         target_list += explore_army_targets(game, cube)
//     return target_list

// def generate_targets(game):
//     """Based on the target list, pick generate the most optimal targets."""
//     target_list = create_target_list(game)
//     target_list.sort(key=operator.itemgetter(0), reverse=True)
//     for target_list_element in target_list:
//         yield target_list_element

// def controller(game, target):
//     """Attempts to calculate the most optimal move and performs the necessary
//     click_on_tile() Player method calls to execute them.
//     """
//     origin_tile = game.world.get(target.origin)
//     target_tile = game.world.get(target.target)
//     tilepair_origin = (target.origin, origin_tile)
//     tilepair_target = (target.target, target_tile)
//     game.current_player.click_on_tile(tilepair_origin)
//     game.current_player.click_on_tile(tilepair_target)

// # def create_threat_list():
// #     """For every hostile army in the game world, calculate the threat level."""
// #     pass

// # def pick_important_threats():
// #     """Based on the threat list, pick 5 most dangerous threats."""
// #     pass

// # def pick_actions():
// #     """Pick """