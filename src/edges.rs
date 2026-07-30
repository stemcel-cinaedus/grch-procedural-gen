use std::sync::atomic::{AtomicUsize};
use std::sync::{Arc, RwLock};

use crate::PLACE_DIST;
use crate::CORRIDOR_OFFSET;

use crate::types::*;

pub static EDGES: RwLock<Vec<(usize, Point3, Point3)>> = RwLock::new(Vec::<(usize, Point3, Point3)>::new());
pub static INDEX: AtomicUsize = AtomicUsize::new(0);

//Make Megumi holes
fn generate_candidates(corner: Point3, bounding_corner: Point3, axis: Axis) -> Vec<Point3> {
    match axis {
        Axis::X => {
            //Start at lowest Z & lowest Y value, move along the face of the shape to create more every d distance
            let y_range = bounding_corner.1 - corner.1;
            let z_range = bounding_corner.2 - corner.2;

            let y = corner.1;
            let mut z = corner.2;

            let mut candidates = Vec::<Point3>::new();

            while y.abs() < (y.abs() + y_range.abs()) && z.abs() < (z.abs() + z_range.abs()) {
                //TODO: I want to change this to check if it's within bounds first
                candidates.push(Point3(corner.0, y + CORRIDOR_OFFSET.1 , z + CORRIDOR_OFFSET.2 ));
                //Currently, I am only placing candidates at the lowest Y level to get a working version first. Later, I will add variable Y as well.
                z += PLACE_DIST;
            }
            return candidates;

        }, //Add steepness check later
        Axis::Y => {return Vec::<Point3>::new()}, //Add vertical corridors later
        Axis::Z => {
            let x_range = bounding_corner.0 - corner.0;
            let y_range = bounding_corner.1 - corner.1;
            
            let mut x = corner.0;
            let y = corner.1;
            
            let mut candidates = Vec::<Point3>::new();

            while y.abs() < (y.abs() + y_range.abs()) && x.abs() < (x.abs() + x_range.abs()) {
                //TODO: I want to change this to check if it's within bounds first
                candidates.push(Point3(x + CORRIDOR_OFFSET.0, y + CORRIDOR_OFFSET.1, corner.2));
                //Currently, I am only placing candidates at the lowest Y level to get a working version first. Later, I will add variable Y as well.
                x += PLACE_DIST;
            }
            return candidates;
        }
    }
}

fn generate_edges(rooms: (Room, Room)) -> () {
    let (lc1, rc1) = (rooms.0.1, rooms.0.2);
    let (lc2, rc2) = (rooms.1.1, rooms.1.2);
    let indices = (rooms.0.0, rooms.1.0);

    let mut candidates1  = Vec::<_>::new();
    let mut candidates2  = Vec::<_>::new();

    for n in 0..2 {
        let lc2_val = match n {
            0 => lc2.0,
            1 => lc2.1,
            2 => lc2.2,
            _ => 0          
        };
        let rc1_val = match n {
            0 => rc1.0,
            1 => rc1.1,
            2 => rc1.2,
            _ => 0        
        };

        let n = n as i32;
        if lc2_val - rc1_val > 0 {
            candidates1.push(generate_candidates(lc2, rc2, Axis::try_from(n).unwrap()));
            candidates2.push(generate_candidates(rc1, lc1, Axis::try_from(n).unwrap()));
        } else if lc2_val - rc1_val == 0 {
            panic!("Rooms have overlapping edges, critical error in BSP room creation function")
        } else {
            candidates1.push(generate_candidates(rc2, lc2, Axis::try_from(n).unwrap()));
            candidates2.push(generate_candidates(lc1, rc1, Axis::try_from(n).unwrap()));
        }
    }

    //TODO: Implement distance algorithm for each candidate array, ideally in O(k log_k), somehow a hard task

    let mut shortest =  f64::INFINITY;
    let mut shortest_points = (Point3(0,0,0), Point3(0,0,0));

    let mut c1 = Vec::<Point3>::new();
    let mut c2 = Vec::<Point3>::new();
    candidates1.iter().for_each(|v| c1.extend(v));
    candidates2.iter().for_each(|v| c2.extend(v));

    for e1 in &c1 {
        for e2 in &c2 {
            let dist = ((e1.0 as f64 + e2.0 as f64).powf(2.0) + (e1.1 as f64 + e2.1 as f64).powf(2.0) + (e1.2 as f64 + e2.2 as f64).powf(2.0)).sqrt();
            if dist < shortest {
                shortest = dist;
                shortest_points = (*e1, *e2); 
            }
        }
    }
    let shortest_points = ((indices.0, shortest_points.0, shortest_points.1), (indices.1, shortest_points.0, shortest_points.1));
    EDGES.write().unwrap().push(shortest_points.0);
    EDGES.write().unwrap().push(shortest_points.1);
}

/*

*/

pub fn edge_dfs(root: &BSPNode<Tile>, divisions: i64, l_rooms: Arc<RwLock<Vec<Room>>>, r_rooms: Arc<RwLock<Vec<Room>>>) {
    fn leafeon(root: &BSPNode<Tile>, l_rooms: Arc<RwLock<Vec<Room>>>, r_rooms: Arc<RwLock<Vec<Room>>>) {
        match ((root.left.as_deref().unwrap().value.room), (root.right.as_deref().unwrap().value.room)) {
            (Some(l), Some(r)) => {
                generate_edges((l, r));
                l_rooms.write().unwrap().push(l);
                r_rooms.write().unwrap().push(r);
            },
            (Some(l), None) => {l_rooms.write().unwrap().push(l)},
            (None, Some(r)) => {r_rooms.write().unwrap().push(r)}
            (None, None) => ()
        }
    }

    if root.right.is_some() && divisions - root.value.split_count > 1 {
        edge_dfs(&(root.right.as_deref().unwrap()), divisions, Arc::clone(&l_rooms), Arc::clone(&r_rooms));
        edge_dfs(&(root.left.as_deref().unwrap()), divisions, Arc::clone(&l_rooms), Arc::clone(&r_rooms));
    } else if root.right.is_some() && divisions - root.value.split_count == 1   {
        leafeon(root, Arc::clone(&l_rooms), Arc::clone(&r_rooms));
    } else if !(l_rooms.read().unwrap().is_empty() || r_rooms.read().unwrap().is_empty()) {
        generate_edges((*l_rooms.read().unwrap().last().unwrap(), *r_rooms.read().unwrap().first().unwrap()));        
    }
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

