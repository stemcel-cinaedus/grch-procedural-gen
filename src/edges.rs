use core::panic;
use std::sync::atomic::{AtomicUsize};
use std::sync::{RwLock};
use bumpalo::Bump;
use bumpalo::collections::Vec as BumpVec;

use crate::{ROOM_SCALE_FACTOR};
use crate::CORRIDOR_WIDTH;

use crate::types::*;

pub static EDGES: RwLock<Vec<(usize, Point3, Point3, Axis)>> = RwLock::new(Vec::<(usize, Point3, Point3, Axis)>::new());
pub static INDEX: AtomicUsize = AtomicUsize::new(0);

fn edge_rooms(r1: &Room, r2: &Room, axis: Axis) -> (usize, Point3, Point3, Axis) {
    match axis {
        Axis::X => {
            let left_mid = Point3(r1.2.0,
                r1.2.1 - ((r1.2.1 - r1.1.1) / 2), 
                r1.2.2 - ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0,
                r2.1.1 + ((r2.2.1 - r2.1.1) / 2), 
                r2.1.2 + ((r2.2.2 - r2.1.2) / 2));
            
            return (r1.0, left_mid, right_mid, axis)
        }, //Add steepness check later
        Axis::Y => {
            let left_mid = Point3(r1.2.0 - ((r1.2.0 - r1.1.0) / 2),
                r1.2.1,
                r1.2.2 - ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0 + ((r2.2.0 - r2.1.0) / 2),
                r2.1.1,
                r2.1.2 + ((r2.2.2 - r2.1.2) / 2));

            //EDGES.write().unwrap().push((left_mid, right_mid, axis));
            return (r1.0, left_mid, right_mid, axis)
        }
        Axis::Z => {
            let left_mid = Point3(r1.2.0  - ((r1.2.0 - r1.1.0) / 2),
                r1.2.1 - ((r1.2.1 - r1.1.1) / 2), 
                r1.2.2);
            let right_mid = Point3(r2.1.0 + ((r2.2.0 - r2.1.0) / 2),
                r2.1.1 + ((r2.2.1 - r2.1.1) / 2), 
                r2.1.2);

            //EDGES.write().unwrap().push((left_mid, right_mid, axis));
            return (r1.0, left_mid, right_mid, axis)
        } 
    }
}

fn generate_edges(rooms: (&[&Room], &[&Room]), axis: Axis, split_pos: Point3) -> () {
    //Refactor to make it find the rooms with the room positions closest to the split , 
    //Lazy to implement rn, but basically it would be like match the plane & split point, and then grab the top n rooms on the left that are closest to the split,
    //grab the top n rooms on the right that are closest to the split, and run generate candidates on them for a total of n*n calculations, saving a lot more resources than just 
    //checking every possible vertice.
    fn get_point_dist(r: &&Room, p: Point3) -> (i64, i64) {
        return (((p.0 - r.1.0).pow(2) + (p.1 - r.1.1).pow(2) + (p.2 - r.1.2).pow(2)), ((p.0 - r.2.0).pow(2) + (p.1 - r.2.1).pow(2) + (p.2 - r.2.2).pow(2)))
    }

    if rooms.0.is_empty() || rooms.1.is_empty() {
        return
    }

    let mut left_closest = (rooms.0[0], i64::MAX);
    rooms.0.iter().for_each(|r| {
        let d = get_point_dist(r, split_pos);
        if d.0 < left_closest.1 || d.1 < left_closest.1 {
            left_closest = (r, d.0.min(d.1));           
        }
    });

    let mut right_closest = (rooms.0[0], i64::MAX);
    rooms.1.iter().for_each(|r| {
        let d = get_point_dist(r, split_pos);
        if d.0 < right_closest.1 || d.1 < right_closest.1 {
            right_closest = (r, d.0.min(d.1));           
        }
    });

    EDGES.write().unwrap().push(edge_rooms(left_closest.0, right_closest.0, axis));  


    //Now I should decide how exactly I want to find the rooms closest to the centers.

    //Now the idea is to take the n closest and draw corridors between them
    //1 for testing currently
    //TODO: Implement distance algorithm for each candidate array, ideally in O(k log_k), somehow a hard task
}

