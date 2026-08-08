use core::panic;
use std::sync::atomic::{AtomicUsize};
use std::sync::{RwLock};
use std::todo;
use bumpalo::Bump;
use bumpalo::collections::Vec as BumpVec;

use crate::PLACE_DIST;
use crate::CORRIDOR_OFFSET;

use crate::types::*;

pub static EDGES: RwLock<Vec<(Point3, Point3, Axis)>> = RwLock::new(Vec::<(Point3, Point3, Axis)>::new());
pub static INDEX: AtomicUsize = AtomicUsize::new(0);

//Make Megumi holes
fn corridor_rooms(r1: &Room, r2: &Room, axis: Axis) -> (Point3, Point3, Axis) {
    /*
    //Possible methods to avoid O(n^2) calculations:
    -- Take the Vector of nodes you have and calculate its midpoint (prefix sums/DP is an obvious optimization). From that midpoint, create an "expanding zone" that
     -grows until it touches a room. Then find the room in the cluster closest to the point on the box and calculate the shortest distance between them. Cons: Must check the entire
     array of rooms every time the box is expanded. Enough to make it less efficient than the naive approach.

    -- Sort both Vecs, and have them each find the closest room to the split point. Then find the distance between these rooms. The most intensive task here will be the sorting, but
     - logarithmic time complexity means it's hardly a hassle. Sorting is not even necessary here, since just 1 value is needed. This means that it boils down to finding the room
     - closest to the center of the cluster (or the split point) on each side, and then just connecting them.
    */

    //TODO: "Change the necessary functions to avoid this vec allocation. It's only like this for testing.";

    match axis {
        Axis::X => {
            let left_mid = Point3(r1.2.0,
                r1.2.1 - ((r1.2.1 - r1.1.1) / 2), 
                r1.2.2 - ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0,
                r2.1.1 + ((r2.2.1 - r2.1.1) / 2), 
                r2.1.2 + ((r2.2.2 - r2.1.2) / 2));
            
            return (left_mid, right_mid, axis)
        }, //Add steepness check later
        Axis::Y => {
            let left_mid = Point3(r1.2.0 - ((r1.2.0 - r1.1.0) / 2),
                r1.2.1,
                r1.2.2 - ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0 + ((r2.2.0 - r2.1.0) / 2),
                r2.1.1,
                r2.1.2 + ((r2.2.2 - r2.1.2) / 2));

            //EDGES.write().unwrap().push((left_mid, right_mid, axis));
            return (left_mid, right_mid, axis)
        }
        Axis::Z => {
            let left_mid = Point3(r1.2.0  - ((r1.2.0 - r1.1.0) / 2),
                r1.2.1 - ((r1.2.1 - r1.1.1) / 2), 
                r1.2.2);
            let right_mid = Point3(r2.1.0 + ((r2.2.0 - r2.1.0) / 2),
                r2.1.1 + ((r2.2.1 - r2.1.1) / 2), 
                r2.1.2);

            //EDGES.write().unwrap().push((left_mid, right_mid, axis));
            return (left_mid, right_mid, axis)
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

    EDGES.write().unwrap().push(corridor_rooms(left_closest.0, right_closest.0, axis));  


    //Now I should decide how exactly I want to find the rooms closest to the centers.

    //Now the idea is to take the n closest and draw corridors between them
    //1 for testing currently
    //TODO: Implement distance algorithm for each candidate array, ideally in O(k log_k), somehow a hard task
}

/*

*/

pub fn orthogonal_paths(edges: Vec<(Point3, Point3, Axis)>) -> Vec<(Point3, Point3, Axis)> {
    //TODO: Fix vector pass chain so that a function in edges.rs calls orthogonal rooms

    let delta: f64 = 0.5;
    let mut new_edges = Vec::new();

    //Take all the edges and do something something something -> now there are orthogonal paths :D
    for e in edges.iter() { 
        let mut k = 0.0;
        let mut dx = (e.1.0 - e.0.0) as f64;
        let mut dy = (e.1.1 - e.0.1) as f64;
        let mut dz = (e.1.2 - e.0.2) as f64;

        let mut ex = *e;
        let mut ey = *e;
        let mut ez = (e.0, e.0, e.2);

        let delta_o;

        //Take the axis the edges are split on, subtract the offset for that axis from Δₐₓᵢₛ (call this δₓ), start the corridor from a new edge that goes e₀ to 1/2 δₓ, then do the Orthogonal
        //path like normal, and then add another 1/2 δₓ to the end of that path to touch the face of the shape
        
        match e.2 {
            Axis::X => {
                delta_o = dx / 5.0;
                dx = 4.0 * (dx / 5.0);
                ez = (ez.0, Point3(ez.0.0 + (delta_o / 2.0) as i64, ez.0.1, ez.0.2), ez.2);
                new_edges.push(ez);

            },
            Axis::Y => {
                delta_o = dy / 5.0;
                dy = 4.0 * (dy / 5.0);
                ez = (ez.0, Point3(ez.0.0, ez.0.1 + (delta_o / 2.0) as i64, ez.0.2), ez.2);
                new_edges.push(ez);

            },
            Axis::Z => {
                delta_o = dz / 5.0;
                dz = 4.0 * (dz / 5.0);
                ez = (ez.0, Point3(ez.0.0, ez.0.1, ez.0.2 + (delta_o / 2.0) as i64), ez.2);
                new_edges.push(ez);

            }
        }
        
        while k < 1.0 {
            //There is probably a more efficient way to do this, but I am not really conscious right now so this is the best you will get
            ex = (ez.1, Point3(ez.1.0 + (dx * delta) as i64, ez.1.1, ez.1.2), e.2);
            ey = (ex.1, Point3(ex.1.0, ex.1.1 + (dy * delta) as i64, ex.1.2), e.2);
            ez = (ey.1, Point3(ey.1.0, ey.1.1, ey.1.2 + (dz * delta) as i64), e.2);

            k += delta;
            new_edges.push(ex);
            new_edges.push(ey);
            new_edges.push(ez);
        }

        match e.2 {
            Axis::X => {
                ez = (ez.1, Point3(ez.1.0 + (delta_o / 2.0) as i64, ez.1.1, ez.1.2), ez.2);
                new_edges.push(ez);
            }
            Axis::Y => {
                ez = (ez.1, Point3(ez.1.0, ez.1.1 + (delta_o / 2.0) as i64, ez.1.2), ez.2);
                new_edges.push(ez);
            }
            Axis::Z => {
                ez = (ez.1, Point3(ez.1.0, ez.1.1, ez.1.2 + (delta_o / 2.0) as i64), ez.2);
                new_edges.push(ez);
            }
        }
    }

    return new_edges
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