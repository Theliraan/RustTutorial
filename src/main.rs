#![allow(dead_code)]
//#![allow(unused_imports)]
#![warn(unused_imports)]
#![warn(unused_variables)]

mod stack_and_heap;
mod pattern_matching;

use std::io::stdin;
use std::mem;

const MEANING_OF_LIFE: u8 = 42;
static STATIC_MEANING_OF_LIFE: i32 = 42;
static mut MUTABLE_STATIC_MEANING_OF_LIFE: u64 = 42;

fn scope_and_shadowing() {
    let a = 123;
    println!("Scoops: Non-Shadowed External A = {}", a);

    let a = 321; // Shadowing

    {
        let a = 456;
        let b = 456;
        println!("Scoops: Scooped A = {}", a);
        println!("Scoops: Scooped B = {}", b);
    }

    println!("Scoops: Shadowed A = {}", a);
}

fn operators() {
    let mut a: i32 = 2 + 3 * 4;
    println!("Arithmetics: a = {}", a);

    a += 2;
    println!("Arithmetics: {} % {} = {}", a, 3, a % 3);

    let a_squared = i32::pow(a, 2);
    println!("Arithmetics: {}^2 = {}", a, a_squared);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("Arithmetics: {} -> {} -> {}", b, b_cubed, b_to_pi);

    println!("Bitwise: 1 | 2 = {}", 1 | 2);
    println!("Bitwise: 1 << 10>> = {}", 1 << 10);

    println!("Logical: PI less 4 = {}", std::f64::consts::PI < 4.0);
    println!("Logical: PI equals 4 = {}", std::f64::consts::PI == 4.0);
}

fn basic_types() {
    let hello1: &str = "Hello world 1";
    let hello2: String = "Hello world 2".to_string();
    println!("{}", hello1);
    println!("{}", hello2);

    let a: u8 = 123;
    let mut b: i8 = 123;

    println!(" a = {}, b = {} with {} bytes", a, b, mem::size_of_val(&b));
    b = -123;

    println!(" a = {}, b = {} with {} bytes", a, b, mem::size_of_val(&b));

    // usize and isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, size of z = {}, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8 // 8 is "bits in byte"
    );

    let d: char = 'x';
    let size_of_d = mem::size_of_val(&d);
    println!("d = {}, size of d = {}", d, size_of_d);

    let f: f64 = 2.5;
    let size_of_f = mem::size_of_val(&f);
    println!("f = {}, size of f = {}", f, size_of_f);

    let boolean: bool = true;
    let size_of_boolean = mem::size_of_val(&boolean);
    println!(
        "boolean = {}, size of boolean = {}",
        boolean, size_of_boolean
    );
}

fn statements() {
    let temperature = 25;
    if temperature > 30
    // brackets are not good
    {
        // false
    } else if temperature < 10 {
        // false
    } else {
        // true
    }

    let day = if temperature > 20 { "sunny" } else { "cloudy" }; // almost ternary
}

fn cycles() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
    }
    println!("Cycles: While. X twice 64 times = {}", x);

    let mut y = 1;
    loop
    // while true
    {
        y *= 2;
        if y == 1 << 10 {
            break;
        }
    }
    println!("Cycles: Loop. Y shifted 10 times = {}", y);

    let mut z = 0;
    for i in 1..11 {
        z += i;
    }
    println!("Cycles: For. Z accumulate each value from 1 to 11 = {}", z);

    for (index, value) in (30..41).enumerate() {
        println!("Cycles: For Corteges. Index {} has value {}", index, value);
    }
}

fn matches() {
    let country_code = 5;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid", // remove this line and you'll get compile error
    };
    println!(
        "Match: The country {} has country code {}",
        country, country_code
    );
}

fn combinations() {
    enum State {
        Locked,
        Failed,
        Unlocked,
    }

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => entry.push_str(&input.trim_end()),
                    Err(_) => continue,
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("Combinations: FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("Combinations: UNLOCKED");
                return;
            }
        }
    }
}

fn structures() {
    struct Point {
        x: f64,
        y: f64,
    }

    struct Line {
        start: Point,
        end: Point,
    }

    let p = Point { x: 3.0, y: 4.0 };
    println!("Structures: Point p is at ({}, {})", p.x, p.y);
}