pub fn orthogonal_paths(edges: Vec<(usize, Point3, Point3, Axis)>, map: Vec<Vec<(usize, Point3, Point3)>>) -> Vec<(usize, Point3, Point3, Axis)> {
    //TODO: Fix vector pass chain so that a function in edges.rs calls orthogonal rooms
    
    let delta: f64 = 1.0;
    let mut new_edges = Vec::new();

    for e in edges.iter() {
        let mut dx = (e.2.0 - e.1.0) as f64;
        let mut dy = (e.2.1 - e.1.1) as f64;
        let mut dz = (e.2.2 - e.1.2) as f64;

        let mut ex = *e;
        let mut ey = *e;
        let mut ez = (e.0, e.1, e.1, e.3);

        let delta_o;
        let mut start_pos = e.1;

        //The span of the tile that cnontains the room minus the span of the room
        //Should not be treated as an actual point
        fn get_free_space(lc: Point3, rc: Point3) -> Point3 {
                let b0 = (rc.0 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64 - lc.0 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64) - (rc.0 - lc.0);
                let b1 = (rc.1 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64 - lc.1 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64) - (rc.1 - lc.1);
                let b2 = (rc.2 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64 - lc.2 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64) - (rc.2 - lc.2);
                return Point3(b0, b1, b2);
            }
        let mut free_space = get_free_space(e.1, e.2);

       //Math might be fucked now becase refactor
        
        match e.3 {
            Axis::X => {
                //Add check to ensure delta_o movement doesn't escape bounds 
                delta_o = (dx / 4.0) as i64;
                dx -= delta_o as f64;
                ez = (ez.0, ez.1, Point3(ez.1.0 + delta_o / 2, ez.1.1, ez.1.2), ez.3);
                new_edges.push(ez);


            },
            Axis::Y => {
                delta_o = (dy / 4.0) as i64;
                dy -= delta_o as f64;
                ez = (ez.0, ez.1, Point3(ez.1.0, ez.1.1 + (delta_o / 2), ez.1.2), ez.3);
                new_edges.push(ez);

            },
            Axis::Z => {
                delta_o = (dz / 4.0) as i64;
                dz -= delta_o as f64;
                ez = (ez.0, ez.1, Point3(ez.1.0, ez.1.1, ez.1.2 + (delta_o / 2)), ez.3);
                new_edges.push(ez);

            }
        }
        
        let target: Point3 = match e.3 {
            Axis::X => Point3(e.2.0 - (delta_o / 2), e.2.1, e.2.2),
            Axis::Y => Point3(e.2.0, e.2.1 - (delta_o / 2), e.2.2),
            Axis::Z => Point3(e.2.0, e.2.1, e.2.2 - (delta_o / 2)),
        };

        while ez.2 != target {

            //X bounds check
            let x_change = ez.2.0 +  (dx * delta) as i64;
            if x_change > start_pos.0 + free_space.0 {
                //If this doesn't work, double check that tile bounds are allowed to overlap
                let r = map[0].iter()
                        .filter(|t| t.1.0 == x_change + free_space.0 || t.2.0 == x_change + free_space.0)
                        .min_by_key(|t| ((e.1.sum().pow(2) + e.2.sum().pow(2)) - (t.1.sum().pow(2) + t.2.sum().pow(2))).pow(2));
                let r = match r {
                        Some(v) => v,
                        None => panic!("No room found with bounds that match queried bounds! Critical error in pathing function!")
                    };

                start_pos = Point3(start_pos.0 + free_space.0, ez.2.1, ez.2.2);
                free_space = get_free_space(r.1, r.2);
                dx += (x_change - free_space.0) as f64;

            }

            ex = (ez.0, ez.2, Point3(ez.2.0 + (dx * delta) as i64, ez.2.1, ez.2.2), e.3);

            //Y bounds check
            let y_change = ex.2.1 +  (dy * delta) as i64;
            if y_change > start_pos.1 + free_space.1 {
                let r = map[1].iter()
                        .filter(|t| t.1.1 == y_change + free_space.1 || t.2.1 == y_change + free_space.1)
                        .min_by_key(|t| ((e.1.sum().pow(2) + e.2.sum().pow(2)) - (t.1.sum().pow(2) + t.2.sum().pow(2))).pow(2));
                let r = match r {
                        Some(v) => v,
                        None => panic!("No room found with bounds that match queried bounds! Critical error in pathing function!")
                    };

                start_pos = Point3(ex.2.0, start_pos.1 + free_space.1, ex.2.2);
                free_space = get_free_space(r.1, r.2);
                dy += (y_change - free_space.1) as f64;

            }

            ey = (ez.0, ex.2, Point3(ex.2.0, ex.2.1 + (dy * delta) as i64, ex.2.2), e.3);

            //Z bounds check
            let z_change = ey.2.2 +  (dz * delta) as i64;
            if z_change > start_pos.2 + free_space.2 {
                let r = map[2].iter()
                        .filter(|t| t.1.2 == z_change + free_space.2 || t.2.2 == z_change + free_space.2)
                        .min_by_key(|t| ((e.1.sum().pow(2) + e.2.sum().pow(2)) - (t.1.sum().pow(2) + t.2.sum().pow(2))).pow(2));
                let r = match r {
                        Some(v) => v,
                        None => panic!("No room found with bounds that match queried bounds! Critical error in pathing function!")
                    };

                start_pos = Point3(ey.2.0, ey.2.1, start_pos.2 + free_space.2);
                free_space = get_free_space(r.1, r.2);
                dz += (z_change - free_space.2) as f64;

            }
            ez = (ez.0, ey.2, Point3(ey.2.0, ey.2.1, ey.2.2 + (dz * delta) as i64), e.3);

            new_edges.push(ex);
            new_edges.push(ey);
            new_edges.push(ez);
        }

        match e.3 {
            Axis::X => {
                ez = (ez.0, ez.2, Point3(ez.2.0 + (delta_o / 2), ez.2.1, ez.2.2), ez.3);
                new_edges.push(ez);
            }
            Axis::Y => {
                ez = (ez.0, ez.2, Point3(ez.2.0, ez.2.1 + (delta_o / 2), ez.2.2), ez.3);
                new_edges.push(ez);
            }
            Axis::Z => {
                ez = (ez.0, ez.2, Point3(ez.2.0, ez.2.1, ez.2.2 + (delta_o / 2)), ez.3);
                new_edges.push(ez);
            }
        }
    }

    return new_edges
}

