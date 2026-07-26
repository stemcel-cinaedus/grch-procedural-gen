use rand::{self};
use std::ops::*;


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
    fn split(&mut self) {
            match self.right  {
                Some(_) => (),
                None => {
                    self.right = Some(Box::from(BSPNode{
                        value: Tile { 
                            lc: self.value.lc,
                            rc: if self.split_on_x == true {
                                Point2((self.value.rc.0 as f64 * rand::random_range((1.0 / 5.0)..(4.0 / 5.0))) as i64, self.value.rc.1)
                                } else {
                                Point2(self.value.rc.0, (self.value.rc.1 as f64 * rand::random_range((1.0 / 5.0)..(4.0 / 5.0))) as i64)
                                },
                            traversible: false,
                            split_count: (self.value.split_count + 1),
                            room: None
                        },
                        left:  None,
                        right: None,
                        room:  None,
                        split_on_x: rand::random_bool(1.0 / 2.0),
                }));
                self.left = Some(Box::from(BSPNode{
                        value: Tile { 
                            lc: self.value.lc,
                            rc: self.value.rc - Point2(self.right.as_ref().unwrap().value.get_width(), self.right.as_ref().unwrap().value.get_height()),
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

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
struct Room(Point2, Point2, bool);


fn split_dfs(root: &mut BSPNode<Tile>, mut depth: i64) {
    if root.value.split_count < depth {
        depth -= 1;
        root.split();
        split_dfs(root.right.as_mut().unwrap(), depth);
        split_dfs(root.left.as_mut().unwrap(), depth);
    } else {
        return
    }
}

fn build_dfs(root: BSPNode<Tile>, map: &mut Map) -> () {
    if root.right != None {
        build_dfs(*root.right.unwrap(), map);
        build_dfs(*root.left.unwrap(), map);
        } else {
        map.tiles.push(Tile{lc: root.value.lc, rc: root.value.rc, traversible: true, split_count: root.value.split_count, room: Some(Room(Point2(0, 0), Point2(0, 0), false))})
    }
}


pub fn initbt(size: Point2, divisions: i64) -> () {
    let mut root = BSPNode{ value: Tile{lc: Point2(0, 0), rc: size, traversible: false, split_count: 0, room: None}, right: None, left: None, room: None, split_on_x: rand::random_bool(1.0/2.0)};
    split_dfs(&mut root, divisions);
    let mut map = Map{max_height: 512, max_width: 512, tiles: Vec::<Tile>::new()};
    build_dfs(root, &mut map);
    
    for tile in map.tiles {
        println!("{:#?} {:#?} {:#?}", tile.lc, tile.rc, tile.traversible)
    }
}

fn main() {
    let divisions: i64 = 4;
    let mut root = BSPNode{ value: Tile{lc: Point2(-512, -512), rc: Point2(0, 0), traversible: false, split_count: 0, room: None}, right: None, left: None, room: None, split_on_x: rand::random_bool(1.0/2.0)};
    split_dfs(&mut root, divisions);
    let mut map = Map{max_height: 512, max_width: 512, tiles: Vec::<Tile>::new()};
    build_dfs(root, &mut map);
    
    for tile in &map.tiles {
        println!("Tile specs: Left Corner: {:#?}; Right Corner: {:#?}; Traversible: {:#?}; Split Count: {:#?}; Vector Length: {}", tile.lc, tile.rc, tile.traversible, tile.split_count, map.tiles.len())
    }
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