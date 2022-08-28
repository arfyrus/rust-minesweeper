# rust-minesweeper

Minesweeper game created in Rust.

## Features

1. Configurable cell width, bomb percentage, and grid width and height.
2. Last selected cell is shown.

## Configuration

You can change the constants in `config.rs` to fit your liking.

1. W
   Width of the grid. Default: 15.
2. H
   Height of the grid. Default: 15.
3. DISPLAY_SPACE
   How much space to display cells. Can't be lower than 5. Preferably an odd number. Default: 5.
4. BOMB_PERCENT
   Percentage that a cell would be a bomb. Default: 75.
