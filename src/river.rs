// River tiles exist along the edges of hex tiles.
// They are represented by Cube<f32>
// For example, to represent a river tile in between (1, 0, 1) and (1, -1, 0)
// the position (1, -0.5, 0.5) is chosen

use macroquad::miniquad::start;
use rand::seq::index::sample;

use crate::Cube;
use crate::DIRECTIONS;
use crate::REV_DIRECTIONS;

use std::collections::HashSet;
use std::ops::Deref;
use std::ops::Index;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

// Cube(1, -0.5, 0.5) becomes CubeSide(Cube(1, -0, 0), Cube(false, true, true))
// q = side.int.q + (side.half.q).copysign(side.int.q)
// CubeSide = (Cube, avg(Cube, (Cube + dir)).floatpart.asbool)
// HOWEVER, signed 0 does not exist, which introduces the need for the 3rd component sign
// so now
// Cube(1, -0.5, 0.5) becomes CubeSide(Cube(1, 0, 0), Cube(false, true, true), Cube(true, false, true))
// q = side.int.q + ( side.half.q * (if side.sign.q { 1.0 } else { -1.0 }) )
// OR one could simply always add/always sub the half component, removing the need for the sign component.
// todo: replace half with DIRECTION, make it an enum?
#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CubeSide {pub int: Cube<i32>, pub half: Cube<bool>, pub sign: Cube<bool>}


impl CubeSide {
    fn is_first_decimal_half<T: Into<f32> + Copy + Display>(value: T) -> bool {
        let first_decimal = (value.into() * 10.0).abs() as u32 % 10;
        let half = match first_decimal {
            5 => true,
            0 => false,
            _ => panic!("Invalid CubeSide: {:}, first_decimal: {:}", value, first_decimal),
        };
        half
    }

    fn new<T: Into<f32> + Copy + Display, U: Into<f32> + Copy + Display>(q: T, r: U) -> Self {
        let q_int = q.into() as i32;
        let r_int = r.into() as i32;
        let q_half = CubeSide::is_first_decimal_half(q);
        let r_half = CubeSide::is_first_decimal_half(r);
        let q_sign = q.into().is_sign_positive();
        let r_sign = r.into().is_sign_positive();
        // println!("q:{:}, qint:{:}, qhalf:{:}, qsign:{:}, r:{:}, rint:{:}, rhalf:{:}, rsign:{:}", q, q_int, q_half, q_sign, r, r_int, r_half, r_sign);
        let s = Self {int: Cube::new(q_int, r_int),
              half: Cube::new(q_half, r_half),
              sign: Cube::new(q_sign, r_sign)};
        println!("{:}", s);
        s
    }

    pub fn to_float(self) -> Cube<f32> {
        let q = self.int.q() as f32 + (if self.half.q() {0.5} else {0.}) * (if self.sign.q() {1.} else {-1.});
        let r = self.int.r() as f32 + (if self.half.r() {0.5} else {0.}) * (if self.sign.r() {1.} else {-1.});
        Cube::new(q, r)
    }
}

impl From<Cube<f32>> for CubeSide {
    fn from(cube: Cube<f32>) -> Self {
        Self::new(cube.q(), cube.r())
    }
}

impl Display for CubeSide {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let cube_float = self.to_float();
        // let q = self.int.q() as f32 + (self.half.q() as i32 as f32 / 2.);
        // let r = self.int.r() as f32 + (self.half.r() as i32 as f32 / 2.);
        write!(f, "({}, {})", cube_float.q(), cube_float.r())
    }
}

impl Debug for CubeSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let cube_float = self.to_float();
        // let q = self.int.q() as f32 + ((self.half.q() as i32) as f32 / 2.);
        // let r = self.int.r() as f32 + ((self.half.r() as i32) as f32 / 2.);
        // let s = -q -r;
        f.debug_tuple("Cube")
         .field(&cube_float.q())
         .field(&cube_float.r())
         .field(&cube_float.s())
         .finish()
    }
}



