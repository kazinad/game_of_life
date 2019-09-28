# Rust newbie learning project: game_of_life
This is a console based implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) cellular automation algorithm. This project intends to demonstrate most basic features of the Rust language.

## Known issues
- The current implementation uses console screen to display the evolving cell grid, and one cell represented by a single character on the console screen. On most systems the default console size is 80x40 characters, which is too small for the implemented cell automation algorithm. For this reason, please increase your console size to say 383x114 or bigger for a much enjoyable result.
## Features
- Each cell in a cell grid is represented by a single bit. The bits are grouped into Rust's [usize](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/mod.rs#L12) scalar data type, so on a 64 bit system a single cell group stores [64 cells](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/indexer.rs#L5) in a single usize value. All cells of a cell grid are stored in an array of cell groups [cells: Vec\<usize>](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/cellgrid.rs#L7). This dense representation of cell grid cells is eight times smaller than the cell <-> byte representation, i.e. scales better on larger cell grid sizes.
- A cell has three properties in a cell grid: x and y positions and the alive/dead value. The alive/dead value is represented as a single bit in the cells vector, but its x and y coordinates are computed by the position of the bit within the bit array. This is done by the Indexer [index](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/indexer.rs#L55) and [pos](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/indexer.rs#L70) methods.
- Computing cell grid generations is a [pure function](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules), which means, we need the n<sup>th</sup> generation of the cell grid to compute the (n+1)<sup>th</sup> generation. This is represented in [Universe current and next CellGrid values](https://github.com/kazinad/game_of_life/blob/master/src/universe.rs#L4-L5). Simply [swapping the two values](https://github.com/kazinad/game_of_life/blob/master/src/universe.rs#L51) avoids frequent large memory reallocations.
- Computing next generation of the cell grid uses all the [cpu cores available on the running system](https://github.com/kazinad/game_of_life/blob/master/src/universe.rs#L23). Rust does not allow multiple threads to write the same memory, i.e. it denies to have different threads holding mutable references to the same cells vector. For this reason we have to [create slices](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/cellgrid_slice.rs#L4) of the cells vector. The cell vector slices are represented by the [CellVectorSlice](https://github.com/kazinad/game_of_life/blob/master/src/cellgrid/cellgrid_slice.rs#L35) which maps distinct regions of the original cells vector to [different threads](https://github.com/kazinad/game_of_life/blob/master/src/universe.rs#L30-L42). There is no need to use additional thread synchronization methods, so those do not block each other and can run on full speed.
- Displaying the current generation is also [parallel](https://github.com/kazinad/game_of_life/blob/master/src/universe.rs#L25-L28) to computing the next generation. This possible because Rust allows to have multiple immutable references to the same value.
- Computing cell grid generations and displaying the current one are implemented in separate modules, so the current [console display method](https://github.com/kazinad/game_of_life/blob/master/src/screen.rs#L8) is easy to replace.
- The original algorithm settles to a relatively static combination of cells after a while. To avoid this, cells are added at random positions if the number of living cells gets stable with 5 generations.
