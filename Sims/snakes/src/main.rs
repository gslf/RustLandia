use std::thread;
use std::time::Duration;

use stdt::utils::random;
use stdt::utils::clear_cli;

#[derive(Clone, Copy)]
struct Coords {
    x: usize,
    y: usize,
}

struct Snake {
    sym: char,
    body: Vec<Coords>,
}

impl Snake {
    fn new(canvas_w: usize, canvas_h: usize, sym: char) -> Self {
        let max_x = (canvas_w - 1) as i128;
        let max_y = (canvas_h - 1) as i128;

        let x = random::integer_in(0, max_x) as usize;
        let y = random::integer_in(0, max_y) as usize;

        let c = Coords { x, y };

        Snake {
            sym: sym,
            body: vec![c],
        }
    }

    fn step(&mut self, grid: &Vec<Vec<char>>) {
        if self.body.is_empty() {
            return;
        }

        let head = self.body[0];

        let head_x = head.x as isize;
        let head_y = head.y as isize;

        let directions: [(isize, isize); 4] = [
            (0, -1), 
            (0,  1), 
            (-1, 0), 
            (1,  0), 
        ];

        let mut valid_moves = Vec::new();

        for (dx, dy) in directions {
            let nx = head_x + dx;
            let ny = head_y + dy;

            // Check boundaries
            if nx >= 0
                && ny >= 0
                && (ny as usize) < grid.len()
                && (nx as usize) < grid[0].len()
            {
                if grid[ny as usize][nx as usize] == ' ' {
                    valid_moves.push(Coords {
                        x: nx as usize,
                        y: ny as usize,
                    });
                }
            }
        }

        if valid_moves.is_empty() {
            self.body.clear();
            return;
        }

        let max_idx = (valid_moves.len() - 1) as i128;
        let idx = random::integer_in(0, max_idx) as usize;
        let new_head = valid_moves[idx];

        self.body.insert(0, new_head);
    }
}

fn generate_grid(width: usize, height: usize, snakes: &Vec<Snake>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];

    for snake in snakes {
        for c in &snake.body {
            if c.x < width && c.y < height {
                grid[c.y][c.x] = snake.sym;
            }
        }
    }

    grid
}

fn display(grid: &Vec<Vec<char>>) {
    clear_cli::clear().unwrap();
    for row in grid {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

fn main() {
    const SNAKES_NUMBER: usize = 40;
    const WIDTH: usize = 80;
    const HEIGHT: usize = 30;
    const BODIES: &[char] = &['#', '@', '$', '%', '&'];

    let mut snakes: Vec<Snake> = Vec::new();
    let mut last_grid: Vec<Vec<char>> = vec![vec![' '; WIDTH]; HEIGHT];

    // Init snakes
    for i in 0..=SNAKES_NUMBER {
        let sym: char = BODIES[i % BODIES.len()];
        snakes.push(Snake::new(WIDTH, HEIGHT, sym));
    }

    // Main loop
    loop {
        if snakes.is_empty() {
            break;
        }

        for snake in &mut snakes {
            snake.step(&last_grid);
        }
        snakes.retain(|s| !s.body.is_empty());

        last_grid = generate_grid(WIDTH, HEIGHT, &snakes);
        display(&last_grid);
        thread::sleep(Duration::from_millis(100));
    }
}