// let rivers: HashSet<Cube<f32>> = HashSet::new();

pub fn generate_river(world: HashSet<&Cube<i32>>) -> HashSet<CubeSide> {
    let mut river = HashSet::new();
    // pick a random starting position
    let starting_pos_idx = sample(&mut rand::thread_rng(), world.len(), 1).index(0);
    let mut origin: Cube<i32> = *world.iter().skip(starting_pos_idx).next().unwrap().clone();
    println!("start: {:}", origin);

    // pick a random direction out of the 4 non-parallel directions
    // either add the next direction/2 or add current direction + prev_dir/2
    let mut reverse = true;
    let mut dirs = DIRECTIONS.iter().cycle().skip(6);
    // let mut prev_direction = dirs.next().unwrap().clone();
    let mut next_direction = dirs.next().unwrap().clone();
    let mut current_dir = next_direction;
    println!("______first direction: {:}______", next_direction);
    // add half of the direction
    let mut next_pos = CubeSide::from(Cube::<f32>::from(origin) + (next_direction / 2));
    for _ in 1..307 {
        river.insert(next_pos);
        let advance_a_tile = rand::random::<f32>();
        if advance_a_tile > 0.3 {
            origin += current_dir;
            current_dir = current_dir * -1;
            println!("advancing to {:}", origin);

            // skip to current pos
            let d = if reverse {DIRECTIONS} else {REV_DIRECTIONS};
            let mut idx = d.iter().position(|&c| c == (current_dir)).unwrap();
            // let mut idx = dirs.position(|&c| c == current_dir).unwrap();
            println!("original idx: {:}", idx);

            let geo_mapping = vec![3, 4, 5, 0, 1, 2]; // or mul by -1
            let rev_mapping = vec![5, 4, 3, 2, 1, 0];
            // 5 <-> 2
            // 4 <-> 1
            // 3 <-> 0
    
            // idx = geo_mapping[idx];
            idx = rev_mapping[idx];
            println!("mapped idx: {:}", idx);

            dirs = DIRECTIONS.iter().cycle().skip(idx + 1);
            
            if reverse {
                // 0 <-> 5
                // 1 <-> 4
                // 2 <-> 3

                // let mapping = vec![5, 4, 3, 2, 1, 0];
                // let mapping = vec![3, 4, 5, 0, 1, 2];
                // idx = mapping[idx];
                // println!("mapped idx: {:}", idx);
                dirs = REV_DIRECTIONS.iter().cycle().skip(idx + 1);
            }
            reverse = !reverse;

            next_direction = dirs.next().unwrap().clone();
            println!("curr, next: {:}, {:}", current_dir, next_direction);
            // println!("prev, curr, next: {:}, {:}, {:}", prev_direction, current_dir, next_direction);
            next_pos = CubeSide::from(Cube::<f32>::from(origin) + (next_direction / 2));
            // next_pos = CubeSide::from(Cube::<f32>::from(origin) + ((prev_direction * -1) / 2));
            // next_direction = dirs.next().unwrap().clone();
            // prev_direction = next_direction;
        } else {
            next_direction = dirs.next().unwrap().clone();
            next_pos = CubeSide::from(Cube::<f32>::from(origin) + (next_direction / 2));
        }
        // prev_direction = current_dir;
        current_dir = next_direction;
    }

    // println!("{:}, {:}", -0.0, (-0.0 as i32 as f32).copysign(0.0));
    // f32::total_cmp(&0.3, &(0.1 + 0.2));
    river
}

// take 1st dir
// take either next dir or go to next tile
// if going to next tile, add current dir to current pos to get new pos
// reverse the iterator


        // let adjacent_directions: Vec<&Cube<i32>> = DIRECTIONS.iter().filter(|cube| cube != &&next_direction && cube != &&-next_direction).collect();
        // let next_direction_idx = sample(&mut rand::thread_rng(), 4, 1).index(0);
        // next_direction = **adjacent_directions.index(next_direction_idx);