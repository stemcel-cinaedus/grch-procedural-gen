use std::sync::atomic::{AtomicUsize};
use std::sync::{RwLock};

use crate::PLACE_DIST;
use crate::CORRIDOR_OFFSET;

use crate::types::*;

pub static EDGES: RwLock<Vec<(Point3, Point3)>> = RwLock::new(Vec::<(Point3, Point3)>::new());
pub static INDEX: AtomicUsize = AtomicUsize::new(0);

//Make Megumi holes
fn corridor_rooms(r1: &Room, r2: &Room, axis: Axis) -> () {
    /*
    //Possible methods to avoid O(n^2) calculations:
    -- Take the Vector of nodes you have and calculate its midpoint (prefix sums/DP is an obvious optimization). From that midpoint, create an "expanding zone" that
     -grows until it touches a room. Then find the room in the cluster closest to the point on the box and calculate the shortest distance between them. Cons: Must check the entire
     array of rooms every time the box is expanded. Enough to make it less efficient than the naive approach.

    -- Sort both Vecs, and have them each find the closest room to the split point. Then find the distance between these rooms. The most intensive task here will be the sorting, but
     - logarithmic time complexity means it's hardly a hassle. Sorting is not even necessary here, since just 1 value is needed. This means that it boils down to finding the room
     - closest to the center of the cluster (or the split point) on each side, and then just connecting them.
    */

    match axis {
        Axis::X => {
            let left_mid = Point3(r1.2.0 + ((r1.2.0 - r1.1.0) / 2), r1.2.1 + ((r1.2.1 - r1.1.2) / 2), r1.2.2);
            let right_mid = Point3(r2.1.0 - ((r2.2.0 - r2.1.0) / 2), r2.1.1 - ((r2.2.1 - r2.1.1) / 2), r2.1.2);

            EDGES.write().unwrap().push((left_mid, right_mid))
        }, //Add steepness check later
        Axis::Y => {}//DO NOTHING!!!!}, //Add vertical corridors later
        Axis::Z => {
            let left_mid = Point3(r1.2.0, r1.2.1 + ((r1.2.1 - r1.1.1) / 2), r1.2.2 + ((r1.2.2 - r1.1.2) / 2));
            let right_mid = Point3(r2.1.0, r2.1.1 - ((r2.2.1 - r2.1.1) / 2), r2.1.2 - ((r2.2.2 - r2.1.2) / 2));

            EDGES.write().unwrap().push((left_mid, right_mid))
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

    corridor_rooms(left_closest.0, right_closest.0, axis);  


    //Now I should decide how exactly I want to find the rooms closest to the centers.

    //Now the idea is to take the n closest and draw corridors between them
    //1 for testing currently
    //TODO: Implement distance algorithm for each candidate array, ideally in O(k log_k), somehow a hard task
}

/*

*/

pub fn edge_dfs(root: &BSPNode<Tile>, divisions: i64) -> Vec<&Room> {
    if divisions - root.value.split_count >= 1 {
        let mut left_rooms:Vec<&Room> = edge_dfs(&(root.left.as_deref().unwrap()), divisions);
        let right_rooms:Vec<&Room>  = edge_dfs(&(root.right.as_deref().unwrap()), divisions);
        generate_edges((&left_rooms, &right_rooms), root.split_d, (root.left.as_deref().unwrap()).value.rc);
        left_rooms.extend(right_rooms);
        return left_rooms
    }  else {
        let mut rooms = Vec::<&Room>::new();
        if root.value.room.is_some() {
            rooms.push(&(root.value.room.as_ref().unwrap()));
            return rooms
        } else {
            return rooms
        }
    }
}