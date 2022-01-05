#![allow(dead_code)]

use std::mem;

struct Point
// 8 + 8 bytes = 16 bytes
{
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn execute() {
    let p1 = origin();
    let p2 = Box::new(origin());

    println!(
        "Stack & Heap: p1 takes up {} bytes (should be 16 bytes. No overhead on stack)",
        std::mem::size_of_val(&p1)
    );
    println!("Stack & Heap: p2 takes up {} bytes (should be 8 bytes. Stack contains only pointer to heap)", std::mem::size_of_val(&p2));

    let p3 = *p2;
    println!(
        "Stack & Heap: p3 takes up {} bytes (should be 16 bytes again. Relocated back to stack)",
        std::mem::size_of_val(&p3)
    );
}
