#![allow(dead_code)]
//#![allow(unused_imports)]
#![warn(unused_imports)]
#![warn(unused_variables)]

mod pattern_matching;
mod stack_and_heap;

use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
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
        Color::CmykColor { black: 255, .. } => println!("Enums: color black"),
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
        (x + y, x * y)
    }
    let tuple: (i32, i32) = (10, 20);
    let sp = sum_and_product(10, 20);
    println!("Tuples: {:?} -> sum and product {:?}", tuple, sp);
    println!("Tuples: (v1) sum {} and product {}", sp.0, sp.1);

    let (sum, product) = sp;
    println!("Tuples: (v2) sum {} and product {}", sum, product);
}

fn generics() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2: Point<i8> = Point { x: 3, y: 4 };
    let p3: Point<f32> = Point { x: 5.5, y: 6.4 };
    println!(
        "Generics: ({},{}), ({},{}), ({},{})",
        p1.x, p1.y, p2.x, p2.y, p3.x, p3.y
    );
}

fn vectors() {
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    println!("Vectors: {:?}", vec1);
    vec1.push(4);

    // Can't use u32/i32 for indexing - different operation systems with different.
    // It should be isize/usize
    let first_index: usize = 0;
    println!(
        "Vectors: {:?}, the first element is {} and last added is {}",
        vec1,
        vec1[first_index],
        vec1.last().unwrap()
    );

    match vec1.get(6) {
        Some(x) => println!("Vectors: Item 6 is exist and it is {}", x),
        None => println!("Vectors: Item 6 is not exists"),
    }

    for x in &vec1 {
        println!("Vectors: Value {}", x);
    }

    let removed_last_element = vec1.pop(); // can't use let Some(x) coz it's refutable
    println!(
        "Vectors: Last item removed. So it was: {:?}",
        removed_last_element
    );

    while let Some(x) = vec1.pop() {
        // let Some can fails and break while loop
        println!("Vectors: Removing value {}", x);
    }
}

fn hashmap() {
    let mut shapes_map = HashMap::new();
    shapes_map.insert(String::from("triangle"), 3);
    shapes_map.insert(String::from("square"), 4);

    println!(
        "HashMaps: Indexing: The square has {} sides",
        shapes_map["square"]
    );

    // Note: .into() method used to convert string to target type
    shapes_map.insert("square".into(), 5);

    for (shape, corners) in &shapes_map {
        println!("HashMaps: ForEach: The {} has {} sides", shape, corners);
    }

    shapes_map.entry("circle".into()).or_insert(1);

    {
        // Note: you can't dereference without .or_insert or without .or_default
        let actual = shapes_map.entry("circle".into()).or_default();
        *actual = 14;
    }

    println!("HashMaps: Whole print: {:?}", shapes_map);
}