fn enums() {
    enum Color {
        Red,
        Green,
        Blue,
        RgbColor(u8, u8, u8), // tuple
        CmykColor {
            cyan: u8,
            magenta: u8,
            yellow: u8,
            black: u8,
        }, //struct
    }

    let color: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 255,
        black: 255,
    };
    match color {
        Color::Red => println!("Enums: color r"),
        Color::Green => println!("Enums: color g"),
        Color::Blue => println!("Enums: color b"),
        Color::RgbColor(0, 0, 0) => println!("Enums: color black"),
        Color::RgbColor(r, g, b) => println!("Enums: color ({}, {}, {})", r, g, b),
        /*Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("Enums: color black"),*/
        Color::CmykColor{black: 255,..} => println!("Enums: color black"),
        _ => (),
    }
}

fn unions() {
    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let int = unsafe { iof.i };
    println!("Unions: Int part is {}", int);

    fn process_value(iof: IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat { i: 42 } => println!("Unions: Int part is 42! OMG"),
                IntOrFloat { f } => println!("Unions: Is not 42. Interpreting as = {}", f),
            }
        }
    }
    process_value(IntOrFloat { f: 11.11 });
}

fn optionals() {
    let x = 3.0;
    let y = 0.0;

    let result = if y != 0.0 { Some(x / y) } else { None }; // result: Option<f32>
    match result {
        Some(z) => println!("Options. {}/{}={}", x, y, z),
        None => println!("Options. Can't divide by zero"),
    }

    if let Some(z) = result {
        println!("Options. {}/{}={}", x, y, z);
    }

    while let Some(z) = result {
        println!("Options. {}/{}={}", x, y, z);
    }
}

fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Arrays: Array has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("Arrays: {:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("Arrays: Not matching: {:?}", a);
    }

    let b = [1u16; 10]; // b.len() == 10;
    for i in 0..b.len() {
        println!("Arrays: Element in index {} has index {}", i, b[i]);
    }
    println!(
        "Arrays: Array with {} items use {} bytes",
        b.len(),
        mem::size_of_val(&b)
    );

    let matrix: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("Arrays: Matrix: {:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("Arrays: Matrix diagonal: {}", matrix[i][j]);
            }
        }
    }
}

fn slices() {
    fn use_slice1(slice: &[i32]) {
        println!("Slices: Array slice: {:?}", slice);
    }
    fn use_slice2(slice: &mut [i32]) {
        for i in 0..slice.len() {
            slice[i] *= 2;
        }
    }
    let mut data = [1, 2, 3, 4, 5];
    use_slice1(&data[1..4]);
    use_slice2(&mut data);
    use_slice1(&data);
}

fn tuples() {
    fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
        (x+y, x*y)
    }
    let tuple: (i32, i32) = (10, 20);
    let sp = sum_and_product(10, 20);
    println!("Tuples: {:?} -> sum and product {:?}", tuple, sp);
    println!("Tuples: (v1) sum {} and product {}", sp.0, sp.1);

    let (sum, product) = sp;
    println!("Tuples: (v2) sum {} and product {}", sum, product);
}

fn generics()
{
    struct Point<T> {
        x: T,
        y: T
    }

    let p1 = Point { x: 1, y: 2 };
    let p2:Point<i8> = Point { x: 3, y: 4 };
    let p3:Point<f32> = Point { x: 5.5, y: 6.4 };
    println!("Generics: ({},{}), ({},{}), ({},{})", p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
}

fn vectors()
{
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("Vectors: {:?}", vec1);
    vec1.push(4);

    // Can't use u32/i32 for indexing - different operation systems with different.
    // It should be isize/usize
    let first_index: usize = 0;
    println!("Vectors: {:?}, the first element is {} and last added is {}", 
        vec1,
        vec1[first_index],
        vec1.last().unwrap()
    );

    match vec1.get(6)
    {
        Some(x) => println!("Vectors: Item 6 is exist and it is {}", x),
        None => println!("Vectors: Item 6 is not exists"),
    }

    for x in &vec1 { println!("Vectors: Value {}", x); }

    let removed_last_element = vec1.pop(); // can't use let Some(x) coz it's refutable
    println!("Vectors: Last item removed. So it was: {:?}", removed_last_element);

    while let Some(x) = vec1.pop() { // let Some can fails and break while loop
        println!("Vectors: Removing value {}", x);
    }
}

fn main() {
    //stack_and_heap::execute();

    //basic_types();
    //operators();
    //scope_and_shadowing();
    //statements();
    //cycles();
    //matches();
    //combinations();
    //enums();
    //unions();
    //optionals();
    //arrays();
    //slices();
    //tuples();
    //generics();
    vectors();

    //pattern_matching::execute();

    //unsafe
    //{
    //    MUTABLE_STATIC_MEANING_OF_LIFE = 1;
    //}
}
