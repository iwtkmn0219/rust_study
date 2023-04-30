use ::rand::Rng;

const MAP_SIZE: usize = 25;

fn main() {
    // Random number creator
    let mut rng = rand::thread_rng();
    // Init maze
    let mut maze = [[0; MAP_SIZE]; MAP_SIZE];
    for n in 0..MAP_SIZE {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[MAP_SIZE - 1][n] = 1;
        maze[n][MAP_SIZE - 1] = 1;
    }
    // Create another wall
    for y in 2..MAP_SIZE - 2 {
        for x in 2..MAP_SIZE - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            // Create pillar
            maze[y][x] = 1;
            // Create random position
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y - 1][x] = 1,
                1 => maze[y + 1][x] = 1,
                2 => maze[y][x - 1] = 1,
                3 => maze[y][x + 1] = 1,
                _ => {}
            }
        }
    }
    // Show maze
    let tiles = ["⬜", "⬛"];
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            print!("{}", tiles[maze[y][x]]);
        }
        println!();
    }
}
