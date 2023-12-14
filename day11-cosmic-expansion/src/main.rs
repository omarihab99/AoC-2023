mod helper;

fn main() {
    let input = helper::read_input();
    let mut indices : Vec<(usize, usize)> = Vec::new();
    let mut sum  = 0;
    input.iter().enumerate().for_each(|(i, v)| {
        v.iter().enumerate().for_each(|(j, c)| {
            if *c == '#' {
                indices.push((i, j));
            }
        })
    });
    for (i, v )in indices.iter().enumerate() {
        for j in i..indices.len() {
            let (x, y) = indices[j];
            sum += (x as i32 - v.0 as i32).abs() + (y as i32 - v.1 as i32).abs() ;
        }
    }
    println!("Part one: {}", sum);
}
