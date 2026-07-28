use std::collections::HashMap;
use rand::{self, RngExt, random_bool};
use rand::rng;
use rand::rngs::StdRng;
use rand::SeedableRng;
//Used for plotting the tiles:
use serde_json::json;

pub mod types;
use crate::types::*;

pub const SEED: u64 = 184138956713986453;





fn split_dfs(root: &mut BSPNode<Tile>, depth: i64) {
    if root.value.split_count < depth {
        root.split();
        split_dfs(root.right.as_mut().unwrap(), depth);
        split_dfs(root.left.as_mut().unwrap(), depth);
    } else {
        return
    }
}

fn construct_room(tile: Tile, abs_dist_x: i64, abs_dist_y: i64, abs_dist_z: i64, room_scale_factor: f64, rng: &mut StdRng) -> Option<Room> {
    if tile.traversible == false {
        return None
    }

    //ABSOLUTE MESS rn, I need to add the random % movement as a function parameter as well. Works, but makes me feel illiterate
    let dist_from_x: i64 = (tile.get_width() as f64 * rng.random_range(0.0..0.10)) as i64;
    let dist_from_y: i64 = (tile.get_height() as f64 * rng.random_range(0.0..0.10)) as i64;
    let dist_from_z: i64 = (tile.get_depth() as f64 * rng.random_range(0.0..0.10)) as i64;

    return Some(Room(
        Point3(
            ((tile.lc.0 as f64 + tile.get_width() as f64 * room_scale_factor) as i64 + (abs_dist_x / 2)) + dist_from_x,
            ((tile.lc.1 as f64 + tile.get_height() as f64 * room_scale_factor)) as i64 + (abs_dist_y / 2) + dist_from_y,
            ((tile.lc.2 as f64 + tile.get_depth() as f64 * room_scale_factor)) as i64 + (abs_dist_z / 2) + dist_from_z
        ),
        Point3(
            ((tile.rc.0 as f64 - tile.get_width() as f64 * room_scale_factor) as i64 - (abs_dist_x / 2)) - dist_from_x,
            ((tile.rc.1 as f64 - tile.get_height() as f64 * room_scale_factor)) as i64 - (abs_dist_y / 2) - dist_from_y,
            ((tile.rc.2 as f64 - tile.get_depth() as f64 * room_scale_factor)) as i64 - (abs_dist_z / 2) - dist_from_z
        ),
        true ))
}

fn build_dfs(root: &BSPNode<Tile>, map: &mut Map, rng: &mut StdRng) -> () {
    
    if root.right != None {
        build_dfs(root.right.as_deref().unwrap(), map, rng);
        build_dfs(root.left.as_deref().unwrap(), map, rng);
        } else {
        map.tiles.push(Tile{
            lc: root.value.lc,
            rc: root.value.rc,
            traversible: true,
            split_count: root.value.split_count,
            room: construct_room(Tile {
                lc: (root.value.lc),
                rc: (root.value.rc),
                traversible: rng.random_bool(2.0 / 3.0),
                split_count: root.value.split_count,
                room: None
            }, 0, 0, 0, 0.05, rng)
        })
    }
}


pub fn initbt(size: Point3, divisions: i64) -> () {
    let mut rng = StdRng::seed_from_u64(SEED);

    let mut root = BSPNode{
        value: Tile{lc: Point3(0, 0, 0), rc: size, traversible: false, split_count: 0, room: None},
        right: None,
        left: None,
        room: None,
        split_d: SplitAxis::random_variant()
    };
    split_dfs(&mut root, divisions);
    let mut map = Map{tiles: Vec::<Tile>::new()};
    build_dfs(&root, &mut map, &mut rng);
    
    for tile in map.tiles {
        println!("{:#?} {:#?} {:#?}", tile.lc, tile.rc, tile.traversible)
    }
}

fn build_planes(tree: &BSPNode<Tile>) -> Vec<_> {
    let planes = Vec::<_>::new();

    while tree.right.is_some() {
        build_planes(&(tree.right.as_deref().unwrap()));
        build_planes(&(tree.right.as_deref().unwrap()));
    }

    let (lc, rc) = (tree.value.room.unwrap().0, tree.value.room.unwrap().1);
    //Point P = x_0, y_0, z_0; Normal vector N = <a,b,c>, gen eqn: a(x - x_0) + b(y - y_0) + c(z - z_0) = 0
    //let left_face_plane = -1(x - lc.0) + 0(y - y0) + 0(z - z0) = 0

    return planes
}

fn get_groups(root: &BSPNode<Tile>, divisions: i64) {
    while root.value.split_count > 2 {
        get_groups(&(root.right.as_deref().unwrap()), divisions);
        get_groups(&(root.left.as_deref().unwrap()), divisions);
    }

    

}


fn union_find(rooms: Vec::<Room>) -> i32 {
    let mut vert_map = std::collections::HashMap::new();

    rooms.into_iter().scan(-1, |i, room| {
        *i += 1;
        Some((i.clone(), room))
    }).map(|(a, b)| vert_map.insert(a, b));

    let mut parents: Vec<i32> = vec![0; vert_map.len()];
    let mut rank: Vec<i32> = vec![0; vert_map.len()];

    fn find(node1: (i32, Room), parents: &mut Vec<i32>) -> i32 {
        let mut res = node1.0;
        
        while res != parents[res as usize] as i32 {
            parents[res as usize] = parents[parents[res as usize] as usize];
            res = parents[res as usize];
           // vert_map[res.0] = vert_map[vert_map.contains_key(res.0)]
        }
        return res
    }

    fn union(c1: (i32, Room), c2: (i32, Room), mut parents: Vec<i32>, mut rank: Vec<i32>) -> i32{
        let p1 = find(c1, &mut parents);
        let p2 = find(c2, &mut parents);

        if p1 == p2 {
            return 0
        }

        if rank[p2 as usize] > rank[p1 as usize] {
            parents[p1 as usize] = p2;
            rank[p2 as usize] += rank[p1 as usize];
        } else {
            parents[p2 as usize] = p1;
            rank[p1 as usize] += rank[p2 as usize];
        } return 1
    }

    /*
    let mut result: i32 = vert_map.len() as i32;
    for (n1, n2) in edges {
        result -= union(n1, n2, parents, rank);
    } 
    return result
    */
    return 0
}





fn main() {
    let mut rng = StdRng::seed_from_u64(SEED);

    let divisions: i64 = 6;
    let mut root = BSPNode{ value: Tile{lc: Point3(0,0,0), rc: Point3(2048, 2048, 2048), traversible: false, split_count: 0, room: None}, right: None, left: None, room: None, split_d: SplitAxis::random_variant()};
    split_dfs(&mut root, divisions);
    let mut map = Map{tiles: Vec::<Tile>::new()};
    build_dfs(&root, &mut map, &mut rng);
    
    let tile_json = json!({
    "Tiles": &map.tiles.iter().map(|tile| {

        let mut tile_json = json!({
            "Left Corner": (tile.lc.0, tile.lc.1, tile.lc.2 ),
            "Right Corner": (tile.rc.0, tile.rc.1, tile.rc.2 ),
            "traversible": tile.traversible,
            "split_count": tile.split_count,
            });

        if let Some(room) = tile.room {
            tile_json["Room"] = json!({
                    "Left Corner": (room.0.0, room.0.1, room.0.2 ),
                    "Right Corner": (room.1.0, room.1.1, room.1.2 )
                })
            };
            tile_json
        }).collect::<Vec<_>>()
    });

// Convert the JSON object to a pretty-printed String
let tile_json = serde_json::to_string_pretty(&tile_json).unwrap();

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