fn hashset() {
    let mut letters = HashSet::new();
    letters.insert("alpha");
    letters.insert("beta");

    println!("HashSets: Whole print: {:?}", letters);

    let insert_result = letters.insert("beta");
    println!("HashSets: \"beta\" insert result: {}", insert_result);

    let insert_result = letters.insert("delta");
    println!("HashSets: \"delta\" insert result: {}", insert_result);

    let contain_result = letters.contains("gamma");
    println!("HashSets: Do we have \"gamma\"?: {}", contain_result);

    let remove_result = letters.contains("delta");
    println!("HashSets: Remove \"delta\" result: {}", remove_result);

    let remove_result = letters.contains("gamma");
    println!("HashSets: Remove \"gamma\" result: {}", remove_result);

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!(
        "HashSets: Subset: Is {:?} subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );
    println!(
        "HashSets: Disjoint: Is {:?} disjoin of {:?}? {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );
    println!(
        "HashSets: Union: {:?} unions with {:?} to {:?}",
        _1_5,
        _6_10,
        _1_5.union(&_6_10)
    );
    println!(
        "HashSets: Sub: {:?} minus {:?} is {:?}",
        _1_10,
        _2_8,
        &_1_10 - &_2_8
    );
    println!(
        "HashSets: Dif: {:?} difference from {:?} in {:?}",
        _1_10,
        _2_8,
        _1_10.difference(&_2_8)
    );
}

fn iterators() {
    let mut vec = vec![3, 2, 1];

    for v in vec.iter() {
        println!("Iterators. Iterating. {}", v);
    }

    for v in vec.iter_mut().rev() {
        *v *= 2;
        println!("Iterators. Iterating mutable reverse. {}", v);
    }

    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("Iterators. Extend (why iterating?). {:?}", vec2);
}

fn strings() {
    //let string_slice1 = "String slice 1";
    let string_slice2: &'static str = "STR SLICE ON STACK";

    for c in string_slice2.chars().rev() {
        println!("Strings: Slice. String slice characters: {}", c);
    }

    if let Some(first_char) = string_slice2.chars().nth(0) {
        println!("Strings: Slice. First character is: {}", first_char);
    }

    let mut letters = String::new();
    let mut letter = 'a' as u8;
    while letter <= ('g' as u8) {
        letters.push(letter as char);
        letters.push_str(",");
        letter += 1;
    }
    println!("Strings: String. {:?}", letters);

    // Convert
    let to_slice = &letters;
    let to_string = String::from(string_slice2);
    println!(
        "Strings: Convert: To slice: {:?} ::: To string: {:?}",
        to_slice, to_string
    );

    // Concatenation
    let concat1 = letters + "abc" + &string_slice2;
    let mut concat2 = "Hello world".to_string();
    concat2.remove(0);
    concat2.push_str("!!!");
    println!(
        "Strings: Concat: {:?} ::: {:?}",
        concat1,
        concat2.replace("ello", "goodbye")
    );
}

fn string_formatting() {
    let first_name = "John";
    let last_name = "Doe";
    let greeting = format!("Hi, my name is {0} {1}. Just {0}", first_name, last_name);
    println!("String Formatting: Simple pattern: {}", greeting);

    let greeting = format!("{1} {} {0} {} {delta}", "alpha", "beta", delta = "delta");
    println!("String Formatting: Mixed pattern: {}", greeting);
}

fn number_guessing_game() {
    let number = rand::thread_rng().gen_range(1..=101); // exclusive, so let's use 101 to include 100

    println!("Number guessing game: Enter your guess ");
    loop {
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Number guessing game: Your guest is out of range. Try again");
                        } else if guess < number {
                            println!("Number guessing game: Your guest is too LOW. Try again");
                        } else if guess > number {
                            println!("Number guessing game: Your guest is too HIGH. Try again");
                        } else {
                            println!("Number guessing game: Correct!");
                            break;
                        }
                    }
                    Err(e) => println!(
                        "Number guessing game: Could not read input: {}, try again",
                        e
                    ),
                }
            }
            Err(_) => continue,
        }
    }
}

fn functions() {
    fn print_value(x: i32) {
        println!("Functions: Printing argument. {}", x);
    }
    print_value(33);

    fn increase(x: &mut i32) {
        *x += 1;
    }
    let mut z = 1;
    increase(&mut z);
    println!("Functions: Increasing argument. Has was 1, now it is {}", z);

    fn product(x: i32, y: i32) -> i32 {
        x * y
    }
    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("Functions: Return value. Product of {} ^ {} = {}", a, b, p);
}

