use itertools::Itertools;
mod helper;
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}
struct Dig {
    direction: Direction,
    distance: i64,
}
fn main() {
    let input = helper::read_input();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
fn part_two(input: &Vec<(char, i64, String)>) -> i64 {
    let digs = input
        .iter()
        .map(|(_, _, color)| {
            let color_numbers = color.replace(['(', ')', '#'], "");
            let distance = &color_numbers[..5];
            let distance = i64::from_str_radix(distance, 16).unwrap();
            let direction = &color_numbers.chars().nth(5).unwrap();
            match direction {
                '0' => Dig{direction:Direction::Right, distance},
                '1' => Dig{direction:Direction::Down, distance},
                '2' => Dig{direction:Direction::Left, distance},
                '3' => Dig{direction:Direction::Up, distance},
                _ => panic!("Invalid direction"),
            }
        })
        .collect::<Vec<_>>();
    get_area(&digs)
}
fn part_one(input: &Vec<(char, i64, String)>) -> i64 {
    let digs = input
        .iter()
        .map(|(direction, distance, _)| match direction {
            'R' => Dig {
                direction: Direction::Right,
                distance: *distance,
            },
            'L' => Dig {
                direction: Direction::Left,
                distance: *distance,
            },
            'U' => Dig {
                direction: Direction::Up,
                distance: *distance,
            },
            'D' => Dig {
                direction: Direction::Down,
                distance: *distance,
            },
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<_>>();
    get_area(&digs)
}

fn get_area(digs: &Vec<Dig>) -> i64 {
    let mut current_point = Point { x: 0, y: 0 };
    let mut edges: Vec<Point> = vec![current_point];
    let mut total = 0;
    digs.iter().for_each(|dig| {
        match dig.direction {
            Direction::Up => current_point.y -= dig.distance,
            Direction::Down => current_point.y += dig.distance,
            Direction::Left => current_point.x -= dig.distance,
            Direction::Right => current_point.x += dig.distance,
        }
        edges.push(current_point);
        total += dig.distance;
    });
    let area = edges
        .iter()
        .tuple_windows()
        .fold(0, |acc, (point1, point2)| {
            acc + (point1.x * point2.y) - (point1.y * point2.x)
        });
    (total + area) / 2 + 1
}

// 1- start from initial point (0,0)
// 2- for every direction, move in that direction while updating the the current point so that it can be used for the next direction.
