use rand::{self};
use std::ops::*;
//Used for plotting the tiles:
use serde_json::json;



#[derive(Debug)]
#[derive(PartialEq)]
struct BSPNode<T> {
    value: T,
    left: Option<Box<BSPNode<T>>>,
    right: Option<Box<BSPNode<T>>>,
    room: Option<Room>,
    split_on_x: bool
}


impl BSPNode<Tile> {
    fn random_split_helper_right(hw: Point2, tile_height: i64, tile_width: i64, s: bool) -> Point2 {
        if (hw.1 as f64) / (hw.0 as f64) <= 0.2 {
            return Point2((hw.0 - (tile_width as f64 * rand::random_range((2.0 / 5.0)..(3.0 / 5.0))) as i64), hw.1)
        } else if (hw.1 as f64) / (hw.0 as f64) >= 5.0 {
            return Point2(hw.0, (hw.1 - (tile_height as f64 * rand::random_range((2.0 / 5.0)..(3.0 / 5.0))) as i64))
        } else if s == true {
            return Point2((hw.0 - (tile_width as f64 * rand::random_range((2.0 / 5.0)..(3.0 / 5.0))) as i64), hw.1)
        } else {
            return Point2(hw.0, (hw.1 - (tile_height as f64 * rand::random_range((2.0 / 5.0)..(3.0 / 5.0))) as i64))
        }
    }


    fn split(&mut self) {
            match self.left  {
                Some(_) => (),
                None => {
                    self.left = Some(Box::from(BSPNode{
                        value: Tile { 
                            lc: self.value.lc,
                            rc: BSPNode::random_split_helper_right(self.value.rc, self.value.get_height(), self.value.get_width(), self.split_on_x ),
                            traversible: false,
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        room:  None,
                        split_on_x: rand::random_bool(1.0 / 2.0),
                }));
                self.right = Some(Box::from(BSPNode{
                        value: Tile { 
                            //Add conditional to make the tiles squares instead of line segments
                            
                            lc: if self.split_on_x == true {
                                Point2(self.left.as_ref().unwrap().value.rc.0, self.left.as_ref().unwrap().value.lc.1) 
                            } else {
                                Point2(self.left.as_ref().unwrap().value.lc.0, self.left.as_ref().unwrap().value.rc.1) 
                            },
                                
                            rc: self.value.rc,
                            traversible: false,
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        room:  None,
                        split_on_x: rand::random_bool(1.0 / 2.0)}));
            
                }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]

struct Tile {
    lc: Point2,
    rc: Point2,
    traversible: bool,
    split_count: i64,
    room: Option<Room>
}

impl Tile {
    fn get_height(&self) -> i64 {
        return self.rc.1 - self.lc.1
    }
    fn get_width(&self) -> i64 {
        return self.rc.0 - self.lc.0
    }
    fn dist_to() {}
}

struct Map {
    max_width : i64,
    max_height: i64,
    tiles: Vec<Tile>,
}

impl Map {
    fn set_tile(lc: Point2, rc: Point2) {
        ()
    }
    fn get_tile(lc: Point2, rc: Point2) {
        ()
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]

pub struct Point2(i64, i64);

impl Sub<Point2> for Point2 {
    type Output = Point2;

    fn sub(self, rhs: Point2) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add<Point2> for Point2 {
    type Output = Point2;

    fn add(self, rhs: Point2) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
struct Room(Point2, Point2, bool);


fn split_dfs(root: &mut BSPNode<Tile>, depth: i64) {
    if root.value.split_count < depth {
        root.split();
        split_dfs(root.right.as_mut().unwrap(), depth);
        split_dfs(root.left.as_mut().unwrap(), depth);
    } else {
        return
    }
}

fn construct_room(tile: Tile) -> Room {
    let dist_from_lc: i64 = rand::random_range(1..3);
    let dist_from_rc: i64 = rand::random_range(1..3);
    return Room(
        Point2(tile.rc.0 - dist_from_lc, tile.rc.1 - dist_from_lc),
        Point2(tile.rc.0 - dist_from_rc, tile.rc.1 - dist_from_rc),
        true )
}

fn build_dfs(root: BSPNode<Tile>, map: &mut Map) -> () {
    if root.right != None {
        build_dfs(*root.right.unwrap(), map);
        build_dfs(*root.left.unwrap(), map);
        } else {
        map.tiles.push(Tile{
            lc: root.value.lc,
            rc: root.value.rc,
            traversible: true,
            split_count: root.value.split_count,
            room: Some(construct_room(Tile {
                lc: (root.value.lc),
                rc: (root.value.rc),
                traversible: true,
                split_count: root.value.split_count,
                room: None
            }))
        })
    }
}


pub fn initbt(size: Point2, divisions: i64) -> () {
    let mut root = BSPNode{
        value: Tile{lc: Point2(0, 0), rc: size, traversible: false, split_count: 0, room: None},
        right: None,
        left: None,
        room: None,
        split_on_x: rand::random_bool(1.0/2.0)
    };
    split_dfs(&mut root, divisions);
    let mut map = Map{max_height: 512, max_width: 512, tiles: Vec::<Tile>::new()};
    build_dfs(root, &mut map);
    
    for tile in map.tiles {
        println!("{:#?} {:#?} {:#?}", tile.lc, tile.rc, tile.traversible)
    }
}

fn main() {
    let divisions: i64 = 4;
    let mut root = BSPNode{ value: Tile{lc: Point2(0,0), rc: Point2(512, 512), traversible: false, split_count: 0, room: None}, right: None, left: None, room: None, split_on_x: rand::random_bool(1.0/2.0)};
    split_dfs(&mut root, divisions);
    let mut map = Map{max_height: 512, max_width: 512, tiles: Vec::<Tile>::new()};
    build_dfs(root, &mut map);
    
    let tile_data = json!({
        "Tile": &map.tiles.iter().map(|t| {
            json!({"Left Corner": (t.lc.0, t.lc.1),
                    "Right Corner": (t.rc.0, t.rc.1),
                    "Traversible": t.traversible,
                    "Split Count": t.split_count
                    })
            }).collect::<Vec<_>>()
        });

    let tile_json = serde_json::to_string_pretty(&tile_data).unwrap();

    print!("{}", tile_json);
}



/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/