fn methods() {
    fn print_vector(x: &Vec<i32>) {
        println!("Inside print_vector function {:?}", x);
    }
    let v = vec![10, 20, 30];
    print_vector(&v);
    println!("{}", v[0]); // this line gives error

    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    struct Line {
        start: Point,
        end: Point,
    }

    impl Line {
        // You should have &self as 1st argument
        fn len(&self) -> f64 {
            let dx = self.start.x - self.end.x;
            let dy = self.start.y - self.end.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let line = Line { start: p1, end: p2 };
    println!(
        "Methods: Distance between {:?} and {:?} is {}. 
         You can't use p1 and p2 here cuz they BORROWED by struct 'line'.
         Also you have to use #[derive(Debug)] to debug struct by default.",
        line.start,
        line.end,
        line.len()
    );
}

fn closures() {
    fn say_hello() {
        println!("Closures: Hello");
    }

    {
        // captured upper context
        let sh = say_hello;
        sh();
    }

    // lambda
    let plus_one = |x| x + 1;
    let six = 6;
    println!("Closures: {} + 1 = {}", six, plus_one(six));

    let two = 2;
    let plus_two = |x: i32| -> i32 {
        let mut z = x;
        z += two;
        z
    };
    println!("Closures: {} + 2 = {}", two, plus_two(two));

    // Can't use: 'two' borrowed by lambda.
    // You can closure lambda and println to brackets.
    //let borrow_two = &mut two;

    /* Lambda closures
     * T: pass by values
     * T&
     * &mut &
     */
    let plus_three = |x: &mut i32| *x += 3;
    let mut five_or_eight = 5;
    plus_three(&mut five_or_eight);
    println!("Closures: 5 + 3 = {}", five_or_eight);
}

fn high_order_functions() {
    // functions that take functions as arguments
    // f(g) { let x = g(); }

    // function that return functions
    // generators
    // f() -> g

    // Example: sum of all even squares < 500
    fn generate_greater_than(limit: u32) -> impl Fn(u32) -> bool {
        move |x| x > limit
    }

    let limit = 500;
    let mut sum = 0;

    let is_even = |x| x % 2 == 0;
    //let above_limit = |x| x > limit;
    let above_limit = generate_greater_than(limit);

    for i in 0.. {
        let isq = i * i;

        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("High order functions: Sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("High order functions: Sum2 = {}", sum2);
}

fn traits() {
    trait Animal {
        fn create(name: &'static str) -> Self;
        fn name(&self) -> &'static str;
        fn talk(&self) {
            println!("Traits: {} can't talk", self.name());
        }
    }

    struct Human {
        name: &'static str,
    }

    impl Animal for Human {
        fn create(name: &'static str) -> Self {
            Human { name: name }
        }
        fn name(&self) -> &'static str {
            self.name
        }
        fn talk(&self) {
            println!("Traits: {} says hello", self.name());
        }
    }

    let human = Human { name: "John" };
    human.talk();

    struct Cat {
        name: &'static str,
    }

    impl Animal for Cat {
        fn create(name: &'static str) -> Self {
            Cat { name } // Shorthand initialization
        }
        fn name(&self) -> &'static str {
            self.name
        }
        fn talk(&self) {
            println!("Traits: {} says meow", self.name());
        }
    }

    let cat = Cat { name: "Wooly" };
    cat.talk();

    let static_create_human = Human::create("Bill");
    static_create_human.talk();

    // Notice that it's available to use explicit variable and train creator
    let static_create_animal: Cat = Animal::create("BarkProtector");
    static_create_animal.talk();

    trait Summable<T> {
        fn sum(&self) -> T;
    }

    impl Summable<i32> for Vec<i32> {
        fn sum(&self) -> i32 {
            // 'Classic way'
            //let mut result: i32 = 0;
            //for x in self {
            //    result += *x;
            //}
            //result

            // Functional way
            self.iter().fold(0, |a, &b| a + b)
        }
    }

    let vector = vec![1, 2, 3];
    println!("Traits: Sum of {:?} is {}", vector, vector.sum());
}

use std::fmt::Debug;
fn traits_as_parameters() {
    #[derive(Debug)]
    struct Circle {
        radius: f64,
    }
    #[derive(Debug)]
    struct Square {
        side: f64,
    }
    trait Shape {
        fn area(&self) -> f64;
    }
    impl Shape for Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    // You can combine trait as you wish, but Debug require std::fmt::Debug
    //fn print_area(shape: impl Shape + Debug) {

    // Benefits: you can declare shape: T, shape2: T, shape3: T
    //fn print_area<T: Shape + Debug>(shape: T) {

    fn print_area<T>(shape: T)
    where
        T: Shape + Debug,
    {
        println!(
            "Traits as parameters: Shape {:?} has area {}",
            shape,
            shape.area()
        );
    }
    print_area(Circle { radius: 5.0 });
}

// Convertor from one type to another
fn into_trait() {
    struct Person {
        name: String,
    }
    impl Person {
        //fn new(name: &str) -> Person {
        //    Person { name: name.to_string() }
        //}

        //fn new<S: Into<String>>(name: S) -> Person {
        fn new<S>(name: S) -> Person
        where
            S: Into<String>,
        {
            Person { name: name.into() }
        }
    }

    let john = Person::new("John");
    let jane_name: String = "Jane".to_string();
    let jane = Person::new(jane_name /*.as_ref()*/);
    println!("Into trait: {} and {} are married", john.name, jane.name);
}

// Destructor
fn drop_trait() {
    struct Creature {
        name: String,
    }
    impl Creature {
        fn new(name: &str) -> Creature {
            println!("Drop trait: {} enters the game", name);
            Creature { name: name.into() }
        }
    }
    impl Drop for Creature {
        fn drop(&mut self) {
            println!(
                "Drop trait: {} leaves the game (drop trait = auto destructor)",
                self.name
            );
        }
    }

    {
        let wizard = Creature::new("Wizard");
        let goblin = Creature::new("Goblin");
        println!("Drop trait: Game starts");
        println!(
            "Drop trait: Game processing: {} attacks {}",
            wizard.name, goblin.name
        );
        drop(goblin);
        println!("Drop trait: Game ends");
    }

    println!("----------------------------------");

    let clever: Creature;
    {
        println!("Drop trait: Game starts");
        let orc = Creature::new("Orc");
        println!("Drop trait: Game processing: In the scoop {}", orc.name);
        clever = orc;
    }
    println!("Drop trait: Game processing: Out of scoop {}", clever.name);
    println!("Drop trait: Game ends");
}

use std::{
    cmp::{
        Eq,
        PartialEq
    },
    ops::{
        Add, 
        AddAssign, 
        Neg
    }
};
fn operator_overload_via_trait() {
    #[derive(Debug/*, PartialEq, Eq, PartialOrd, Ord*/)]
    struct Complex<T> {
        re: T,
        im: T
    }
    impl<T> Complex<T> {
        fn new(re: T, im: T) -> Complex<T> {
            Complex::<T> { re, im }
        }
    }
    //impl Add for Complex<i32> {
    //    type Output = Complex<i32>;
    //    fn add(self, rhs: Self) -> Self::Output {
    //        // unimplemented!()
    //        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    //    }
    //}
    impl<T> Add for Complex<T> where T: Add<Output = T> {
        type Output = Complex<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Complex { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }

    let complex1 = Complex::new(1, 1);
    let complex2 = Complex::new(2, 2);
    println!("Overload traits: Sum of two complex values is {:?}", complex1 + complex2);

    let complex3 = Complex::new(3.0, 3.0);
    let complex4 = Complex::new(4.0, 4.0);
    println!("Overload traits: Sum of two complex values is {:?}", complex3 + complex4);

    impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.re += rhs.re;
            self.im += rhs.im;
        }
    }
    let mut complex5 = Complex::new(5.0, 5.0);
    let complex6 = Complex::new(6.0, 6.0);
    complex5 += complex6;
    println!("Overload traits: Add assign of next complex values is {:?}", complex5);

    impl<T> Neg for Complex<T> where T: Neg<Output = T> {
        type Output = Complex<T>;
        fn neg(self) -> Self::Output {
            Complex { re: -self.re, im: -self.im }
        }
    }
    println!("Overload traits: Inverse (neg) previous complex values is {:?}", -complex5);

    // you can't support "full equality" trait for f32 cuz you'll get NAN infection,
    // where some value can become NAN (Not A Number) after 0/0 or inf/inf.
    // The problem here is you'll get NAN == NAN = false and NAN * x = NAN

    // So you have to support "partial equality"
    impl<T> PartialEq for Complex<T> where T: PartialEq
    {
        fn eq(&self, rhs: &Self) -> bool {
            self.re == rhs.re && self.im == rhs.im
        }
    }
    let complex7 = Complex::new(7, 7);
    let complex8 = Complex::new(8, 8);
    println!("Overload traits: Are they equal? {}", complex7 == complex8);
    
    impl<T: Eq> Eq for Complex<T> where T: Eq
    {
    }
    let complex9_1 = Complex::new(9, 9);
    let complex9_2 = Complex::new(9, 9);
    println!("Overload traits: Are they equal? {}", complex9_1 == complex9_2);
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
    //vectors();
    //hashmap();
    //hashset();
    //iterators();
    //strings();
    //string_formatting();
    //number_guessing_game();
    //functions();
    //methods();
    //closures();
    //high_order_functions();
    //traits();
    //traits_as_parameters();
    //into_trait();
    //drop_trait();
    operator_overload_via_trait();

    //pattern_matching::execute();

    //unsafe
    //{
    //    MUTABLE_STATIC_MEANING_OF_LIFE = 1;
    //}
}