pub fn create_corridors(edges: Vec<(usize, Point3, Point3, Axis)>) -> Vec<(Point3, Point3)> {
    //Incomplete, needs math to fill in gaps between corridor boxes (if there is a Y offset, you will just be able to see inside from the gap)

    let mut boxes = Vec::<(Point3, Point3)>::new();
    for e in edges {
        let c1 = Point3(e.1.0 - CORRIDOR_WIDTH, e.1.1 - CORRIDOR_WIDTH, e.1.2 - CORRIDOR_WIDTH);
        let c2 = Point3(e.2.0 + CORRIDOR_WIDTH, e.2.1 + CORRIDOR_WIDTH, e.2.2 + CORRIDOR_WIDTH);
        boxes.push((c1, c2)); 
    }
    boxes
}

pub fn edge_dfs<'a>(root: &'a BSPNode<Tile>, divisions: u32, arena: &'a Bump) -> BumpVec<'a, &'a Room> {
    if divisions - root.value.split_count >= 1 {
        let mut left_rooms = edge_dfs(&(root.left.as_deref().unwrap()), divisions, arena);
        let right_rooms  = edge_dfs(&(root.right.as_deref().unwrap()), divisions, arena);
        generate_edges((&left_rooms, &right_rooms), root.split_d, (root.left.as_deref().unwrap()).value.rc);
        left_rooms.extend(right_rooms);
        return left_rooms
    }  else {
        let mut rooms = BumpVec::new_in(arena);
        if let Some(room) = &root.value.room {
            rooms.push(room);
        }
        return rooms
    }
}