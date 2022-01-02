extern crate num;
use num::Signed;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

use std::f64::consts::PI;

// use std::collections::HashSet;

#[derive(Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Cube<T>(T,T);

impl<T> Cube<T> where T: Copy + Signed {
    fn q(&self) -> T {
        self.0
    }

    fn r(&self) -> T {
        self.1
    }

    fn s(&self) -> T {
        -self.0 - self.1
    }
}

impl<T: Signed + Copy + Debug> Debug for Cube<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("Cube")
         .field(&self.0)
         .field(&self.1)
         .field(&self.s())
         .finish()
    }
}

// Allow casting Cube(1,2) to Cube(1.0, 2.0)
impl From<Cube<i32>> for Cube<f64> {
    fn from(cube_in: Cube<i32>) -> Self {
        Cube(f64::from(cube_in.0), f64::from(cube_in.1))
    }
}

// This doesn't work as I cannot constrain T != U and thus it overrides the default implementation
// impl<T, U: From<T>> From<Cube<U>> for Cube<T> {
//     fn from(arg: U) -> Self {
//         T::From(arg)
//     }
// }


impl<T, U> Add<Cube<U>> for Cube<T> where T: Add<U> {
    type Output = Cube<<T as Add<U>>::Output>;

    fn add(self, rhs: Cube<U>) -> Self::Output {
        Cube(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T, U> Sub<Cube<U>> for Cube<T> where T: Sub<U> {
    type Output = Cube<<T as Sub<U>>::Output>;

    fn sub(self, rhs: Cube<U>) -> Self::Output {
        Cube(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T, U: Copy> Mul<U> for Cube<T> where T: Mul<U> {
    type Output = Cube<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Cube(self.0 * rhs, self.1 * rhs)
    }
}

impl Cube<f64> {
    fn round(&self) -> Cube<i32> {
        let mut qi = self.q().round();
        let mut ri = self.r().round();
        let si = self.s().round();
        let q_diff = qi - self.q();
        let r_diff = ri - self.r();
        let s_diff = si - self.s();
        if q_diff > r_diff && q_diff > s_diff {
            qi = -ri - si;
        } else if r_diff > s_diff {
            ri = -qi - si;
        }
        Cube(qi as i32, ri as i32)
    }
    fn lerp(&self, &other: &Cube<f64>, t: f64) -> Cube<f64> {
        Cube(
            self.q() * (1.0 - t) + other.q() * t,
            self.r() * (1.0 - t) + other.r() * t,
        )
    }
    fn to_pixel(&self, &layout: &Layout) -> [f64; 2] {
        let matrix = layout.orientation;
        let size = layout.size;
        let origin = layout.origin;
        let x = (matrix.f0 * self.q() + matrix.f1 * self.r()) * size[0];
        let y = (matrix.f2 * self.q() + matrix.f3 * self.r()) * size[1];
        [x + origin[0], y + origin[1]]
    }
    fn corner_offset(&layout: &Layout, corner: u8) -> [f64; 2] {
        let matrix = layout.orientation;
        let size = layout.size;
        let angle = 2.0 * PI * (matrix.start_angle - corner as f64) / 6.0;
        [size[0] * angle.cos(), size[1] * angle.sin()]
    }
    fn corners(&self, &layout: &Layout) -> [[f64; 2]; 6] {
        let mut corners = [[0.0, 0.0]; 6];
        let center = self.to_pixel(&layout);
        for i in 0..6 {
            let offset = Cube::<f64>::corner_offset(&layout, i);
            corners[i as usize] = [center[0] + offset[0], center[1] + offset[1]];
        }
        corners
    }

}

impl Cube<i32> {
    // https://gamedev.stackexchange.com/a/51267
    fn ring(&self, n: usize) -> Vec<Cube<i32>> {
        let mut result = vec![Cube(0,0); 6 * n];
        let distance_to_start = DIRECTIONS[0] * n as i32;
        // let distance_to_start2 = Cube::<i32>::from(distance_to_start);
        let start = *self + distance_to_start;
        let mut counter = 0;
        for direction in DIRECTIONS {
            for i in 0..n {
                result[counter] = start + Cube::<i32>::from(direction);
                counter += 1;
            };
        };
        result
    }
    fn disc(&self, n: usize) -> Vec<Cube<i32>> {
        let mut triangular_number = n;
        for i in 1..n {
            triangular_number += (n - i);
        }
        let size = triangular_number * 6;

        let mut result = vec![Cube(0,0); size];
        let mut result_index = 0;
        for ring_number in 1..n+1 {
            let ring = self.ring(ring_number);
            for item in ring {
                result[result_index] = item;
                result_index += 1;
            }
        }
        result
    }
}

impl<T> Cube<T> where T: Copy + Signed + Div<i32, Output=T> { // , Cube<T>: From<Cube<i32>>  From<Cube<i32>> + Copy + Signed...
    fn length(&self) -> T {
        (self.q().abs() + self.r().abs() + self.s().abs()) / 2
    }
    fn distance(&self, &rhs: &Cube<T>) -> T {
        let delta_cube = *self - rhs;
        (delta_cube.q().abs() + delta_cube.r().abs() + delta_cube.s().abs()) / 2
    }
}

// pretend it's a 2x3 matrix
const DIRECTIONS: [Cube<i32>; 6] = [
    Cube(1, -1), Cube(1, 0), Cube(0, 1),
    Cube(-1, 1), Cube(-1, 0), Cube(0, -1),
];

// const DIRECTIONS: [Point<i32>; 6] = [
//     Point{q: 1, r: -1}, Point{q: 1, r: 0}, Point{q: 0, r: 1},
//     Point{q: -1, r: 1}, Point{q: -1, r: 0}, Point{q: 0, r: -1},
// ];

pub trait Summary {
    fn summarize(&self) -> String;
}

// impl<T> Point<T> where T: Copy + Signed + Div<i32, Output=T> {
    // fn get_neighbour(&self, direction: usize) -> Point<T> {
    //     *self + DIRECTIONS[direction]
    // }

    // fn get_nearest_neighbours(&self) -> [Point<T>; 6] {
    //     DIRECTIONS.map(|other| *self + other)
    // }

    // fn get_n_nearest_neighbours(&self, n: u32) -> Vec<Point<T>> {
    //     let mut nnn: HashSet<Point<T>> = HashSet::from_iter([*self]);
    //     for _ in 0..n {
    //         for neighbour in nnn.clone() {
    //             let nn = neighbour.get_nearest_neighbours();
    //             nnn.extend(nn);
    //         };
    //     };
    //     nnn.remove(self);
    //     Vec::from_iter(nnn)
    // }
// }

fn main() {
    let c = Cube(1,1);
    let c2 = Cube(1,-1);
    let c3 = c - c2;
    let cf = Cube(0.4, 0.2);
    let cffromi = Cube(1,1);

    // println!("{:?}", cf.round());

    test_cube_round();

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

// #[derive(Default)]
// struct Board<T: Default, const N: usize> {
//     elems: [T; N]
// }

#[derive(Clone, Copy)]
struct Orientation {
    f0: f64,
    f1: f64,
    f2: f64,
    f3: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
    start_angle: f64,
}

#[derive(Clone, Copy)]
struct Layout {
    orientation: Orientation,
    size: [f64; 2],
    origin: [f64; 2]
}

// no const sqrt() yet...
const SQRT3: f64 = 1.732050807568877293527446341505872366942805253810380628055806; // sqrt(3)

const POINTY: Orientation = Orientation {
    f0: SQRT3, 
    f1: SQRT3 / 2.0, 
    f2: 0.0, 
    f3: 3.0 / 2.0,
    b0: SQRT3 / 3.0, 
    b1: -1.0 / 3.0, 
    b2: 0.0, 
    b3: 2.0 / 3.0, 
    start_angle: 0.5
};

const FLAT: Orientation = Orientation {
    f0: 3.0 / 2.0,
    f1: 0.0,
    f2: SQRT3 / 2.0,
    f3: SQRT3,
    b0: 2.0 / 3.0,
    b1: 0.0,
    b2: -1.0 / 3.0,
    b3: SQRT3 / 3.0,
    start_angle: 0.0,
};

fn pixel_to_cube(&layout: &Layout, pixel: [f64; 2]) -> Cube<f64> {
    let matrix = layout.orientation;
    let size = layout.size;
    let origin = layout.origin;
    let pt = [(pixel[0] - origin[0]) / size[0], (pixel[1] - origin[1]) / size[1]];
    let q = matrix.b0 * pt[0] + matrix.b1 * pt[1];
    let r = matrix.b2 * pt[0] + matrix.b3 * pt[1];
    Cube(q, r)
}

fn equal_cube(name: &str, a:Cube<i32>, b:Cube<i32>){
    // assert!(a.q() == b.q() && a.s() == b.s() && a.r() == b.r());
    println!{"{}: {}, a: {:?} b: {:?}", name, a == b, a, b};
    //complain(name)
}

fn test_cube_round() {
    let a = Cube(0.0, 0.0);
    let b = Cube(1.0, -1.0);
    let c = Cube(0.0, -1.0);
    equal_cube("cube_round 1", Cube(5, -10), (Cube(0.0, 0.0).lerp(&Cube(10.0, -20.0), 0.5)).round());
    equal_cube("cube_round 2", a.round(), (a.lerp(&b, 0.499).round()));
    equal_cube("cube_round 3", b.round(), (a.lerp(&b, 0.501).round()));
    let right1 = Cube(a.q() * 0.4 + b.q() * 0.3 + c.q() * 0.3, a.r() * 0.4 + b.r() * 0.3 + c.r() * 0.3); //, a.s() * 0.4 + b.s() * 0.3 + c.s() * 0.3)).round());
    equal_cube("cube_round 4", a.round(), right1.round());
    let right2 = Cube(a.q() * 0.3 + b.q() * 0.3 + c.q() * 0.4, a.r() * 0.3 + b.r() * 0.3 + c.r() * 0.4); //, a.s() * 0.3 + b.s() * 0.3 + c.s() * 0.4)).round());
    equal_cube("cube_round 5", c.round(), right2.round());
}