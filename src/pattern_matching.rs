fn how_many(x:i32) -> &'static str
{
    match x {
        0 => "no",
        1|2 => "one or two",
        12 => "dozen",
        z @ 9..=11 => "lots of", // you can use 'z' :)
        _ if (x % 2 == 0) => "some",
        _ => "few"
    }
}

pub fn execute()
{
    for x in 0..13 {
        println!("Patter Matching: I have {} oranges", how_many(x));
    }

    let point = (3, 4);
    match point {
        (0, 0) => println!("Patter Matching: Origin"),
        (0, y) => println!("Patter Matching: Zero X. Y = {}", y),
        (ref x, 0) => println!("Patter Matching: Zero Y. X = {}", x), // you can switch X 
        p => println!("Patter Matching: Point is {:?}", p),
    }
}