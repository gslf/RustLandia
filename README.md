# RustLandia
Experimenting in Rust


## CONTENTS

### Conway's Game of Life
Conway’s Game of Life (Toroidal Grid)

**Classic rules:**
- A live cell with 2 or 3 live neighbors survives.
- A dead cell with exactly 3 live neighbors becomes alive.
- Otherwise, the cell dies or stays dead.

**Toroidal topology:**
The world “wraps” around itself. Neighbors beyond the right edge are on the left edge (and vice versa), and the same for top/bottom. This eliminates hard borders and yields more uniform behavior.
