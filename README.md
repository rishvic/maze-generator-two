# maze-generator-two

A WebAssembly library written in Rust, which generates mazes, and also generates
vertex buffers and indices to be used in a WebGL context to render out as basic
maze.

## Build Instructions

To build the WebAssembly module, you need
[Rust](https://www.rust-lang.org/tools/install) and
[`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/).

Once you have those installed, to build the module, run
```sh
wasm-pack build
```

To build without the `console_error_panic_hook` feature (adds better console
logging on panic, but increases build size quite a lot), remove the default
features,
```sh
wasm-pack build -- --no-default-features
```

To build with `wee_alloc` feature (decreases build size a bit, but a bit slower
memory allocation),
```sh
wasm-pack build -- --features wee_alloc
```


You can also combine both the options as you like.

## Usage

Once built, you'll be left with a WebAssembly module in `./pkg`. To include the
module in you npm or Yarn, add it to the `dependencies` section as such:
```json
{
  "dependencies": {
    "maze-generator-two": "portal:<path/to/pkg>"
  }
}
```

## API

* [`MazeAlgo`](#mazealgo)
  - [`MazeAlgo.Dfs`](#mazealgodfs)
  - [`MazeAlgo.Kruskal`](#mazealgokruskal)
* [`SquareMaze`](#squaremaze)
  - `SquareMaze.from_seed(width: usize, height: usize, algo: MazeAlgo, seed_p1: u32, seed_p2: u32) -> SquareMaze`
  - `SquareMaze.width(&self) -> usize`
  - `SquareMaze.height(&self) -> usize`
  - `SquareMaze.seed_p1(&self) -> u32`
  - `SquareMaze.seed_p2(&self) -> u32`
* [`SqMzBuffer`](#sqmzbuffer)
  - `SqMzBuffer.new(mz: SquareMaze, r: f32) -> SqMzBuffer`
  - `SqMzBuffer.get_vertices(&self) -> *const f32, SqMzBuffer.get_vertices_count(&self) -> usize`
  - `SqMzBuffer.get_indices(&self) -> *const u32, SqMzBuffer.get_indices_count(&self) -> usize`

### `MazeAlgo`

An enum, representing the available maze algorithms to generate a maze.

#### `MazeAlgo.Dfs`

Enum variant, which generates a maze using
[randomised depth-first search](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_depth-first_search).

#### `MazeAlgo.Kruskal`

Enum variant, which generates a maze using
[randomised Kruskal's algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal's_algorithm).

### `SquareMaze`

Class representing a rectangular maze with square cells, each cell having 4
neighbours.

#### `SquareMaze.from_seed(width: usize, height: usize, algo: MazeAlgo, seed_p1: u32, seed_p2: u32) -> SquareMaze`

Generates a new `SquareMaze` of the given width, height, using the specified
maze generation algorithm, and with the provided seed (needs a 64-bit seed, so
we need to provide 2 32-bit ints).

#### `SquareMaze.width(&self) -> usize`

Returns the width of a generated maze.

#### `SquareMaze.height(&self) -> usize`

Returns the height of a generated maze.

#### `SquareMaze.seed_p1(&self) -> u32`

Returns the provided `seed_p1` of a maze during its generation.

#### `SquareMaze.seed_p2(&self) -> u32`

Returns the provided `seed_p2` of a maze during its generation.

### `SqMzBuffer`

Class which contains methods to get the required array buffers and element
indices to render the maze in a WebGL context.

#### `SqMzBuffer.new(mz: SquareMaze, r: f32) -> SqMzBuffer`

Generates the buffers for the given Square Maze and the given ratio of wall
thickness to path thickness.

#### `SqMzBuffer.get_vertices(&self) -> *const f32, SqMzBuffer.get_vertices_count(&self) -> usize`

Return a pointer to the vertex array buffer and its size respectively.

#### `SqMzBuffer.get_indices(&self) -> *const u32, SqMzBuffer.get_indices_count(&self) -> usize`

Return a pointer to the element indices array buffer and its size respectively.
