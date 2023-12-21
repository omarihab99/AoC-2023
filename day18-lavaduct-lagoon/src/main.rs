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
    x: i32,
    y: i32,
}
struct Dig {
    direction: Direction,
    distance: i32,
}
fn main() {
    let input = helper::read_input();
    let digs = input
        .iter()
        .map(|(direction, distance)| match direction {
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
    println!("Part one: {}", (total + area) / 2 + 1);
}

// 1- start from initial point (0,0)
// 2- for every direction, move in that direction while updating the the current point so that it can be used for the next direction.
