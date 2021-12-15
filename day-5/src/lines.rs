#![allow(dead_code)]
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(coordinates: (i32, i32)) -> Self {
        Point {
            x: coordinates.0,
            y: coordinates.1,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Line {
            start: start,
            end: end,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let points: Vec<(i32, i32)> = input
            .split(" -> ")
            .flat_map(|x| {
                x.split(",")
                    .map(|y| y.parse::<i32>().unwrap())
                    .collect_tuple::<(i32, i32)>()
            })
            .collect();
        Line {
            start: Point::new(points[0]),
            end: Point::new(points[1]),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn get_contained_points(&self) -> Vec<Point> {
        let mut points = Vec::new();        
        if self.is_horizontal() {
            let start = if self.start.x > self.end.x { self.end.x } else { self.start.x };
            let end = if self.start.x > self.end.x { self.start.x } else { self.end.x };
            for x in start..=end {
                points.push(Point::new((x, self.start.y)));
            }
        } else if self.is_vertical() {
            let start = if self.start.y > self.end.y { self.end.y } else { self.start.y };
            let end = if self.start.y > self.end.y { self.start.y } else { self.end.y };
            for y in start..=end {
                points.push(Point::new((self.start.x, y)));
            }
        } else {
            for i in 0..=(self.start.x - self.end.x).abs() {
                let x = if self.start.x > self.end.x {self.start.x - i} else { self.start.x + i };
                let y = if self.start.y > self.end.y {self.start.y - i} else { self.start.y + i };
                points.push(Point::new((x, y)));
            }            
        }
        points
    }
}

#[test]
fn test_create_line_from_string() {
    assert_eq!(
        Line::from_string("629,581 -> 123,75"),
        Line::new(Point::new((629, 581)), Point::new((123, 75)))
    );
}

#[test]
fn test_is_horizontal() {
    let l = Line::from_string("629,581 -> 123,581");
    assert_eq!(l.is_horizontal(), true);
}

#[test]
fn test_is_vertical() {
    let l = Line::from_string("629,581 -> 629,420");
    assert_eq!(l.is_vertical(), true);
}