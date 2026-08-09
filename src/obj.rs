use crate::types::*;
use crate::edges::*;

pub fn create_obj() -> String {
    return String::from(print!("#OBJ file exported by grch-procedural-gen\n o grchDungeon\n"))
}

pub fn add_obj_box(c1: Point3, c2: Point3, obj_data: &mut String, v: usize) -> usize {
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c1.1, c1.2));
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c1.1, c1.2));
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c2.1, c1.2));
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c2.1, c2.2));
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c2.1, c2.2));
    obj_data.push_str(&format!("v {} {} {}\n", c2.0, c1.1, c2.2));
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c2.1, c1.2));
    obj_data.push_str(&format!("v {} {} {}\n", c1.0, c1.1, c2.2));

    let o = v;

    obj_data.push_str(&format!("f {} {} {} {}\n", o+1, o+7, o+5, o+8)); // Left
    obj_data.push_str(&format!("f {} {} {} {}\n", o+1, o+7, o+3, o+2)); // Front
    obj_data.push_str(&format!("f {} {} {} {}\n", o+2, o+3, o+4, o+6)); // Right
    obj_data.push_str(&format!("f {} {} {} {}\n", o+1, o+8, o+6, o+4)); // Bottom
    obj_data.push_str(&format!("f {} {} {} {}\n", o+7, o+5, o+4, o+3)); // Top
    obj_data.push_str(&format!("f {} {} {} {}\n", o+5, o+8, o+4, o+6)); // Back

    return o+8;
}

