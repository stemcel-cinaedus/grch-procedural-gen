use std::ops::*;
use rand::{RngExt, random_range};
use rand::rngs::StdRng;
use rand::rng;
use rand::SeedableRng;
use crate::SEED;


#[derive(Debug)]
#[derive(PartialEq)]
pub struct BSPNode<T> {
    pub value: T,
    pub left: Option<Box<BSPNode<T>>>,
    pub right: Option<Box<BSPNode<T>>>,
    pub room: Option<Room>,
    pub split_d: SplitAxis
}





impl BSPNode<Tile> {
    //Refactor WIP: Point2 -> Point3, creating new splitting algorithm


    pub fn split(&mut self) {
            let rng_factor = rand::random_range(0.3..0.7);

            let next_split = match (self.split_d) {
                            SplitAxis::X => {
                                if self.value.get_depth() > self.value.get_height() {
                                    SplitAxis::Z
                                } else {
                                    SplitAxis::Y
                                }}
                            SplitAxis::Y => {
                                if self.value.get_depth() > self.value.get_width() {
                                    SplitAxis::Z
                                } else {
                                    SplitAxis::X
                                }}
                            SplitAxis::Z => {
                                if self.value.get_width() > self.value.get_height() {
                                    SplitAxis::X
                                } else {
                                    SplitAxis::Y
                                }
                            }
                        };
            
            match self.left  {
                Some(_) => (),
                None => {
                    self.left = Some(Box::from(BSPNode{
                        value: Tile {
                            lc: self.value.lc,
                            rc: { 
                                if self.split_d == SplitAxis::Z {
                                    Point3(self.value.rc.0, self.value.rc.1, self.value.rc.2 - (self.value.get_depth() as f64 * rng_factor) as i64)
                                } else if self.split_d == SplitAxis::Y {
                                    Point3(self.value.rc.0, self.value.rc.1 - (self.value.get_height() as f64 * rng_factor) as i64, self.value.rc.2 )
                                } else {
                                    Point3(self.value.rc.0 - (self.value.get_width() as f64 * rng_factor) as i64, self.value.rc.1, self.value.rc.2)
                                }
                            },
                            traversible: false,
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        room:  None,
                        split_d: next_split.clone()
                    }));
                    
                self.right = Some(Box::from(BSPNode{
                        value: Tile { 
                            //Add conditional to make the tiles squares instead of line segments
                            
                            lc: {
                                if self.split_d == SplitAxis::Z {
                                    Point3(self.value.lc.0, self.value.lc.1, self.left.as_ref().unwrap().value.rc.2)
                                } else if self.split_d == SplitAxis::Y {
                                    Point3(self.value.lc.0, self.left.as_ref().unwrap().value.rc.1, self.value.lc.2)
                                } else {
                                    Point3(self.left.as_ref().unwrap().value.rc.0, self.value.lc.1, self.value.lc.2)
                                }
                            },
                            rc: self.value.rc,
                            traversible: false,
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        room:  None,
                        split_d: next_split
                }));
            }
        }
    }
}



#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum SplitAxis {
    X,
    Y,
    Z
}

impl SplitAxis {
    pub fn random_variant() -> SplitAxis {
        let mut rng = StdRng::seed_from_u64(SEED);
        return match rng.random_range(1..4) {
            1..2 => SplitAxis::X,
            2..3 => SplitAxis::Y,
            3..4 => SplitAxis::Z,
            _ => SplitAxis::X
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]

pub struct Tile {
    pub lc: Point3,
    pub rc: Point3,
    pub traversible: bool,
    pub split_count: i64,
    pub room: Option<Room>
}

impl Tile {
    pub fn get_depth(&self) -> i64 {
        return self.rc.2 - self.lc.2
    }
    pub fn get_height(&self) -> i64 {
        return self.rc.1 - self.lc.1
    }
    pub fn get_width(&self) -> i64 {
        return self.rc.0 - self.lc.0
    }
    fn dist_to() {}
}

pub struct Map {
    pub tiles: Vec<Tile>,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]

pub struct Point3(pub i64, pub i64, pub i64);

impl Sub<Point3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Add<Point3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Room(pub Point3, pub Point3, pub bool);