use std::sync::atomic::{AtomicUsize};
use std::sync::{RwLock};

use crate::PLACE_DIST;
use crate::CORRIDOR_OFFSET;

use crate::types::*;

pub static EDGES: RwLock<Vec<(Point3, Point3)>> = RwLock::new(Vec::<(Point3, Point3)>::new());
pub static INDEX: AtomicUsize = AtomicUsize::new(0);

//Make Megumi holes
fn generate_candidates(corner: Point3, bounding_corner: Point3, axis: Axis) -> Vec<Point3> {
    match axis {
        Axis::X => {
            //Start at lowest Z & lowest Y value, move along the face of the shape to create more every d distance
            let y_range = bounding_corner.1;
            let z_range = bounding_corner.2;

            let y = corner.1;
            let mut z = corner.2;

            let mut candidates = Vec::<Point3>::new();

            while y.abs() < (y_range.abs()) && z.abs() < (z_range.abs()) {
                //TODO: I want to change this to check if it's within bounds first
                candidates.push(Point3(corner.0, y + CORRIDOR_OFFSET.1 , z + CORRIDOR_OFFSET.2 ));
                //Currently, I am only placing candidates at the lowest Y level to get a working version first. Later, I will add variable Y as well.
                z += PLACE_DIST;
            }
            return candidates;

        }, //Add steepness check later
        Axis::Y => {return Vec::<Point3>::new()}, //Add vertical corridors later
        Axis::Z => {
            let x_range = bounding_corner.0;
            let y_range = bounding_corner.1;
            
            let mut x = corner.0;
            let y = corner.1;
            
            let mut candidates = Vec::<Point3>::new();

            while y.abs() < y_range.abs() && x.abs() < x_range.abs() {
                //TODO: I want to change this to check if it's within bounds first
                candidates.push(Point3(x + CORRIDOR_OFFSET.0, y + CORRIDOR_OFFSET.1, corner.2));
                //Currently, I am only placing candidates at the lowest Y level to get a working version first. Later, I will add variable Y as well.
                x += PLACE_DIST;
            }
            return candidates;
        }
    }
}

fn generate_edges(rooms: (&[Room], &[Room]), axis: Axis, split_pos: Point3) -> () {
    //Refactor to make it find the rooms with the room positions closest to the split , 
    //Lazy to implement rn, but basically it would be like match the plane & split point, and then grab the top n rooms on the left that are closest to the split,
    //grab the top n rooms on the right that are closest to the split, and run generate candidates on them for a total of n*n calculations, saving a lot more resources than just 
    //checking every possible vertice.

    fn get_closest(rooms: &[Room], split_pos: Point3, axis: Axis, n: usize) -> impl Iterator<Item = &Room> {
        
        //Probably will change "closest_room" to be a tuple or maybe a Vec of length n so that I can grab the 1st, 2nd, etc closest
        match axis {
            Axis::X => {
                let mut closest_rooms: Vec<&Room> = rooms.iter().collect();
                closest_rooms.sort_by_key(|room| {
                    return (split_pos.0 - room.1.0).abs().min((split_pos.0 - room.2.0).abs());
                });
                return closest_rooms.into_iter().take(n);                
            }
            Axis::Y => {
                let mut closest_rooms: Vec<&Room> = rooms.iter().collect();
                closest_rooms.sort_by_key(|room| {
                    return (split_pos.1 - room.1.1).abs().min((split_pos.1 - room.2.1).abs());
                });
                return closest_rooms.into_iter().take(n);                
            }
            Axis::Z => {
                let mut closest_rooms: Vec<&Room> = rooms.iter().collect();
                closest_rooms.sort_by_key(|room| {
                    return (split_pos.2 - room.1.2).abs().min((split_pos.2 - room.2.2).abs());
                });
                return closest_rooms.into_iter().take(n);                
            }
        }
    }



    //Now the idea is to take the n closest and draw corridors between them
    //1 for testing currently
    let left_candidates = get_closest(rooms.0, split_pos, axis, 1).map(|room| generate_candidates(room.1, room.2, axis));
    let right_candidates = get_closest(rooms.1, split_pos, axis, 1).map(|room| generate_candidates(room.2, room.1, axis));
   
    for (c1, c2) in left_candidates.zip(right_candidates) {
        let mut shortest =  i64::MAX;
        let mut shortest_points = (Point3(i64::MAX,i64::MAX,i64::MAX), Point3(i64::MAX,i64::MAX,i64::MAX));

        for e1 in &c1 {
            for e2 in &c2 {
                let dist = (e2.0 - e1.0).pow(2) + (e2.1 - e1.1).pow(2) + (e2.2 - e1.2).pow(2);
                if dist < shortest {
                    shortest = dist;
                    shortest_points = (*e1, *e2); 
                }
            }
        }
        EDGES.write().unwrap().push(shortest_points);
    }

    //TODO: Implement distance algorithm for each candidate array, ideally in O(k log_k), somehow a hard task
}

