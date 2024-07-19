fn main() {
    // struct definition
    struct Vec3 {
        x: u32,
        y: u32,
        z: u32,
    }

    // simple struct and accessing fields
    let v = Vec3 {
        x: 10,
        y: 20,
        z: 30,
    };
    println!("v = ({}, {}, {})", v.x, v.y, v.z);

    // destructuring structs
    let Vec3 { x, y, z } = v;

    println!("x={}, y={}, z={}", x, y, z);
}
