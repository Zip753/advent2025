use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use rectangle::Point;

fn main() -> Result<(), Box<dyn Error>> {
    // open file
    let file = File::open("../input.txt")?;
    let reader = BufReader::new(file);

    // read lines into a vec of structs
    let mut points: Vec<Point> = vec![];
    for line in reader.lines() {
        let line = line?;
        let (x, y) = line.split_once(',').expect("invalid format");
        points.push(Point {
            x: x.parse::<u64>().expect("not a number"),
            y: y.parse::<u64>().expect("not a number"),
        });
    }

    // println!("points: {:?}", points);

    // call solve with vec
    let rect = rectangle::find_biggest_inner_rectangle(&points);
    if let Some(rect) = rect {
        // println!("{:?}, {:?}", rect.from, rect.to);
        println!("{}", rect.area());
    } else {
        println!("no inner rectangles found");
    }

    Ok(())
}
