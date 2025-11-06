use std::collections::VecDeque;
use std::io::{self, Write};
use rand::Rng;
use rand::rng;

/// Coords: 
/// x = column, 
/// y = row
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coords {
    x: usize,
    y: usize,
}

type Map = Vec<Vec<char>>;

fn clear_console() {
    print!("\x1B[2J\x1B[H");
    let _ = io::stdout().flush();
}

fn print_map(map: &Map) {
    clear_console();
    let mut out = String::new();
    for row in map {
        for &ch in row {
            out.push(ch);
        }
        out.push('\n');
    }
    let _ = io::stdout().lock().write_all(out.as_bytes());
}

fn create_filled(height: usize, width: usize) -> Map {
    vec![vec!['#'; width]; height]
}

/// Returns true if the given coordinates are within the map bounds.
fn in_bounds(map: &Map, p: Coords) -> bool {
    p.y < map.len() && p.x < map[0].len()
}

/// Attempts to apply an offset (dx, dy) to a coordinate.
/// Returns Some(new) only if the result stays within the map bounds.
fn try_offset(map: &Map, p: Coords, dx: isize, dy: isize) -> Option<Coords> {
    let nx = p.x as isize + dx;
    let ny = p.y as isize + dy;
    if nx < 0 || ny < 0 {
        return None;
    }
    let (nxu, nyu) = (nx as usize, ny as usize);
    if nyu < map.len() && nxu < map[0].len() {
        Some(Coords { x: nxu, y: nyu })
    } else {
        None
    }
}

/// Opens a cell (sets ' ') at the given coordinates.
fn open(map: &mut Map, p: Coords) {
    map[p.y][p.x] = ' ';
}

/// Returns true if the cell at the given coordinates is a wall ('#').
fn is_wall(map: &Map, p: Coords) -> bool {
    map[p.y][p.x] == '#'
}

/// Returns true if the cell at the given coordinates is walkable.
fn is_open(map: &Map, p: Coords) -> bool {
    map[p.y][p.x] == ' ' || map[p.y][p.x] == '.'
}

/// Generates a perfect maze starting from `start`
fn carve_maze(map: &mut Map, start: Coords) {
    let mut stack = vec![start];
    open(map, start);

    // Directions spaced by 2 cells to maintain walls between corridors
    let dirs: [(isize, isize); 4] = [(-2, 0), (0, 2), (2, 0), (0, -2)];

    let mut rng = rng();

    loop {
        let cur = match stack.pop() {
            Some(c) => c,
            None => break,        
        };

        let mut candidates = Vec::with_capacity(4);
        for (dx2, dy2) in dirs {
            if let Some(nxt) = try_offset(map, cur, dx2, dy2) {
                let mid = Coords {
                    x: (cur.x + nxt.x) / 2,
                    y: (cur.y + nxt.y) / 2,
                };
                if is_wall(map, mid) && is_wall(map, nxt) {
                    candidates.push((mid, nxt));
                }
            }
        }

        if candidates.is_empty() {
            continue;
        }

        let idx = rng.gen_range(0..candidates.len());
        let (mid, nxt) = candidates[idx];
       
        stack.push(cur);
        open(map, mid);
        open(map, nxt);
        stack.push(nxt);
    }
}

/// Solves from point s to g using BFS over open cells.
/// Returns the path including s and g if one exists.
fn solve_bfs(map: &Map, s: Coords, g: Coords) -> Option<Vec<Coords>> {
    if !in_bounds(map, s) || !in_bounds(map, g) {
        return None;
    }
    if !is_open(map, s) || !is_open(map, g) {
        return None;
    }

    let h = map.len();
    let w = map[0].len();
    let mut parent: Vec<Vec<Option<Coords>>> = vec![vec![None; w]; h];
    let mut q = VecDeque::new();

    q.push_back(s);
    parent[s.y][s.x] = Some(s);

    let steps: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(cur) = q.pop_front() {
        if cur == g {
            break;
        }
        for (dx, dy) in steps {
            if let Some(nxt) = try_offset(map, cur, dx, dy) {
                if parent[nxt.y][nxt.x].is_none() && is_open(map, nxt) {
                    parent[nxt.y][nxt.x] = Some(cur);
                    q.push_back(nxt);
                }
            }
        }
    }

    if parent[g.y][g.x].is_none() {
        return None;
    }

    // Reconstruct path
    let mut path = Vec::new();
    let mut cur = g;
    while cur != s {
        path.push(cur);
        cur = parent[cur.y][cur.x].unwrap();
    }
    path.push(s);
    path.reverse();
    Some(path)
}

fn main() {
    // Use ODD dimensions for consistent cell+wall structure
    const WIDTH: usize = 79;  // columns (x)
    const HEIGHT: usize = 25; // rows (y)
    assert!(WIDTH % 2 == 1 && HEIGHT % 2 == 1, "Use odd dimensions");

    let mut map = create_filled(HEIGHT, WIDTH);

    // Start and goal cells must be on ODD coordinates
    let start = Coords { x: 1, y: 1 };
    let goal = Coords {
        x: WIDTH - 2,
        y: HEIGHT - 2,
    };

    // 1) Generate a PERFECT maze
    carve_maze(&mut map, start);

    // 2) Open entrance and exit on opposite borders
    map[start.y][0] = ' ';
    map[goal.y][WIDTH - 1] = ' ';

    // 3) Solve from start to goal
    if let Some(path) = solve_bfs(&map, start, goal) {
        // 4) Draw the path with '·'
        for &p in &path {
            if map[p.y][p.x] == ' ' {
                map[p.y][p.x] = '·';
            }
        }
        // Highlight start and goal
        map[start.y][start.x] = 'S';
        map[goal.y][goal.x] = 'G';
    } else {
        eprintln!("No solution found (should never happen in a perfect maze).");
    }

    print_map(&map);
}