/*

*/

pub fn edge_dfs(root: &BSPNode<Tile>, divisions: i64) -> Vec<Room> {

    //TRASH DOESNT WORK, NEED TO CHANGE GENERATE EDGES TO TAKE VARIADIC INPUT WITH VECS 
    let mut left_rooms = Vec::new();
    let mut right_rooms = Vec::new();

    if divisions - root.value.split_count >= 1 {
        left_rooms = edge_dfs(&(root.left.as_deref().unwrap()), divisions);
        right_rooms = edge_dfs(&(root.right.as_deref().unwrap()), divisions);
        generate_edges((&left_rooms, &right_rooms), root.split_d, (root.left.as_deref().unwrap()).value.rc);  
    }  else {
        if root.value.room.is_some() {
            left_rooms.push(root.value.room.unwrap());
        }
    }
    return left_rooms.into_iter().chain(right_rooms).collect();
}








    /*
    let mut rooms_in_grandparent = Arc::new(RwLock::new(Vec::<Room>::new()));

    if root.value.split_count > 2 {
        get_groups(&(root.right.as_deref().unwrap()), divisions);
        get_groups(&(root.left.as_deref().unwrap()), divisions);
    }

    fn get_leaf_rooms(root: &BSPNode<Tile>) -> (Option<Room>, Option<Room>) {
        match ((root.right.as_deref().unwrap().value.room), (root.left.as_deref().unwrap().value.room)) {
            (Some(r1), Some(r2)) => return (Some(r1), Some(r2)),
            (Some(r1), None) => return (Some(r1), None),
            (None, Some(r2)) => return (None, Some(r2)),
            _ => return (None, None)
        }
    }
    //TODO: Use better connection method. Using the first Room found in the grandparent is retarded.
    fn match_get_leaf(root: &BSPNode<Tile>, rooms_in_grandparent: &mut Arc<RwLock<Vec<Room>>>) {
            match get_leaf_rooms(root) {
            (Some(r1), Some(r2)) => {
                let e = generate_edges((r1, r2));
                EDGES.write().unwrap().push(e.0);
                EDGES.write().unwrap().push(e.1);
                rooms_in_grandparent.write().unwrap().push(r1);
                rooms_in_grandparent.write().unwrap().push(r2);},
            (Some(r1), None) => {
                if !rooms_in_grandparent.read().unwrap().is_empty() {
                    let e = generate_edges((r1, rooms_in_grandparent.read().unwrap()[0]));
                    EDGES.write().unwrap().push(e.0);
                    EDGES.write().unwrap().push(e.1);
                }
                rooms_in_grandparent.write().unwrap().push(r1);
            },
            (None, Some(r2)) => {
                if !rooms_in_grandparent.read().unwrap().is_empty() {
                    let e = generate_edges((r2, rooms_in_grandparent.read().unwrap()[0]));
                    EDGES.write().unwrap().push(e.0);
                    EDGES.write().unwrap().push(e.1);
                }
                rooms_in_grandparent.write().unwrap().push(r2);
            },
            (None, None) => () 
        }
    }

    match_get_leaf(&(root.left.as_deref().unwrap()), &mut rooms_in_grandparent);
    match_get_leaf(&(root.right.as_deref().unwrap()), &mut rooms_in_grandparent);

    //Remaining until working demo: Connect the two parent nodes, connect grandparent nodes
    */


/*
pub fn union_find(rooms: Vec::<Room>) -> i32 {
    let mut vert_map = std::collections::HashMap::new();

    rooms.into_iter().scan(-1, |i, room| {
        *i += 1;
        Some((i.clone(), room))
    }).for_each(|(a, b)| {
         vert_map.insert(a, b);
        });

    let mut parents: Vec<usize> = vec![0; vert_map.len()];
    let mut rank: Vec<i32> = vec![0; vert_map.len()];

    fn find(node1: (usize, Room), parents: &mut Vec<usize>) -> usize {
        let mut res = node1.0;
        
        while res != parents[res as usize] {
            parents[res as usize] = parents[parents[res as usize] as usize];
            res = parents[res as usize];
           // vert_map[res.0] = vert_map[vert_map.contains_key(res.0)]
        }
        return res
    }

    fn union(c1: (usize, Room), c2: (usize, Room), mut parents: Vec<usize>, mut rank: Vec<i32>) -> usize{
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
    let mut result: usize = vert_map.len();
    for (i, n1, n2) in EDGES.read().unwrap().iter() {
        result -= union(n1, n2, parents, rank);
    } 
    return result
    */
    return 0
}
*/

