// ============
// Matrix Rain
// ============
use std::thread;
use std::time::Duration;
use std::io;
use std::io::Write;
use rand::{rng, Rng};


const GRID_ROWS: usize = 40;
const GRID_COLS: usize = 140;
const START_PROB: f64 = 0.03;
const STOP_PROB: f64 = 0.06;
const GLYPHS: &[char] = &[
    'ｱ','ｲ','ｳ','ｴ','ｵ','ｶ','ｷ','ｸ','ｹ','ｺ','ｻ','ｼ','ｽ','ｾ','ｿ',
    'ﾀ','ﾁ','ﾂ','ﾃ','ﾄ','ﾅ','ﾆ','ﾇ','ﾈ','ﾉ','ﾊ','ﾋ','ﾌ','ﾍ','ﾎ',
    'ﾏ','ﾐ','ﾑ','ﾒ','ﾓ','ﾔ','ﾕ','ﾖ','ﾗ','ﾘ','ﾙ','ﾚ','ﾛ','ﾜ','ｦ','ﾝ'
];


// Print the grid using a buffer
fn print_grid(g: &[[char; GRID_COLS]; GRID_ROWS]){
    let mut buf = String::with_capacity(GRID_ROWS * ( GRID_COLS + 1 ) + 8);

    // Clear
    buf.push_str("\x1B[H");

    for (y, row) in g.iter().enumerate() {
        for &ch in row {
            buf.push(ch);
        }
        
        if y + 1 < GRID_ROWS {
            buf.push('\n');
        }
    }

    let mut out = io::stdout().lock();
    let _ = out.write_all(buf.as_bytes());
    let _ = out.flush();
}

// Update the grid, adding by adding a row of 
// characters at the top and shifting the 
// rest of the rows down
fn update_grid(g: &mut [[char; GRID_COLS]; GRID_ROWS], r: &[char; GRID_COLS]){
    g.rotate_right(1);
    g[0] = *r;
}

// Create the new line to insert in the grid
fn generate_line(ac: &mut [bool; GRID_COLS]) -> [char; GRID_COLS]{

    let mut rng = rng();
    let mut line = [' '; GRID_COLS];

    for i in 0..GRID_COLS {
        // Active column management
        if ac[i] {
            let idx = rng.random_range(0..GLYPHS.len());
            line[i] = GLYPHS[idx] as char;

            if rng.random_bool(STOP_PROB) {
                ac[i] = false;
            }

        // Inactive column management
        } else {
            if rng.random_bool(START_PROB) {
                ac[i] = true;
                let idx = rng.random_range(0..GLYPHS.len());
                line[i] = GLYPHS[idx] as char;
            } else {
                line[i] = ' ';
            }
        }
    }

    line
}


fn main() {
    let mut grid: [[char; GRID_COLS]; GRID_ROWS] = [[' '; GRID_COLS]; GRID_ROWS];
    let mut active_cols: [bool; GRID_COLS] = [false; GRID_COLS];
    
    let duration = Duration::from_millis(100);
    loop{
        print_grid(&grid);
        let new_line = generate_line(&mut active_cols);
        update_grid(&mut grid, &new_line);        
        thread::sleep(duration);
    }
}


 
