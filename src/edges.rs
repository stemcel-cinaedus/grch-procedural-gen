use core::panic;
use std::sync::atomic::{AtomicUsize};
use std::sync::{RwLock};
use std::todo;
use bumpalo::Bump;
use bumpalo::collections::Vec as BumpVec;

use crate::{PLACE_DIST, ROOM_SCALE_FACTOR};
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

/*

*/

pub fn orthogonal_paths(edges: Vec<(usize, Point3, Point3, Axis)>, map: Vec<Vec<(usize, i64, i64)>>) -> Vec<(usize, Point3, Point3, Axis)> {
    //TODO: Fix vector pass chain so that a function in edges.rs calls orthogonal rooms
    //TODO 2: Bound the paths using the values used in room construction so that clipping through a room is impossible

    //
    
    let delta: f64 = 1.0;
    let mut new_edges = Vec::new();

    //Take all the edges and do something something something -> now there are orthogonal paths :D
    for e in edges.iter() { 
        let mut k = 0.0;
        let mut dx = (e.2.0 - e.1.0) as f64;
        let mut dy = (e.2.1 - e.1.1) as f64;
        let mut dz = (e.2.2 - e.1.2) as f64;

        let mut ex = *e;
        let mut ey = *e;
        let mut ez = (e.0, e.1, e.1, e.3);

        let delta_o;

       //Edges now have the index of the room they start from, so this function can be refactored to do lattice routing
       //I am not sure how to do it without spamming conditionals, however.
       //The map vec has vecs sorted by psoition in the vec's respective dimension, so as long as the index of the current room is known, it will be easy to look at the dimensions of the next room.

        //Take edge, find room edge is from in the map vectors, calculate the maximum allowed space, if the space is exceeded, find which room's space the edge is now in, and work based off that
        //Remember that the stored positions in map are the right corners of the room

        //Make free space a vec for now, change it to be a tuple with one space for every dimension later
        let mut free_space = Vec::<i64>::new(); 
        let start_pos = e.1;

        for v in &map {
            //The span of the tile that contains the room minus the span of the room
            let bounds = (v[e.0].2 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64 - v[e.0].1 * (1.0 / (1.0 - ROOM_SCALE_FACTOR)) as i64) - (v[e.0].2 - v[e.0].1);
            free_space.push(bounds);
        }

       //Math might be fucked now becase refactor
        
        match e.3 {
            Axis::X => {
                delta_o = dx / 4.0;
                dx = 3.0 * (dx / 4.0);
                ez = (ez.0, ez.1, Point3(ez.1.0 + (delta_o / 2.0) as i64, ez.1.1, ez.1.2), ez.3);
                new_edges.push(ez);

            },
            Axis::Y => {
                delta_o = dy / 4.0;
                dy = 3.0 * (dy / 4.0);
                ez = (ez.0, ez.1, Point3(ez.1.0, ez.1.1 + (delta_o / 2.0) as i64, ez.1.2), ez.3);
                new_edges.push(ez);

            },
            Axis::Z => {
                delta_o = dz / 4.0;
                dz = 3.0 * (dz / 4.0);
                ez = (ez.0, ez.1, Point3(ez.1.0, ez.1.1, ez.1.2 + (delta_o / 2.0) as i64), ez.3);
                new_edges.push(ez);

            }
        }
        
        while k < 1.0 {
            //There is probably a more efficient way to do this, but I am not really conscious right now so this is the best you will get
            ex = (ez.0, ez.2, Point3(ez.2.0 + (dx * delta) as i64, ez.2.1, ez.2.2), e.3);
            ey = (ez.0, ex.2, Point3(ex.2.0, ex.2.1 + (dy * delta) as i64, ex.2.2), e.3);
            ez = (ez.0, ey.2, Point3(ey.2.0, ey.2.1, ey.2.2 + (dz * delta) as i64), e.3);

            k += delta;
            new_edges.push(ex);
            new_edges.push(ey);
            new_edges.push(ez);
        }

        match e.3 {
            Axis::X => {
                ez = (ez.0, ez.2, Point3(ez.2.0 + (delta_o / 2.0) as i64, ez.2.1, ez.2.2), ez.3);
                new_edges.push(ez);
            }
            Axis::Y => {
                ez = (ez.0, ez.2, Point3(ez.2.0, ez.2.1 + (delta_o / 2.0) as i64, ez.2.2), ez.3);
                new_edges.push(ez);
            }
            Axis::Z => {
                ez = (ez.0, ez.2, Point3(ez.2.0, ez.2.1, ez.2.2 + (delta_o / 2.0) as i64), ez.3);
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