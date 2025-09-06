
// =========================
//  Conway's Game of Life
// ========================= 

use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};

const GRID_ROWS: usize = 40;
const GRID_COLS: usize = 100;

// An enum that describes the possible statuses of a cell
#[derive(Copy, Clone, PartialEq)]
enum Status{
    ALIVE,
    DEAD
}

// A couple of coordinates
#[derive(Copy, Clone)]
struct Coords{
    x: usize,
    y: usize
}

// A struct that represents a grid cell
struct Cell{
    coords: Coords,
    status: Status
}

// Clear the console
fn clear_console() {
    print!("\x1B[2J\x1B[H"); 
    stdout().flush().ok();
}

// Print a grid on the screen
fn print_grid(grid: &[[Status; GRID_COLS]; GRID_ROWS]){
    clear_console();
    let mut content: String = String::from("");

    for y in 0..GRID_ROWS {
        for x in 0..GRID_COLS {
            match grid[y][x]{
                Status::DEAD => content.push_str("⋅"),
                Status::ALIVE => content.push_str("#")
            }
        }

        content.push_str("\n");
    }

    print!("{}", content);
}

// Coordinates toroidal correction
// Handle negative and overflowed coordinates
fn get_unbound_coords(x: isize, y: isize) -> Coords{    
    let cols = GRID_COLS as isize;
    let rows = GRID_ROWS as isize;

    let corrected_x = ((x % cols) + cols) % cols;
    let corrected_y = ((y % rows) + rows) % rows;

    Coords { x: corrected_x as usize, y: corrected_y as usize}
}

// Set the value of a cell
fn set_cell(
    grid: &mut [[Status; GRID_COLS]; GRID_ROWS], 
    cell: Cell){

    grid[cell.coords.y][cell.coords.x] = cell.status;
}

// Get the value of a cell by coordinates
fn get_cell(
    grid: &[[Status; GRID_COLS]; GRID_ROWS], 
    coords: Coords) -> Status{

    grid[coords.y][coords.x] 
}

// Count alive neighbors
fn get_alive_neighbour(
    grid: &[[Status; GRID_COLS]; GRID_ROWS], 
    coords: Coords) -> i8{

    let mut counter: i8 = 0;
    
    let cx = coords.x as isize;
    let cy = coords.y as isize;

    for y in cy - 1 ..= cy + 1 {
        for x in cx - 1 ..= cx +1 {
            if y == cy && x == cx { continue; }
            
            let neighbour_coords = get_unbound_coords(x, y);
            if  get_cell(grid, neighbour_coords)  == Status::ALIVE { counter += 1; }
        }
    }

    counter
}

// GRID UPDATER
// Update the grid applying Conway's rules
//
// - Survival: A live cell with 2 or 3 neighbors stays alive.
// - Death: A live cell with fewer than 2 neighbors dies (underpopulation).
// - Death: A live cell with more than 3 neighbors dies (overpopulation).
// - Birth: A dead cell with exactly 3 neighbors becomes alive.
fn update_grid(
    grid1: &[[Status; GRID_COLS]; GRID_ROWS], 
    grid2: &mut [[Status; GRID_COLS]; GRID_ROWS]){

    for y in 0 .. GRID_ROWS {
        for x in 0 .. GRID_COLS {
            let alive_neighbours: i8 = get_alive_neighbour(grid1, Coords { y: y, x: x });
            let mut new_status = Status::DEAD;

            if get_cell(grid1, Coords{ y: y,  x: x })  == Status::ALIVE {
                if alive_neighbours == 3 || alive_neighbours == 2 {
                    new_status = Status::ALIVE;
                }
            } else {
                if alive_neighbours == 3 {
                    new_status = Status::ALIVE;
                }
            }

            set_cell(grid2, Cell { coords: Coords { y: y, x: x}, status: new_status});
        }
    }
}


// Helper to draw a pattern using offsets
fn draw_pattern(
    grid: &mut [[Status; GRID_COLS]; GRID_ROWS], 
    x: isize,
    y: isize, 
    offsets: &[(isize, isize)]) {

    for (dx, dy) in offsets {
        let c = get_unbound_coords(x + *dx, y + *dy);
        set_cell(grid, Cell { coords: c, status: Status::ALIVE });
    }
}



// SHAPE FUNCTIONS
// Helpers to draw standard shapes starting from (x,y) position
//
// - Glider
// - Block
// - Blinker
// - Toad
// - Beacon
// - Beehive
// - LWSS
// - Pulsar
// - Pentadecathlon
fn place_glider(
    grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {

    const OFFSETS: &[(isize,isize)] = &[
        (1,0),(2,1),(0,2),(1,2),(2,2)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_block(
    grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {

    const OFFSETS: &[(isize,isize)] = &[
        (0,0),(1,0),(0,1),(1,1)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_blinker(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (0,0),(1,0),(2,0)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_toad(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (1,0),(2,0),(3,0),
        (0,1),(1,1),(2,1)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_beacon(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (0,0),(1,0),(0,1),(1,1),
        (2,2),(3,2),(2,3),(3,3)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_beehive(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (1,0),(2,0),
        (0,1),(3,1),
        (1,2),(2,2)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_lwss(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (0,0),(3,0),
        (4,1),
        (0,2),(4,2),
        (1,3),(2,3),(3,3),(4,3)
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_pulsar(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (2,0),(3,0),(4,0),(8,0),(9,0),(10,0),
        (0,2),(5,2),(7,2),(12,2),
        (0,3),(5,3),(7,3),(12,3),
        (0,4),(5,4),(7,4),(12,4),
        (2,5),(3,5),(4,5),(8,5),(9,5),(10,5),
        (2,7),(3,7),(4,7),(8,7),(9,7),(10,7),
        (0,8),(5,8),(7,8),(12,8),
        (0,9),(5,9),(7,9),(12,9),
        (0,10),(5,10),(7,10),(12,10),
        (2,12),(3,12),(4,12),(8,12),(9,12),(10,12),
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

fn place_pentadecathlon(grid: &mut [[Status; GRID_COLS]; GRID_ROWS], x: isize, y: isize) {
    const OFFSETS: &[(isize,isize)] = &[
        (0,1),(1,1),(2,1),(3,1),
        (4,0),(4,2),
        (5,1),(6,1),(7,1),(8,1),
        (9,1),(10,1),
    ];
    draw_pattern(grid, x, y, OFFSETS);
}

// ------------------------------------------

fn main() {
    
    let mut grid1: [[Status; GRID_COLS]; GRID_ROWS] = [[Status::DEAD; GRID_COLS]; GRID_ROWS];
    let mut grid2: [[Status; GRID_COLS]; GRID_ROWS] = [[Status::DEAD; GRID_COLS]; GRID_ROWS];

    let duration = Duration::from_millis(200);

    // Initial shapes
    place_glider(&mut grid1, 10, 8);
    place_block(&mut grid1, 3, 3);
    place_blinker(&mut grid1, 60, 5);
    place_toad(&mut grid1, 30, 12);
    place_beacon(&mut grid1, 80, 25);
    place_beehive(&mut grid1, 12, 15);
    place_lwss(&mut grid1, 6, 30);
    place_pulsar(&mut grid1, 60, 10);
    place_pentadecathlon(&mut grid1, 20, 35);

    // Main loop
    loop {
        print_grid(&grid1);
        thread::sleep(duration);
        update_grid(& grid1, &mut grid2); 
        std::mem::swap(&mut grid1, &mut grid2);        
    } 
    
}
